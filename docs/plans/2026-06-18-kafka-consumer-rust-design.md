# Design: Kafka-style consumer + persistence for `fragments-api-rust`

Date: 2026-06-18
Status: Validated (Milestone A to be implemented)

## Goal

Extend the learning REST API into a mini media-ingest **consumer**, modelled on a
thin slice of a typical media-ingest consumer service, to learn how Rust
works with Kafka-style message consumption and a Postgres-style store — while
keeping the project's test-driven, `cargo test`-only learning loop.

## Reference flow (the slice we mimic)

1. **Consume** messages keyed by fragment id; `null`/tombstone value ⇒ DELETE,
   otherwise UPDATE.
2. **Dispatch** to a downstream service → maps to a `FragmentStatus`
   (`Processed`, `DownstreamConnectionError`, `DownstreamError`, `Invalid`).
3. **Store** the fragment + its dispatch status in Postgres (per-`FragmentType`
   table — `FragmentType::table_name()` already mirrors this).
4. *(later)* Scheduled retry of non-`Processed` fragments.
5. *(later)* Scheduled cleanup + metrics.

## Staged plan (A → B → C)

The **traits (seams) stay fixed**; implementations get swapped per milestone.
Milestone letters follow implementation order (a → b → c).

- **Milestone A — in-memory (this doc):** in-memory fakes behind traits. No
  Docker. Teaches traits-as-interfaces, async tasks, `tokio::mpsc` channels,
  `Arc<Mutex<…>>` shared state, dependency inversion. Branch: merged to `master`.
- **Milestone B — real Postgres:** swap `FragmentStore` for a `sqlx` +
  Postgres impl (Docker). Teaches async SQL, pools, migrations, row mapping.
  Branch: `feature/milestone-b-postgres` (off `master`).
- **Milestone C — real Kafka:** swap the in-memory bus for an `rdkafka`
  consumer. Teaches real broker config, offsets, acks. Branch:
  `feature/milestone-c-kafka` (off `b`).

Each swap is a small diff that does **not** touch business logic — that is the
Rust dependency-inversion lesson.

## Milestone A — scope

Only behaviors **1–3**: consume → dispatch → store. No scheduling/cleanup/metrics.

### Modules & seams

| Piece | Role | Reference | Java analogy |
|-------|------|-----------|--------------|
| `ConsumedMessage` (`src/message.rs`) | `id`, `fragment_type`, `message_ts`, `value: Option<String>` (`None` = tombstone) | `ConsumerRecord<String,String>` | the Kafka record |
| `trait Dispatcher` (`src/dispatcher.rs`) | `dispatch(id, msg_type, value) -> FragmentStatus` | forward-to-downstream service | a `@Service` interface |
| `FakeDispatcher` | returns a configured status | real downstream HTTP client | test stub / Mockito |
| `trait FragmentStore` (`src/repository.rs`) | `insert` / `get` / `list` / `delete` | a DAO | Spring Data repo interface |
| `FragmentRepository` | **exists**; now `impl FragmentStore` | DAO impl | JPA-backed impl |
| `Worker<D, S>` (`src/worker.rs`) | consume→dispatch→store loop over an mpsc receiver | a Kafka listener's consume-then-store flow | `@KafkaListener` method |

Two deliberate Rust choices: `Worker` is **generic** over `D: Dispatcher` and
`S: FragmentStore` (static dispatch ≈ Java generics, keeps async-in-traits
simple later); the channel is `tokio::mpsc` (sender ≈ producer, receiver ≈
consumer poll loop).

### Data flow

```
produce (tests / tiny simulator)
   └─ tokio::mpsc::Sender<ConsumedMessage>   "the topic"
        └─ Worker: rx.recv().await           ≈ @KafkaListener poll loop
             1. value.is_some() ⇒ Update,  None ⇒ Delete
             2. status = dispatcher.dispatch(id, msg_type, value)   ≈ forward downstream
             3. store.insert(Fragment{ …, status })                 ≈ persist fragment
        Arc<Mutex<FragmentRepository>>  ◄── shared ──►  axum handlers (admin view)
```

Dispatch happens **first**; its `FragmentStatus` is what gets stored. `main`
spawns the worker beside the HTTP server (both on the tokio runtime); the lock
is held only for the brief `insert`.

### Model decisions

1. Add `Fragment::with_status(id, message_type, fragment_type, message_ts,
   fragment, status)`; keep `Fragment::new` (always `Queued`) for the REST POST.
2. Keep `fragment: String`; store `""` for deletes (signal intent via
   `message_type == Delete`). No serde/REST churn.
3. A consumed DELETE **records an event** (inserts a `Fragment` with
   `message_type = Delete`), it does **not** remove the row — kept separate from
   the admin `DELETE /fragments/{id}`.

### Cargo change

Add the `sync` feature to `tokio` (for `mpsc`). Everything else already present.

## TDD plan (Milestone A)

New file `tests/worker_tests.rs` (integration-style, like `api_tests.rs`):

1. `update_message_is_dispatched_and_stored_with_processed_status` — fake returns
   `Processed`; one `Some(..)` message ⇒ stored `Update` + `Processed`.
2. `failed_dispatch_stores_downstream_error_status` — fake returns
   `DownstreamError` ⇒ stored fragment carries it.
3. `delete_message_records_a_delete_event` — `value = None` ⇒ stored `Delete`,
   `fragment == ""`, dispatcher's status.
4. `worker_drains_multiple_messages` — push N, drop the `Sender`, assert all N
   stored (proves the `while let Some(..) = rx.recv().await` drain loop).

**Mechanics:** build channel, spawn worker with `FakeDispatcher` + shared store,
send messages, drop `Sender` so `recv()` returns `None` and the worker exits,
`await` the join handle, then assert against the shared store.

**Exit criteria:** the four worker tests green; existing tests still green;
`cargo run` spawns the worker beside the HTTP server and a tiny demo producer
pushes a couple of messages so `GET /fragments` shows them with statuses.
