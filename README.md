# databaseschema

**Open-source core schema for the TradingPlatform** — Diesel/PostgreSQL models and query helpers for backtesting, strategy management, portfolios, and live-trading state.

---

## Table of Contents

- [Overview](#overview)
- [What's Included / Excluded](#whats-included--excluded)
- [Schema](#schema)
- [Workspace Layout](#workspace-layout)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Migrations](#migrations)
- [Testing](#testing)
- [License](#license)

---

## Overview

`databaseschema` is the shared data-access layer consumed by `BacktestingCore`, `DataEngineCore`, `SignalEngine`, and their private counterparts. It defines:

1. **Diesel schema** (`src/schema.rs`) — table definitions generated from the migration
2. **Model structs** (`src/models/`) — `Queryable`/`Insertable`/`Identifiable` structs per table
3. **Query helpers** (`src/ops/`) — hand-written async query functions for the access patterns the platform actually uses (not a full ORM-repository layer — only what's needed)
4. **Connection pooling** (`src/lib.rs`) — `deadpool` + `diesel-async` pool construction from `DATABASE_URL`

It is a **library crate only** — there is no standalone service to run. Consuming binaries (a backtesting engine, a signal engine, a data engine) depend on it as a path or git dependency and run migrations against their own PostgreSQL instance.

### Key Characteristics

| Attribute | Value |
|-----------|-------|
| **Database** | PostgreSQL (via Diesel 2.2 + `diesel-async`) |
| **Multi-tenancy** | None — deliberately single-tenant, no `tenant_id` column anywhere |
| **Connection pooling** | `deadpool` |
| **Migrations** | Diesel CLI, one consolidated `up.sql`/`down.sql` |

---

## What's Included / Excluded

This schema is a **deliberate subset** of a larger private schema used internally. It contains only the tables needed to run backtests, manage strategies/portfolios, and track live-trading state:

**Included:** strategies, optimization runs, strategy instances/parameters, portfolios, backtest results/jobs/reports/trades, smart-order-routing audit trail, deployed strategies, positions, trade history, P&L snapshots, kill-switch events, exchange credentials, market data health, derivative instruments.

**Excluded (stays in the private platform's schema):** multi-tenancy (`tenant_id`, `tenants`), billing/subscription tables, and AI-research/conversation tables. None of the tables in this crate carry a `tenant_id` column — if you need multi-tenant isolation, add it at your application layer (e.g. a wrapping table, row-level security, or a separate schema per tenant).

Soft cross-references (e.g. `backtest_jobs.result_id`, `backtest_results.strategy_instance_id`) are plain nullable UUID columns **without enforced foreign keys** — these are conceptually optional/deferred pointers (a job doesn't have a result until it completes), not tight parent-child relationships. Enforced FKs are only used where a row is meaningless without its parent (e.g. `optimization_runs.strategy_id`, `strategy_orders.order_id` → fills/state changes).

---

## Schema

### Strategies & Optimization

| Table | Purpose |
|-------|---------|
| `strategies` | Strategy definitions, approval workflow (`approval_status`, `approved_by`, etc.), base configuration |
| `optimization_runs` | Parameter-search runs (method, objective function, ranges, best result) tied to a strategy |
| `strategy_instances` | A concrete parameterization of a strategy, optionally produced by an optimization run |
| `strategy_parameters` | Declared parameter schema for a strategy (type, bounds, whether it's optimizable) |
| `strategy_approval_history` | Audit trail of approval-status transitions |

### Portfolios

| Table | Purpose |
|-------|---------|
| `portfolios` | Named portfolio with a rebalance policy |
| `portfolio_assets` | Per-asset weight/strategy assignment within a portfolio, including the asset's Python strategy source |

### Backtests

| Table | Purpose |
|-------|---------|
| `backtest_results` | Full metrics output of a single backtest run (returns, Sharpe/Sortino/Calmar, drawdown, VaR/ES, slippage, order stats) |
| `backtest_jobs` | Job queue/progress tracking for async backtest execution, including genetic-optimization generation progress and job lineage (`parent_job_id`/`root_job_id`) |
| `backtest_reports` | Generated report artifacts (HTML/PDF/etc.) tied to a result |
| `backtest_report_access_log` | Access audit log for reports |
| `backtest_position_history` | Time series of simulated positions during a backtest |
| `backtest_trades` | Individual simulated fills during a backtest |

### Smart Order Routing (audit trail)

| Table | Purpose |
|-------|---------|
| `strategy_orders` | Order lifecycle (signal → routing → fills → completion), consumed by `SignalEngine`'s `smartorderrouter`/execution path |
| `strategy_order_fills` | Individual fills against an order, including maker/taker and spread-at-fill |
| `strategy_order_state_changes` | State-transition audit trail per order |

### Live Trading

| Table | Purpose |
|-------|---------|
| `deployed_strategies` | A strategy instance running live (paper or real), capital allocation, risk limits, running P&L |
| `deployment_positions` | Current open position per (deployment, exchange, symbol) |
| `trade_history` | Executed live trades, partitioned by `(executed_at, id)` |
| `pnl_snapshots` | Periodic aggregate P&L snapshots across deployments |
| `kill_switch_events` | Emergency-halt audit trail |
| `exchange_credentials` | Encrypted exchange API credentials |
| `market_data_health` | Per-(exchange, symbol) feed health (tick rate, gap count) used for data-quality gating |
| `derivative_instruments` | Options/futures/perpetual contract specs (strike, expiry, multiplier, tick/lot size) |

---

## Workspace Layout

```
databaseschema/
├── src/
│   ├── lib.rs              # Connection pool construction, module re-exports
│   ├── schema.rs            # Diesel table! definitions
│   ├── errors.rs            # Error types
│   ├── models/               # One file per table: Queryable + Insertable structs
│   └── ops/                  # Async query helpers (one file per table group)
├── migrations/
│   └── 2026-01-01-000000_oss_core_schema/
│       ├── up.sql
│       └── down.sql
├── tests/
│   └── smoke_test.rs
└── diesel.toml
```

---

## Quick Start

### Prerequisites

- Rust 1.82+
- PostgreSQL 14+
- Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)

### Set up the database

```powershell
$env:DATABASE_URL = "postgresql://user:pass@localhost/trading"

cd databaseschema
diesel migration run
```

### Add as a dependency

```toml
[dependencies]
databaseschema = { path = "../databaseschema" }
```

---

## Usage

### Connection pool

```rust
use databaseschema::{create_connection_pool, get_connection};

let pool = create_connection_pool(); // reads DATABASE_URL
let mut conn = get_connection(&pool).await?;
```

### Inserting a strategy

```rust
use databaseschema::models::strategy::NewStrategy;
use diesel_async::RunQueryDsl;

let new_strategy = NewStrategy {
    strategy_name: "mean_reversion_v1".to_string(),
    strategy_type: "mean_reversion".to_string(),
    version: "1.0.0".to_string(),
    is_active: false,
    approval_status: "pending".to_string(),
    // ...remaining fields
    ..Default::default()
};

diesel::insert_into(databaseschema::schema::strategies::table)
    .values(&new_strategy)
    .execute(&mut conn)
    .await?;
```

### Using an ops helper

```rust
use databaseschema::ops::deployed_strategy_ops;

let active = deployed_strategy_ops::get_active_deployments(&mut conn).await?;
```

---

## Migrations

This crate ships a single consolidated migration (`2026-01-01-000000_oss_core_schema`) rather than the incremental migration history of the private platform it was extracted from — it represents the OSS schema's starting point, not a historical record.

```powershell
diesel migration run       # apply
diesel migration redo      # rollback + reapply, for testing down.sql
diesel print-schema         # regenerate src/schema.rs after a manual change
```

If you extend the schema, add a new migration rather than editing `up.sql` in place:

```powershell
diesel migration generate add_my_table
```

---

## Testing

```powershell
$env:DATABASE_URL = "postgresql://user:pass@localhost/trading_test"
cargo test
```

`tests/smoke_test.rs` verifies the pool connects and the core tables round-trip.

---

## License

Functional Source License, Version 1.1, ALv2 Future License (FSL-1.1-ALv2) — see [LICENSE](LICENSE). Free for internal use, non-commercial research/education, and professional services; converts to Apache License 2.0 two years after each version's release. See the platform's licensing FAQ for what this means in practice — in short, you can't resell this schema (or a thin wrapper around it) as a competing hosted service, but you can use it, modify it, and build on it for anything else.
