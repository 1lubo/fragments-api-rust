//! In-memory repository.
//!
//! Java/Spring parallel: this is your `@Repository` / DAO. Instead of JDBC or
//! JPA talking to a database, we keep everything in a `HashMap` in memory. The
//! public methods are the contract the rest of the app codes against — just
//! like a Spring Data repository interface.

use std::collections::HashMap;

use crate::model::Fragment;

/// Java/Spring: the repository bean. The `fragments` map is `private` (no `pub`),
/// so callers must go through the methods below — encapsulation, same as a
/// private field with public methods in Java.
#[derive(Debug, Default)]
pub struct FragmentRepository {
    fragments: HashMap<String, Fragment>,
}

impl FragmentRepository {
    /// Java/Spring: `new FragmentRepository()`. `Default` already gives us an
    /// empty map; this is just a conventional, explicit constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Java/Spring: `save(fragment)`. Insert (or overwrite) by `fragment.id`.
    ///
    /// Note ownership: `fragment` is taken *by value* — the repository now owns
    /// it. TODO(step 3): insert the fragment into the map keyed by its `id`.
    ///
    /// Java to port: `fragments.put(fragment.getId(), fragment);`
    pub fn insert(&mut self, fragment: Fragment) {
        todo!("step 3: insert fragment into the map keyed by fragment.id")
    }

    /// Java/Spring: `findById(id)` returning `Optional<Fragment>`. Rust uses
    /// `Option<...>` for the same idea. We return a clone to keep the example
    /// simple (no lifetimes/borrowing puzzles for the caller).
    ///
    /// TODO(step 3): look up `id` and return `Some(clone)` or `None`.
    ///
    /// Java to port: `return Optional.ofNullable(fragments.get(id));`
    pub fn get(&self, id: &str) -> Option<Fragment> {
        todo!("step 3: get a clone of the fragment with this id, or None")
    }

    /// Java/Spring: `findAll()`. Order is unspecified (it's a HashMap).
    ///
    /// TODO(step 3): return all fragments as a `Vec<Fragment>` (clones).
    ///
    /// Java to port: `return new ArrayList<>(fragments.values());`
    pub fn list(&self) -> Vec<Fragment> {
        todo!("step 3: collect all fragments into a Vec")
    }

    /// Java/Spring: `deleteById(id)`. Returns `true` if something was removed.
    ///
    /// TODO(step 3): remove `id` from the map; return whether it existed.
    ///
    /// Java to port: `return fragments.remove(id) != null;`
    pub fn delete(&mut self, id: &str) -> bool {
        todo!("step 3: remove fragment by id, return true if it existed")
    }

    /// Java/Spring: `count()`.
    ///
    /// TODO(step 3): return the number of stored fragments.
    ///
    /// Java to port: `return fragments.size();`
    pub fn len(&self) -> usize {
        todo!("step 3: return the number of fragments")
    }

    /// Convenience used by tests and handlers.
    ///
    /// TODO(step 3): return `true` when there are no fragments.
    ///
    /// Java to port: `return fragments.isEmpty();`
    pub fn is_empty(&self) -> bool {
        todo!("step 3: return true when empty")
    }
}

/// Java/Spring: the persistence *interface* the rest of the app codes against —
/// the seam, like a Spring Data repository interface. Milestone A's only impl is
/// the in-memory `FragmentRepository`; Milestone C adds a `sqlx`/Postgres impl
/// behind this same trait, so the consumer/worker never changes.
///
/// The methods mirror the inherent ones above; this impl just forwards to them
/// (Rust resolves the calls to the inherent methods, so there's no recursion).
pub trait FragmentStore {
    fn insert(&mut self, fragment: Fragment);
    fn get(&self, id: &str) -> Option<Fragment>;
    fn list(&self) -> Vec<Fragment>;
    fn delete(&mut self, id: &str) -> bool;
}

impl FragmentStore for FragmentRepository {
    fn insert(&mut self, fragment: Fragment) {
        FragmentRepository::insert(self, fragment);
    }

    fn get(&self, id: &str) -> Option<Fragment> {
        FragmentRepository::get(self, id)
    }

    fn list(&self) -> Vec<Fragment> {
        FragmentRepository::list(self)
    }

    fn delete(&mut self, id: &str) -> bool {
        FragmentRepository::delete(self, id)
    }
}
