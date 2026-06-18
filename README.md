# Fragments API — a Rust course for Java/Spring developers

A small, **test-driven** REST API you build yourself, modelled on a thin slice of
a typical media-ingest service. Every function body starts as
`todo!()`; the **tests define the behaviour**. Your loop is:

> read the one failing test → implement the stub → `cargo test` → green →
> delete the next `#[ignore]` → repeat.

Each concept is mapped back to **Java/Spring**, since that's where you're coming
from. No async, no networking until the very end — you master structs, enums,
ownership and collections first; `axum` keeps the HTTP layer almost invisible.

Two things make this approachable rather than overwhelming:

- **One step is active at a time.** Every test except the current step's carries
  an `#[ignore = "step N: delete this line to start this step"]` attribute, so a
  fresh clone shows just the **step 1** tests red — not 20 failures at once. When
  a step is green, delete the next step's `#[ignore]` line to unlock it.
- **Java breadcrumbs in every stub.** Each `todo!()` is preceded by a doc-comment
  showing the Spring/Java code to port. Your job is to translate that snippet into
  idiomatic Rust — the behaviour is pinned by the test.

## Branches: exercise vs. solution

This repo is meant to be *worked through*, so the default branch ships the
**exercise**, not the answers:

- **`master` (default)** — every function body the course asks you to write is
  `todo!()`, so `cargo test` is **red**. This is your starting point.
- **`solution`** — the same project fully implemented, all tests green. Use it as
  a reference when you're stuck, or to diff your own work against.

```bash
git clone <repo> && cd fragments-api-rust   # you're on master (the exercise)
cargo test                                  # only step 1 is active (red) — expected
git switch solution                         # peek at the reference answers
git switch master                           # back to your work
```

## What you're building

An in-memory CRUD API for *media fragments*:

| Method & path          | Meaning                         | Spring equivalent          |
|------------------------|---------------------------------|----------------------------|
| `GET /healthz`         | liveness probe → `200`          | actuator health            |
| `GET /fragments`       | list all                        | `@GetMapping`              |
| `GET /fragments/{id}`  | one, or `404`                   | `@GetMapping("/{id}")`     |
| `POST /fragments`      | create → `201`                  | `@PostMapping @RequestBody`|
| `DELETE /fragments/{id}`| delete → `204`, or `404`       | `@DeleteMapping`           |

## Project layout

```
src/
  model.rs       Step 1-2     enums + Fragment struct (serde ≈ Jackson)
  repository.rs  Step 3       in-memory @Repository + FragmentStore trait (the seam)
  state.rs       Step 4       Arc<Mutex<..>>  ≈ a shared singleton bean
  api.rs         Step 5-6     handlers + router ≈ @RestController
  error.rs       Step 6       AppError + IntoResponse ≈ @ControllerAdvice
  main.rs        Step 7 / A   @SpringBootApplication main() + worker wiring
  message.rs     Milestone A  inbound ConsumedMessage (≈ Kafka ConsumerRecord)
  dispatcher.rs  Milestone A  Dispatcher trait + FakeDispatcher (≈ @Service stub)
  worker.rs      Milestone A  consume→dispatch→store loop (≈ @KafkaListener)
tests/
  model_tests.rs        unit tests (JUnit)
  repository_tests.rs   unit tests
  api_tests.rs          MockMvc-style integration via tower oneshot
  worker_tests.rs       Milestone A consumer tests (mpsc + FakeDispatcher)
```

## Rust ↔ Java/Spring cheat-sheet

| Rust | Java/Spring | Note |
|------|-------------|------|
| `struct` with `pub` fields | POJO / `record` | data holder |
| `#[derive(Serialize, Deserialize)]` | Jackson | JSON mapping |
| `enum FragmentType` + `match` methods | `enum` with fields + getters | see Step 1 |
| `Option<T>` | `Optional<T>` | absence without `null` |
| `Result<T, E>` + `?` | checked exceptions / `throws` | error propagation |
| `Arc<Mutex<T>>` | singleton bean | shared, thread-safe state |
| `axum::Router` | `@RestController` mappings | routing |
| `IntoResponse` | `ResponseEntity` / `@ExceptionHandler` | HTTP mapping |

## How the staggered exercise works

Tests are **gated** so you face one step at a time:

