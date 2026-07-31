# mundo_sapiens — PLAN.md

Living decision log for this project. Read this first at the start of every session; append new decisions as they're made, don't rewrite history.

## Context

Client sells magnets pre-made designs. Process: buy a design from a seller, laminate/back it with magnetic sheet, cut into individual magnets, mark a batch "ready", then sell arbitrary quantities out of a pool — individual magnets are never identified. Today this is tracked in a 30k+ row Excel sheet with no audit trail and doesn't scale.

## Decisions

1. **Inventory model: batch + append-only movement ledger.** Track inventory per production batch (not per physical magnet), with an immutable log of state transitions and quantities rather than mutable counters — gives a full audit trail for what would otherwise be unexplainable stock discrepancies.
2. **Platform: native desktop app.** Rust + egui (UI) + embedded Postgres (via `postgresql_embedded` or equivalent) + **sqlx** for queries/migrations. Single machine, single user — no networking or sync needed.
3. **Batch selection on sale: FIFO.** When marking N units sold for a design, draw from the oldest "listo" batch(es) first, automatically — no manual batch picking required.
4. **Distribution: GitHub Actions CI pipeline builds a Windows executable** on push/release, so the client always has an installable build without a local Rust toolchain.
5. **Architecture: one crate, four modules** (`domain`, `app`, `infra`, `presentation`) — not a Cargo workspace. Boundaries are enforced by convention/review (only `main.rs`, the composition root, may reach into `infra`; `presentation` only ever calls `app`), not by the compiler. Simpler to work in day-to-day for this project's size.
6. **`app` is the seam presentation calls through** (`AppFacade` trait), acting as the BFF this app needs even though it's in-process, not networked. Every method is `todo!()` for now — `infra` doesn't exist yet. `main.rs` wires `app::TodoAppFacade`; swapping in a real `infra`-backed implementation later is a one-line change there.
7. **UI language: Brazilian Portuguese** (the client is Brazilian) — all `presentation`-layer strings (labels, buttons, screen names) are pt-BR. Everything else (code identifiers, comments, domain/app/infra internals) stays in English. Pipeline state names are English enum variants: `Purchased, Magnetized, Cut, Ready, Sold` (maps to comprado/imantado/cortado/pronto/vendido).
8. **Theme: neutral base + brand accent**, derived from `logo.jpg`'s saturated palette (purple/cyan/yellow) rather than using those colors directly — they're too heavy for a data-table-heavy admin tool used for hours. Neutrals carry backgrounds/text; purple is the primary accent; semantic colors (warning/success/danger) are kept independent of brand hues. All defined once in `src/presentation/theme.rs` (colors + spacing constants + a `theme::apply(ctx)` call) — every screen reads from there, nothing styles itself ad hoc.
9. **`eframe`/`egui`/`egui_extras` pinned to 0.35.0**, with `eframe`'s `accesskit` default feature explicitly disabled (`default-features = false`, feature list re-added minus `accesskit`) — that version's `accesskit` → `zbus` dependency chain fails to compile against the installed Rust toolchain. Revisit if accessibility/screen-reader support is ever needed.

## Domain model sketch

Pipeline states per batch: `Purchased → Magnetized → Cut → Ready → Sold`

```
designs         (id, name, seller_id, image_path, ...)
sellers         (id, name, contact, ...)
purchases       (id, design_id, seller_id, cost, date)        -- step 1: buy design
batches         (id, design_id, purchase_id, qty_produced, date)
stock_movements (id, batch_id, from_state, to_state, qty, date, note)   -- append-only ledger
sales           (id, date, note)
sale_lines      (id, sale_id, batch_id, qty)                  -- which batch(es) a sale drew from
```

Current stock per state per batch = `SUM` over `stock_movements`, never a mutated field. "Marking 8 sold" = one new movement row (`Ready → Sold`, qty 8), batch chosen via FIFO. Excel imports of "ready" state become: parse sheet, aggregate by design, create/update batch + movement rows — not one row per magnet.

