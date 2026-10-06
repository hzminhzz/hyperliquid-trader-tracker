"""Baseline qualification tests: codifying inspected gaps G01-G11 as reproducible baseline findings.

These tests prove the baseline code behavior and structural limitations identified in
docs/STATUS.md before implementation of S1-S3.
"""

from datetime import UTC, datetime
from decimal import Decimal
from pathlib import Path
from typing import Any

import pytest

from tracker.book import InMemoryBook, SeenTids
from tracker.config import Settings
from tracker.db import WatchlistDB
from tracker.enrich import Enricher
from tracker.resolve import seed_state_from_row

D = Decimal
TS = datetime(2026, 6, 15, 12, 0, tzinfo=UTC)


class _FakeInfoClient:
    """Stands in for InfoClient returning custom clearinghouseState responses."""

    def __init__(self, response: Any) -> None:
        self._response = response

    async def info(self, body: dict[str, Any]) -> Any:
        _ = body
        return self._response


def test_g01_seen_tids_unscoped_dedup() -> None:
    """G01: SeenTids deduplicates only on raw scalar tid without coin/wallet scoping.

    If two trades on different coins share a tid (e.g. simulated collision or hash
    misunderstanding), the second trade is rejected as a duplicate.
    """
    seen = SeenTids(maxlen=10)
    tid = 999999

    # First trade on BTC with tid
    is_dup_1 = seen.check_and_add(tid)
    assert not is_dup_1, "First trade must not be flagged as duplicate"

    # Second trade on ETH with identical tid
    is_dup_2 = seen.check_and_add(tid)
    assert is_dup_2, (
        "Baseline SeenTids drops the second trade because dedup is unscoped to (coin, address)"
    )


def test_g05_fill_epoch_resets_on_reseed() -> None:
    """G05: InMemoryBook.fill_epoch resets to 0 when reseed_wallet is called.

    reseed_wallet invokes drop_wallet, which drops the wallet's epoch counter rather than
    maintaining a monotonically increasing state revision.
    """
    book = InMemoryBook()
    # Ingest a trade to advance epoch to 1
    _ = book.ingest(address="0xa", coin="BTC", delta=D("1"), px=D("100"), ts=TS)
    assert book.fill_epoch("0xa") == 1

    # Reseed wallet
    seeded = seed_state_from_row("0xa", "BTC", D("1"), D("100"), fallback_ts=TS)
    _ = book.reseed_wallet("0xa", [seeded], {"BTC": 10}, expected_epoch=None)

    # In baseline, fill_epoch was removed in drop_wallet, so fill_epoch resets to 0
    assert book.fill_epoch("0xa") == 0, (
        "Baseline reseed_wallet resets fill_epoch to 0 via drop_wallet"
    )


@pytest.mark.asyncio
async def test_g06_missing_asset_positions_treated_as_flat() -> None:
    """G06: Missing assetPositions in clearinghouseState payload parses as empty positions.

    Instead of raising an error or quarantining the unvalidated response,
    enricher treats missing assetPositions as empty, which leads to clearing positions.
    """
    book = InMemoryBook()
    # Populate existing position in book
    _ = book.ingest(address="0xa", coin="BTC", delta=D("2"), px=D("100"), ts=TS)
    assert book.position("0xa", "BTC") is not None

    client = _FakeInfoClient({"marginSummary": {"accountValue": "1000"}})
    enricher = Enricher(Settings(), book, client)
    seeded = await enricher.seed_wallet("0xa")

    assert seeded is True
    # Position was wiped because missing assetPositions was treated as []
    assert book.position("0xa", "BTC") is None, (
        "Baseline seed_wallet wipes positions when assetPositions key is missing"
    )


def test_g07_seeded_position_uses_observation_time() -> None:
    """G07: Seeded positions use the observation-start fallback timestamp.

    Actual historical entry time is unobserved on snapshot seeding.
    """
    now = datetime(2026, 10, 6, 15, 30, tzinfo=UTC)
    seeded = seed_state_from_row("0xa", "BTC", D("5"), D("50000"), fallback_ts=now)

    assert seeded.opened_at == now
    assert seeded.last_added_at == now


@pytest.mark.asyncio
async def test_g11_database_persists_only_subscriptions(tmp_path: Path) -> None:
    """G11: Database persists only subscriptions; book positions/epochs are memory-only."""
    db = WatchlistDB(tmp_path / "tracker.db")
    await db.connect()
    try:
        assert db._conn is not None
        async with db._conn.execute(
            "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
        ) as cur:
            tables = [row[0] for row in await cur.fetchall()]
        # Only 'subscriptions' exists
        assert tables == ["subscriptions"], (
            f"Expected only subscriptions table at baseline, got {tables}"
        )
    finally:
        await db.aclose()
