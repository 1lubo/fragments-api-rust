//! Step 3 & 4 tests — the in-memory repository and shared state.
//!
//! Java/Spring parallel: unit tests for your `@Repository`. No HTTP, no async —
//! pure data-structure logic, the best place to get comfortable with ownership,
//! `Option`, and `HashMap`.
//! Run with:  `cargo test --test repository_tests`

use std::thread;

use fragments_api::model::{Fragment, FragmentStatus, FragmentType, MessageType};
use fragments_api::repository::FragmentRepository;
use fragments_api::state::new_shared_state;

fn sample(id: &str) -> Fragment {
    Fragment::new(
        id.to_string(),
        MessageType::Update,
        FragmentType::Movie,
        1,
        "payload".to_string(),
    )
}

// ---- Step 3: repository CRUD ----------------------------------------------

#[test]
fn new_repository_is_empty() {
    let repo = FragmentRepository::new();
    assert!(repo.is_empty());
    assert_eq!(repo.len(), 0);
    assert!(repo.list().is_empty());
}

#[test]
fn insert_then_get_returns_fragment() {
    let mut repo = FragmentRepository::new();
    repo.insert(sample("a"));

    assert_eq!(repo.len(), 1);
    assert!(!repo.is_empty());
    assert_eq!(repo.get("a"), Some(sample("a")));
    assert_eq!(repo.get("missing"), None);
}

#[test]
fn insert_same_id_overwrites() {
    let mut repo = FragmentRepository::new();
    repo.insert(sample("a"));

    let mut updated = sample("a");
    updated.status = FragmentStatus::Processed;
    repo.insert(updated.clone());

    assert_eq!(repo.len(), 1);
    assert_eq!(repo.get("a"), Some(updated));
}

#[test]
fn list_returns_all_inserted() {
    let mut repo = FragmentRepository::new();
    repo.insert(sample("a"));
    repo.insert(sample("b"));

    let mut ids: Vec<String> = repo.list().into_iter().map(|f| f.id).collect();
    ids.sort();
    assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn delete_removes_and_reports_existence() {
    let mut repo = FragmentRepository::new();
    repo.insert(sample("a"));

    assert!(repo.delete("a"));
    assert!(!repo.delete("a")); // already gone
    assert!(repo.is_empty());
}

// ---- Step 4: shared state -------------------------------------------------

#[test]
fn shared_state_is_shared_across_threads() {
    let state = new_shared_state();

    {
        let mut repo = state.lock().unwrap();
        repo.insert(sample("a"));
    }

    // Clone the Arc and mutate from another thread; the original sees it.
    let clone = state.clone();
    thread::spawn(move || {
        clone.lock().unwrap().insert(sample("b"));
    })
    .join()
    .unwrap();

    assert_eq!(state.lock().unwrap().len(), 2);
}