The dashboard (home screen) mirrors the client's own working sheet: `Design | Em Produção | Pronto`, one row per design. A "Pronto" count at or below 2 is highlighted — that's his own existing heuristic for "finish what's in production, or buy more from the seller."

10. **`infra` implemented: `postgresql_embedded` + sqlx, runtime (not compile-time) queries.** `sqlx::query!`/`query_as!` need `DATABASE_URL` reachable at `cargo build` time, which doesn't fit a DB that's spun up by the app itself — so `PgAppFacade` (`src/infra/facade.rs`) uses runtime `query`/`query_as` with explicit tuple types instead, losing compile-time SQL checking in exchange for not needing a build-time DB. Migrations live in `./migrations` (root-level, resolved relative to `CARGO_MANIFEST_DIR` by `sqlx::migrate!`) and run automatically on every startup via `bootstrap()`.
11. **Sync facade wraps an async client via a single `tokio::runtime::Runtime`, `block_on` per call.** `eframe`'s UI loop is sync; sqlx and `postgresql_embedded` are async. Rather than making the UI event loop async (channels, loading states — overkill here), `PgAppFacade` holds one `Runtime` and blocks on it per method call. Fine because the DB is local/embedded with sub-millisecond latency and there's a single user — this is the standard pragmatic pattern for small local desktop apps, not something that'd scale to a networked multi-user backend.
12. **Postgres data dir is persistent, not ephemeral.** `dirs::data_dir()/mundo_sapiens/pgdata`, `Settings.temporary = false`. The client's inventory can't reset every time the app closes. `PostgreSQL::setup()` is idempotent (checks for `postgresql.conf` before running `initdb`), so `bootstrap()` runs unconditionally on every launch.
13. **Bootstrap password is hardcoded; username is left at the crate's default.** `postgresql_embedded`'s `initdb` always names the bootstrap superuser `BOOTSTRAP_SUPERUSER` ("postgres"), ignoring `Settings.username` entirely — learned by hitting `FATAL: password authentication failed … Role "mundo_sapiens" does not exist` after setting a custom username. The password *is* honored (via `--pwfile`), but only applied on the very first `initdb`; if left at the crate's random default, a fresh random password every launch would stop matching the one already baked into the persisted data dir on the second run onward. So the password is a fixed app-specific string. Neither matters for security: this Postgres only ever listens on localhost, on a fresh OS-assigned port (`port = 0`), for a single local user.

## Current state (functional)

- `src/domain/` — plain entity structs + `BatchState` enum. No logic.
- `src/app/` — `AppFacade` trait + input DTOs. `infra::PgAppFacade` is the only implementation now; the old `TodoAppFacade` stub is gone.
- `src/infra/` — `bootstrap.rs` (starts/attaches to embedded Postgres, creates the DB, runs migrations), `facade.rs` (`PgAppFacade`: real queries per decisions #10–13), `state.rs` (`BatchState` ⇄ persisted string).
- `src/presentation/` — `theme.rs` + `screens/` (dashboard, purchase, production, sale, catalog), all wired to call `AppFacade` methods.
- `migrations/0001_init.sql` — full schema per the model sketch above.
- The app builds, launches, and runs against a real local database (`cargo run`). Verified: schema migrates correctly, the dashboard's stock-netting query and the FIFO available-batches query both produce correct results against hand-inserted data, and data persists across a full process restart (new `cargo run`, same rows still there).

## Open items for future sessions

- Set up the GitHub Actions workflow (build matrix, release artifact naming, versioning).
- Packaging details for the Windows build (installer vs standalone .exe).
- No automated tests yet — verification so far has been manual (`psql` against the running embedded instance). Worth adding integration tests once the crate is split so a test binary can reach `infra` (currently `[[bin]]`-only, no `[lib]` target).
- Currently-unused domain fields/theme constants (e.g. `Design.image_path`, `Seller.contact`, `CYAN`/`YELLOW`/`SUCCESS`/`DANGER`) are intentional — ahead of the screens that will use them, not dead weight to clean up.
