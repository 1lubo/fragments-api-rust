# Fragments API — a Rust course for Java/Spring developers

A small, **test-driven** REST API you build yourself, modelled on a thin slice of
a typical media-ingest service. Every function body starts as
`todo!()`; the **tests define the behaviour**. Your loop is:

> read the failing test → implement the stub → `cargo test` → green → next step.

Each concept is mapped back to **Java/Spring**, since that's where you're coming
from. No async, no networking until the very end — you master structs, enums,
ownership and collections first; `axum` keeps the HTTP layer almost invisible.

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
  model.rs       Step 1-2  enums + Fragment struct (serde ≈ Jackson)
  repository.rs  Step 3    in-memory @Repository over a HashMap
  state.rs       Step 4    Arc<Mutex<..>>  ≈ a shared singleton bean
  api.rs         Step 5-6  handlers + router ≈ @RestController
  error.rs       Step 6    AppError + IntoResponse ≈ @ControllerAdvice
  main.rs        Step 7    @SpringBootApplication main()
tests/
  model_tests.rs        unit tests (JUnit)
  repository_tests.rs   unit tests
  api_tests.rs          MockMvc-style integration via tower oneshot
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

## Running everything

```bash
cargo test            # all tests (red until you implement each step)
cargo test --test model_tests   # one step at a time
cargo run             # start the server (after Step 7)
```

## The domain

A media-metadata "fragment" model (`Fragment`, `FragmentType`, `FragmentStatus`,
`MessageType`) — a generic, self-contained slice with no external dependencies, so
you can focus purely on the Rust concepts.