```rust
#[ignore = "step 2: delete this line to start this step"]
#[test]
fn fragment_new_defaults_to_queued() { ... }
```

- `cargo test` runs only the **un-ignored** tests (step 1 on a fresh clone).
- When those pass, open the relevant test file and **delete the `#[ignore]` line**
  for the next step. `cargo test` now shows that step red — implement its stub(s).
- Each stub names its step, e.g. `todo!("step 3: insert fragment ...")`, and the
  doc-comment above it carries the **Java to port**.

To run a single suite while you work: `cargo test --test model_tests`. To preview
every step at once (all red), run `cargo test -- --include-ignored`.

## The steps

### Step 1 — Enums (`src/model.rs`)
Implement `FragmentType::type_str()` and `table_name()` with `match self { ... }`.
This is the headline Java→Rust lesson: a Java `enum` *with fields and getters*
becomes a fieldless Rust enum whose per-variant data lives in `match`
expressions. The `serde` wire formats are already wired via `#[serde(...)]`.

```bash
cargo test --test model_tests
```

### Step 2 — `Fragment::new` (`src/model.rs`)
Write the constructor; new fragments always start `Queued`. Same test file.

### Step 3 — Repository (`src/repository.rs`)
Implement `insert / get / list / delete / len / is_empty` over the `HashMap`.
Pure data-structure work — get comfortable with ownership, `Option`, `&mut self`.

```bash
cargo test --test repository_tests
```

### Step 4 — Shared state (`src/state.rs`)
Implement `new_shared_state()` → `Arc<Mutex<FragmentRepository>>`. Same test file
(`shared_state_is_shared_across_threads`).

### Step 5 — Handlers + router (`src/api.rs`)
Implement `health`, `list_fragments`, and `build_router`. Now async appears, but
`axum` hides the plumbing: an extractor argument (`State`, `Path`, `Json`) is just
a method parameter; the return value is the `ResponseEntity`.

```bash
cargo test --test api_tests
```

### Step 6 — Errors + remaining handlers (`src/error.rs`, `src/api.rs`)
Implement `IntoResponse for AppError`, then `get_fragment`, `create_fragment`,
`delete_fragment`. Returning `Result<_, AppError>` from a handler is how axum
does `@ExceptionHandler`. Same test file goes fully green.

### Step 7 — Wire `main` (`src/main.rs`)
Build the state + router, bind `0.0.0.0:8080`, `axum::serve`. Then run it:

```bash
cargo run
curl -s localhost:8080/healthz
curl -s -X POST localhost:8080/fragments \
  -H 'content-type: application/json' \
  -d '{"id":"1","messageType":"UPDATE","fragmentType":"movie","messageTs":1,"fragment":"hi"}'
curl -s localhost:8080/fragments
```

### Milestone A — Kafka-style consumer (`src/model.rs`, `src/worker.rs`, `src/main.rs`)
Now the project grows from a CRUD API into a mini **ingest consumer**. The seams
are given for you — the `Dispatcher` and `FragmentStore` traits, `FakeDispatcher`,
`ConsumedMessage`, and the `Worker`'s `mpsc` receive loop — so you focus on the
business logic:

1. `Fragment::with_status` (`src/model.rs`) — like `new`, but the caller supplies
   the dispatch outcome to persist.
2. `Worker::process` (`src/worker.rs`) — derive UPDATE/DELETE from the message,
   dispatch downstream, then store a `Fragment::with_status`. This is the
   consume→dispatch→store heart of the loop.
3. Wire `main` (`src/main.rs`) — spawn the worker beside the HTTP server, sharing
   one `Arc<Mutex<..>>` store, so `GET /fragments` reflects what the worker ingests.

```bash
cargo test --test worker_tests
```

The design and the staged roadmap (Postgres, then real Kafka) live in
`docs/plans/2026-06-18-kafka-consumer-rust-design.md`.

## Running everything

```bash
cargo test                       # only the active (un-ignored) step
cargo test -- --include-ignored  # every step at once (all red) — the full picture
cargo test --test model_tests    # one suite at a time
cargo run                        # start the server (after Step 7)
```

## The domain

A media-metadata "fragment" model (`Fragment`, `FragmentType`, `FragmentStatus`,
`MessageType`) — a generic, self-contained slice with no external dependencies, so
you can focus purely on the Rust concepts.
