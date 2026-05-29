# Chess multiplayer roadmap

## Where we are now

| Done | Not done |
|------|----------|
| Bevy game, offline + AI moves | Shared rules crate |
| SpacetimeDB connect + `Player` table | `Game` / moves on server |
| `is_loading` + network plugin | Online click → server → sync board |

## The big picture

**Offline:** validate in Rust → update Bevy.

**Online:** click → server reducer (same rules) → DB updates → both clients refresh the board from the DB.

---

## Phase 1 — Foundation (no multiplayer gameplay yet)

### Step 1 — `chess_core` crate

- [ ] Add `core/` to the workspace
- [ ] Move pure logic: `Piece`, colors/types, `is_move_valid`, check/checkmate, path helpers
- [ ] Client calls `chess_core` from `move_logic`; behavior should match today
- **Done when:** offline/AI still play exactly as now

### Step 2 — Server uses `chess_core` (smoke test)

- [ ] Add `chess_core` to `server/Cargo.toml`
- [ ] Tiny reducer e.g. `ping_move_valid(from, to, …)` that returns Ok/Err (or log only)
- **Done when:** `spacetime publish` + one test call proves server compiles with shared rules

---

## Phase 2 — Data model on SpacetimeDB

### Step 3 — `game` table

- [ ] Fields: `id`, `white`, `black`, `turn`, `status`, board (`fen` or piece rows)
- [ ] Reducers: `create_game`, `join_game` (second player fills `black`)
- **Done when:** two clients can create/join and see the same `game` row in subscriptions

### Step 4 — `submit_move` reducer

- [x] Args: `game_id`, `from_x/y`, `to_x/y`
- [x] Checks: caller is in game, correct turn, `chess_core::is_legal_move`
- [x] Updates board + turn; optional `move` log table (log table not added yet)
- **Done when:** calling reducer from CLI/SDK updates DB; illegal moves return errors

### Step 5 — Regenerate client bindings

- [ ] `spacetime generate` → new tables/reducers in `module_bindings`
- [ ] Subscribe to `game` (+ `move` if added)
- **Done when:** client compiles and receives table updates after a reducer runs

---

## Phase 3 — Wire Bevy to the server

### Step 6 — Matchmaking flow (minimal)

- [x] Play Online → connect → create/join game → store `game_id` + your color in a resource
- [x] **Done when:** UI shows “waiting for opponent” / “in game” from real DB state

### Step 7 — Online input path

- [x] Multiplayer click: do not run local `move_piece`
- [x] Call `conn.reducers.submit_move(...)`
- **Done when:** click sends reducer; no local board change until server responds

### Step 8 — Sync board from server

- [x] On `game` update: rebuild or patch Bevy pieces from server board
- [x] Reuse turn / game-over from server (or derive with `chess_core` after sync)
- **Done when:** two clients see the same moves with one machine authoritative

---

## Phase 4 — Polish

### Step 9 — Disconnect / back to menu

- [ ] `cleanup_network` + leave game on server (forfeit or mark abandoned)
- [ ] Reset local state like `replay.rs` today

### Step 10 — UX & hardening

- [ ] Loading / error toasts, reconnect, optional move preview (client-only, grayed)
- [ ] Promotion, castling edge cases if not already in `chess_core`

---

## Ignore for now

- Perfect lobby / ELO / spectators
- FEN on server if piece rows are easier — pick one in Step 3 and stick to it
- Rewriting all of Bevy’s `Piece` as `Component` inside `core` (keep mapping at the boundary)

---

## Step 3 decision (pick when we get there)

**Board on server:**

- **A)** `fen: String` on `game` — small schema, need FEN parse/apply in `chess_core` later
- **B)** `piece` rows per game — matches `Piece` struct today

Either works; B often matches current code faster.

---

## Pair-program order

Go **Step 1 → Step 2 → …** — only start Step N+1 when Step N works.

| Step | Main files |
|------|------------|
| 1 | `core/`, `src/pieces/*`, `Cargo.toml` workspace |
| 2 | `server/src/lib.rs` |
| 3–4 | `server/src/lib.rs` |
| 5 | `module_bindings/`, `connection.rs` subscriptions |
| 6–8 | `network/`, `move_logic.rs`, maybe `game_state.rs` |
| 9–10 | `replay.rs`, UI, edge cases |
