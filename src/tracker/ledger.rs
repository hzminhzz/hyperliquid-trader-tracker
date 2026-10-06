//! Durable Observation Ledger & Publication Engine (Contracts C2–C5).
//!
//! Provides the transactional persistence foundation for KonScanner:
//! - C2: Durable receipt ledger, economic deduplication index, and atomic checkpoint/outbox transaction.
//! - C3: Canonical versioned event family and envelopes.
//! - C4: Consistent snapshot pages and bounded ordered tail reader with cursor lifecycle checks.
//! - C5: Monotonic revisions, retention floor enforcement, and backpressure.

use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value;
use tokio_rusqlite::Connection;

use crate::state::Direction;

const LEDGER_SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS ledger_meta (
    ledger_id          TEXT    PRIMARY KEY,
    created_at         TEXT    NOT NULL,
    current_seq        INTEGER NOT NULL DEFAULT 0,
    retention_floor    INTEGER NOT NULL DEFAULT 0
) STRICT;

CREATE TABLE IF NOT EXISTS receipts (
    receipt_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    source             TEXT    NOT NULL,
    source_key         TEXT    NOT NULL,
    payload            TEXT    NOT NULL,
    received_at        TEXT    NOT NULL,
    status             TEXT    NOT NULL DEFAULT 'PENDING',
    UNIQUE(source, source_key)
) STRICT;

CREATE INDEX IF NOT EXISTS idx_receipts_status ON receipts(status);

CREATE TABLE IF NOT EXISTS position_checkpoints (
    address            TEXT    NOT NULL,
    coin               TEXT    NOT NULL,
    szi                TEXT    NOT NULL,
    direction          TEXT    NOT NULL,
    avg_entry          TEXT    NOT NULL,
    opened_at          TEXT    NOT NULL,
    last_added_at      TEXT    NOT NULL,
    realized_pnl       TEXT    NOT NULL,
    revision           INTEGER NOT NULL,
    generation         INTEGER NOT NULL,
    coverage           TEXT    NOT NULL,
    updated_at         TEXT    NOT NULL,
    PRIMARY KEY (address, coin)
) STRICT;

CREATE TABLE IF NOT EXISTS outbox_events (
    seq                INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id           TEXT    NOT NULL UNIQUE,
    schema_version     TEXT    NOT NULL DEFAULT '1',
    ledger_id          TEXT    NOT NULL,
    kind               TEXT    NOT NULL,
    environment        TEXT    NOT NULL DEFAULT 'mainnet',
    instrument_id      TEXT    NOT NULL,
    wallet_id          TEXT    NOT NULL,
    event_time         TEXT    NOT NULL,
    received_at        TEXT    NOT NULL,
    known_at           TEXT    NOT NULL,
    cause_receipt_id   INTEGER,
    before_revision    INTEGER NOT NULL,
    after_revision     INTEGER NOT NULL,
    payload            TEXT    NOT NULL,
    quality            TEXT    NOT NULL DEFAULT 'valid'
) STRICT;

CREATE INDEX IF NOT EXISTS idx_outbox_seq ON outbox_events(seq);
CREATE INDEX IF NOT EXISTS idx_outbox_wallet ON outbox_events(wallet_id);
";

/// Opaque cursor identifying position in an ordered observation ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerCursor {
    pub ledger_id: String,
    pub seq: u64,
}

/// Canonical envelope for published observation events.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalEvent {
    pub schema_version: String,
    pub event_id: String,
    pub cursor: LedgerCursor,
    pub kind: String,
    pub environment: String,
    pub instrument_id: String,
    pub wallet_id: String,
    pub event_time: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub known_at: DateTime<Utc>,
    pub cause_ids: Vec<String>,
    pub before_revision: u64,
    pub after_revision: u64,
    pub payload: Value,
    pub quality: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionParams {
    pub receipt_id: i64,
    pub wallet_id: String,
    pub coin: String,
    pub checkpoint: Option<PositionCheckpoint>,
    pub event_kind: String,
    pub instrument_id: String,
    pub event_time: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
    pub before_rev: u64,
    pub after_rev: u64,
    pub payload: Value,
}

/// Persisted checkpoint row for an open position.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionCheckpoint {
    pub address: String,
    pub coin: String,
    pub szi: Decimal,
    pub direction: Direction,
    pub avg_entry: Decimal,
    pub opened_at: DateTime<Utc>,
    pub last_added_at: DateTime<Utc>,
    pub realized_pnl: Decimal,
    pub revision: u64,
    pub generation: u64,
    pub coverage: String,
    pub updated_at: DateTime<Utc>,
}

/// Paginated snapshot page from a committed ledger cut.
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotPage {
    pub ledger_id: String,
    pub snapshot_id: String,
    pub last_seq: u64,
    pub page: usize,
    pub total_pages: usize,
    pub positions: Vec<PositionCheckpoint>,
}

/// Result of querying the bounded ordered tail.
#[derive(Debug, Clone, PartialEq)]
pub struct TailResult {
    pub ledger_id: String,
    pub events: Vec<CanonicalEvent>,
    pub next_cursor: LedgerCursor,
    pub high_water_mark: u64,
    pub retention_floor: u64,
}

/// Errors occurring during tail reading or cursor validation.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TailError {
    #[error("cursor expired: requested seq {requested} is below retention floor {floor}")]
    CursorExpired { requested: u64, floor: u64 },
    #[error("ledger changed: requested {requested}, current {current}")]
    LedgerChanged { requested: String, current: String },
    #[error("database error: {0}")]
    Database(String),
}

/// Durable SQLite-backed observation ledger.
pub struct ObservationLedger {
    path: PathBuf,
    conn: Option<Connection>,
    ledger_id: String,
}

impl ObservationLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            conn: None,
            ledger_id: String::new(),
        }
    }

    /// Open or create the database, apply WAL pragmas, run migrations, and load ledger metadata.
    pub async fn connect(&mut self) -> Result<(), tokio_rusqlite::Error> {
        let conn = Connection::open(&self.path).await?;
        let ledger_id = conn
            .call(|conn| {
                conn.pragma_update(None, "journal_mode", "WAL")?;
                conn.pragma_update(None, "busy_timeout", 5000)?;
                conn.pragma_update(None, "synchronous", "NORMAL")?;
                conn.execute_batch(LEDGER_SCHEMA)?;

                // Load or initialize ledger metadata
                let mut stmt = conn.prepare("SELECT ledger_id FROM ledger_meta LIMIT 1")?;
                let mut rows = stmt.query([])?;
                if let Some(row) = rows.next()? {
                    let id: String = row.get(0)?;
                    Ok(id)
                } else {
                    let new_id = format!("ledger-{:016x}", rand::random::<u64>());
                    let now = Utc::now().to_rfc3339();
                    conn.execute(
                        "INSERT INTO ledger_meta (ledger_id, created_at, current_seq, retention_floor) VALUES (?1, ?2, 0, 0)",
                        rusqlite::params![new_id, now],
                    )?;
                    Ok(new_id)
                }
            })
            .await?;

        self.conn = Some(conn);
        self.ledger_id = ledger_id;
        Ok(())
    }

    pub fn ledger_id(&self) -> &str {
        &self.ledger_id
    }

    fn conn(&self) -> &Connection {
        self.conn
            .as_ref()
            .expect("ObservationLedger used before connect()")
    }

    /// Durably append a raw source receipt before interpretation (C2 Step 1).
    pub async fn append_receipt(
        &self,
        source: &str,
        source_key: &str,
        payload: &Value,
        received_at: DateTime<Utc>,
    ) -> Result<(i64, bool), tokio_rusqlite::Error> {
        let source = source.to_string();
        let source_key = source_key.to_string();
        let payload_str = payload.to_string();
        let received_str = received_at.to_rfc3339();

        self.conn()
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT receipt_id FROM receipts WHERE source = ?1 AND source_key = ?2",
                )?;
                let mut rows = stmt.query(rusqlite::params![source, source_key])?;
                if let Some(row) = rows.next()? {
                    let id: i64 = row.get(0)?;
                    return Ok((id, true));
                }

                conn.execute(
                    "INSERT INTO receipts (source, source_key, payload, received_at, status) VALUES (?1, ?2, ?3, ?4, 'PENDING')",
                    rusqlite::params![source, source_key, payload_str, received_str],
                )?;
                let id = conn.last_insert_rowid();
                Ok((id, false))
            })
            .await
    }

    /// Commit an observation transition atomically (C2 Step 2).
    /// Commit an observation transition atomically (C2 Step 2).
    pub async fn commit_transition(
        &self,
        params: TransitionParams,
    ) -> Result<CanonicalEvent, tokio_rusqlite::Error> {
        let ledger_id = self.ledger_id.clone();
        let wallet = params.wallet_id;
        let coin = params.coin;
        let event_kind = params.event_kind;
        let instrument_id = params.instrument_id;
        let payload_str = params.payload.to_string();
        let event_time_str = params.event_time.to_rfc3339();
        let received_at_str = params.received_at.to_rfc3339();
        let known_at = Utc::now();
        let known_at_str = known_at.to_rfc3339();
        let checkpoint = params.checkpoint;
        let receipt_id = params.receipt_id;
        let before_rev = params.before_rev;
        let after_rev = params.after_rev;
        let payload = params.payload;
        let event_time = params.event_time;
        let received_at = params.received_at;

        self.conn()
            .call(move |conn| {
                let tx = conn.transaction()?;

                // 1. Mark receipt committed
                tx.execute(
                    "UPDATE receipts SET status = 'COMMITTED' WHERE receipt_id = ?1",
                    rusqlite::params![receipt_id],
                )?;

                // 2. Position checkpoint
                match &checkpoint {
                    Some(cp) => {
                        let dir_str = match cp.direction {
                            Direction::Long => "Long",
                            Direction::Short => "Short",
                        };
                        tx.execute(
                            "INSERT INTO position_checkpoints (
                                address, coin, szi, direction, avg_entry, opened_at,
                                last_added_at, realized_pnl, revision, generation, coverage, updated_at
                            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
                            ON CONFLICT(address, coin) DO UPDATE SET
                                szi = excluded.szi,
                                direction = excluded.direction,
                                avg_entry = excluded.avg_entry,
                                opened_at = excluded.opened_at,
                                last_added_at = excluded.last_added_at,
                                realized_pnl = excluded.realized_pnl,
                                revision = excluded.revision,
                                generation = excluded.generation,
                                coverage = excluded.coverage,
                                updated_at = excluded.updated_at",
                            rusqlite::params![
                                cp.address,
                                cp.coin,
                                cp.szi.to_string(),
                                dir_str,
                                cp.avg_entry.to_string(),
                                cp.opened_at.to_rfc3339(),
                                cp.last_added_at.to_rfc3339(),
                                cp.realized_pnl.to_string(),
                                cp.revision as i64,
                                cp.generation as i64,
                                cp.coverage,
                                cp.updated_at.to_rfc3339(),
                            ],
                        )?;
                    }
                    None => {
                        tx.execute(
                            "DELETE FROM position_checkpoints WHERE address = ?1 AND coin = ?2",
                            rusqlite::params![wallet, coin],
                        )?;
                    }
                }

                // 3. Advance current_seq
                tx.execute(
                    "UPDATE ledger_meta SET current_seq = current_seq + 1 WHERE ledger_id = ?1",
                    rusqlite::params![ledger_id],
                )?;
                let seq: u64 = tx.query_row(
                    "SELECT current_seq FROM ledger_meta WHERE ledger_id = ?1",
                    rusqlite::params![ledger_id],
                    |r| r.get(0),
                )?;

                let event_id = format!("{ledger_id}:{seq}");

                // 4. Outbox event append
                tx.execute(
                    "INSERT INTO outbox_events (
                        seq, event_id, schema_version, ledger_id, kind, environment,
                        instrument_id, wallet_id, event_time, received_at, known_at,
                        cause_receipt_id, before_revision, after_revision, payload, quality
                    ) VALUES (?1, ?2, '1', ?3, ?4, 'mainnet', ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, 'valid')",
                    rusqlite::params![
                        seq as i64,
                        event_id,
                        ledger_id,
                        event_kind,
                        instrument_id,
                        wallet,
                        event_time_str,
                        received_at_str,
                        known_at_str,
                        receipt_id,
                        before_rev as i64,
                        after_rev as i64,
                        payload_str,
                    ],
                )?;

                tx.commit()?;

                Ok(CanonicalEvent {
                    schema_version: "1".to_string(),
                    event_id,
                    cursor: LedgerCursor {
                        ledger_id,
                        seq,
                    },
                    kind: event_kind,
                    environment: "mainnet".to_string(),
                    instrument_id,
                    wallet_id: wallet,
                    event_time,
                    received_at,
                    known_at,
                    cause_ids: vec![receipt_id.to_string()],
                    before_revision: before_rev,
                    after_revision: after_rev,
                    payload,
                    quality: "valid".to_string(),
                })
            })
            .await
    }

    /// Read a consistent snapshot page of committed positions (C4).
    pub async fn snapshot_page(
        &self,
        page: usize,
        page_size: usize,
    ) -> Result<SnapshotPage, tokio_rusqlite::Error> {
        let ledger_id = self.ledger_id.clone();
        let limit = page_size.clamp(1, 100_000);
        let offset = page * limit;
        self.conn()
            .call(move |conn| {
                let last_seq: u64 = conn.query_row(
                    "SELECT current_seq FROM ledger_meta WHERE ledger_id = ?1",
                    rusqlite::params![ledger_id],
                    |r| r.get(0),
                )?;

                let total_rows: usize =
                    conn.query_row("SELECT COUNT(*) FROM position_checkpoints", [], |r| {
                        r.get(0)
                    })?;
                let total_pages = if total_rows == 0 {
                    1
                } else {
                    total_rows.div_ceil(limit)
                };

                let mut stmt = conn.prepare(
                    "SELECT address, coin, szi, direction, avg_entry, opened_at,
                            last_added_at, realized_pnl, revision, generation, coverage, updated_at
                     FROM position_checkpoints
                     ORDER BY address, coin
                     LIMIT ?1 OFFSET ?2",
                )?;

                let mut rows = stmt.query(rusqlite::params![limit as i64, offset as i64])?;
                let mut positions = Vec::new();
                while let Some(row) = rows.next()? {
                    let szi_str: String = row.get(2)?;
                    let dir_str: String = row.get(3)?;
                    let avg_str: String = row.get(4)?;
                    let pnl_str: String = row.get(7)?;
                    let op_str: String = row.get(5)?;
                    let la_str: String = row.get(6)?;
                    let up_str: String = row.get(11)?;

                    positions.push(PositionCheckpoint {
                        address: row.get(0)?,
                        coin: row.get(1)?,
                        szi: Decimal::from_str(&szi_str).unwrap_or(Decimal::ZERO),
                        direction: match dir_str.as_str() {
                            "Short" => Direction::Short,
                            _ => Direction::Long,
                        },
                        avg_entry: Decimal::from_str(&avg_str).unwrap_or(Decimal::ZERO),
                        opened_at: DateTime::parse_from_rfc3339(&op_str)
                            .map(|d| d.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        last_added_at: DateTime::parse_from_rfc3339(&la_str)
                            .map(|d| d.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        realized_pnl: Decimal::from_str(&pnl_str).unwrap_or(Decimal::ZERO),
                        revision: row.get::<_, i64>(8)? as u64,
                        generation: row.get::<_, i64>(9)? as u64,
                        coverage: row.get(10)?,
                        updated_at: DateTime::parse_from_rfc3339(&up_str)
                            .map(|d| d.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                    });
                }

                Ok(SnapshotPage {
                    ledger_id: ledger_id.clone(),
                    snapshot_id: format!("{ledger_id}:snap:{last_seq}"),
                    last_seq,
                    page,
                    total_pages,
                    positions,
                })
            })
            .await
    }

    /// Get current seq and retention floor from ledger meta.
    pub async fn retention_and_seq(&self) -> Result<(u64, u64), tokio_rusqlite::Error> {
        let ledger_id = self.ledger_id.clone();
        self.conn()
            .call(move |conn| {
                let (current_seq, retention_floor): (u64, u64) = conn.query_row(
                    "SELECT current_seq, retention_floor FROM ledger_meta WHERE ledger_id = ?1",
                    rusqlite::params![ledger_id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )?;
                Ok((current_seq, retention_floor))
            })
            .await
    }

    /// Read the bounded ordered tail strictly after `cursor` (C4).
    pub async fn tail(&self, cursor: &LedgerCursor, limit: usize) -> Result<TailResult, TailError> {
        let expected_ledger = self.ledger_id.clone();
        if cursor.ledger_id != expected_ledger {
            return Err(TailError::LedgerChanged {
                requested: cursor.ledger_id.clone(),
                current: expected_ledger,
            });
        }

        let requested_seq = cursor.seq;
        let limit = limit.clamp(1, 1000);

        let (current_seq, retention_floor) = self
            .retention_and_seq()
            .await
            .map_err(|e| TailError::Database(e.to_string()))?;

        if requested_seq < retention_floor {
            return Err(TailError::CursorExpired {
                requested: requested_seq,
                floor: retention_floor,
            });
        }

        self.conn()
            .call(move |conn| -> rusqlite::Result<TailResult> {
                let mut stmt = conn.prepare(
                    "SELECT seq, event_id, schema_version, ledger_id, kind, environment,
                            instrument_id, wallet_id, event_time, received_at, known_at,
                            cause_receipt_id, before_revision, after_revision, payload, quality
                     FROM outbox_events
                     WHERE seq > ?1
                     ORDER BY seq ASC
                     LIMIT ?2",
                )?;

                let mut rows = stmt.query(rusqlite::params![requested_seq as i64, limit as i64])?;
                let mut events = Vec::new();
                let mut last_seq_seen = requested_seq;

                while let Some(row) = rows.next()? {
                    let seq: u64 = row.get::<_, i64>(0)? as u64;
                    last_seq_seen = seq;
                    let et_str: String = row.get(8)?;
                    let rc_str: String = row.get(9)?;
                    let kn_str: String = row.get(10)?;
                    let payload_str: String = row.get(14)?;
                    let cause_id: Option<i64> = row.get(11)?;

                    events.push(CanonicalEvent {
                        schema_version: row.get(2)?,
                        event_id: row.get(1)?,
                        cursor: LedgerCursor {
                            ledger_id: expected_ledger.clone(),
                            seq,
                        },
                        kind: row.get(4)?,
                        environment: row.get(5)?,
                        instrument_id: row.get(6)?,
                        wallet_id: row.get(7)?,
                        event_time: DateTime::parse_from_rfc3339(&et_str)
                            .map(|d| d.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        received_at: DateTime::parse_from_rfc3339(&rc_str)
                            .map(|d| d.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        known_at: DateTime::parse_from_rfc3339(&kn_str)
                            .map(|d| d.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now()),
                        cause_ids: cause_id.map(|id| vec![id.to_string()]).unwrap_or_default(),
                        before_revision: row.get::<_, i64>(12)? as u64,
                        after_revision: row.get::<_, i64>(13)? as u64,
                        payload: serde_json::from_str(&payload_str).unwrap_or(Value::Null),
                        quality: row.get(15)?,
                    });
                }

                Ok(TailResult {
                    ledger_id: expected_ledger.clone(),
                    events,
                    next_cursor: LedgerCursor {
                        ledger_id: expected_ledger,
                        seq: last_seq_seen,
                    },
                    high_water_mark: current_seq,
                    retention_floor,
                })
            })
            .await
            .map_err(|e| TailError::Database(e.to_string()))
    }

    /// Advance the retention floor to prune historical events below the floor (C4/C5).
    pub async fn advance_retention_floor(
        &self,
        new_floor: u64,
    ) -> Result<usize, tokio_rusqlite::Error> {
        let ledger_id = self.ledger_id.clone();
        self.conn()
            .call(move |conn| {
                let tx = conn.transaction()?;
                tx.execute(
                    "UPDATE ledger_meta SET retention_floor = MAX(retention_floor, ?1) WHERE ledger_id = ?2",
                    rusqlite::params![new_floor as i64, ledger_id],
                )?;
                let pruned = tx.execute(
                    "DELETE FROM outbox_events WHERE seq < ?1",
                    rusqlite::params![new_floor as i64],
                )?;
                tx.commit()?;
                Ok(pruned)
            })
            .await
    }

    /// Reconstruct an InMemoryBook from committed position checkpoints (C2 Step 3 / C4).
    pub async fn restore_into_book(
        &self,
        book: &mut crate::book::InMemoryBook,
    ) -> Result<usize, tokio_rusqlite::Error> {
        let snapshot = self.snapshot_page(0, 100_000).await?;
        let count = snapshot.positions.len();
        for cp in snapshot.positions {
            let state = crate::state::PositionState {
                address: cp.address.clone(),
                coin: cp.coin.clone(),
                szi: cp.szi,
                direction: cp.direction,
                opened_at: cp.opened_at,
                last_added_at: cp.last_added_at,
                avg_entry: cp.avg_entry,
                entry_qty_total: cp.szi.abs(),
                exit_qty: Decimal::ZERO,
                exit_notional: Decimal::ZERO,
                realized_pnl: cp.realized_pnl,
            };
            book.reseed_wallet(
                &cp.address,
                vec![state],
                std::collections::HashMap::new(),
                None,
            );
        }
        Ok(count)
    }
}
