//! Domain model — inspired by a typical Java media-ingest service.
//!
//! Java/Spring parallel: this file is your `model` package. The enums and the
//! `Fragment` struct below are the Rust equivalent of the Java enums and the
//! `Fragment` POJO (the class with private fields + getters/setters).

use serde::{Deserialize, Serialize};

/// Java/Spring: `enum MessageType { UPDATE, DELETE }`.
///
/// `#[derive(Serialize, Deserialize)]` is the serde (≈ Jackson) machinery.
/// `#[serde(rename_all = "UPPERCASE")]` makes it (de)serialize as `"UPDATE"` /
/// `"DELETE"`, matching the Java enum constant names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MessageType {
    Update,
    Delete,
}

/// Java/Spring: `enum FragmentStatus { QUEUED, PROCESSED, ... }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FragmentStatus {
    Queued,
    Processed,
    DownstreamConnectionError,
    DownstreamError,
    Invalid,
}

/// Java/Spring: this is the interesting one. The Java version is an enum *with
/// fields and a constructor*:
///
/// ```java
/// MOVIE("movie", "movie_fragments");
/// private final String type;
/// private final String tableName;
/// public String getType() { return type; }
/// public String getTableName() { return tableName; }
/// ```
///
/// In Rust the variants carry no data; instead the per-variant values live in
/// `match` expressions inside methods (see `type_str` / `table_name` below).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FragmentType {
    Character,
    CreditContentMakers,
    Credit,
    Movie,
    Person,
    Programme,
    Rights,
    Season,
    Series,
    Sport,
    Taxonomy,
    Title,
}

impl FragmentType {
    /// Java/Spring: `getType()` — the lowercase wire/type string.
    ///
    /// TODO(step 1): return the type string for each variant via `match self`.
    /// Expected values:
    ///   Character => "character", CreditContentMakers => "creditcontentmarkers",
    ///   Credit => "credit", Movie => "movie", Person => "person",
    ///   Programme => "programme", Rights => "rights", Season => "season",
    ///   Series => "series", Sport => "sport", Taxonomy => "taxonomy",
    ///   Title => "title".
    ///
    /// Java to port: `String getType() { return this.type; }`
    pub fn type_str(&self) -> &'static str {
        todo!("step 1: match self and return the type string")
    }

    /// Java/Spring: `getTableName()` — the DB table backing this fragment type.
    ///
    /// TODO(step 1): return the table name for each variant via `match self`.
    /// Expected values:
    ///   Character => "character_fragments",
    ///   CreditContentMakers => "credit_content_markers_fragments",
    ///   Credit => "credit_fragments", Movie => "movie_fragments",
    ///   Person => "person_fragments", Programme => "programme_fragments",
    ///   Rights => "rights_fragments", Season => "season_fragments",
    ///   Series => "series_fragments", Sport => "sport_fragments",
    ///   Taxonomy => "taxonomy_fragments", Title => "title_fragments".
    ///
    /// Java to port: `String getTableName() { return this.tableName; }`
    pub fn table_name(&self) -> &'static str {
        todo!("step 1: match self and return the table name")
    }
}

/// Java/Spring: the `Fragment` POJO. A `struct` with `pub` fields is the closest
/// Rust equivalent of a class with getters/setters (here we expose the fields
/// directly instead of writing accessors).
///
/// `#[serde(rename_all = "camelCase")]` keeps the JSON keys identical to the
/// Java field names (`messageType`, `messageTs`, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fragment {
    pub id: String,
    pub message_type: MessageType,
    pub fragment_type: FragmentType,
    pub message_ts: i64,
    pub fragment: String,
    pub status: FragmentStatus,
}

impl Fragment {
    /// Java/Spring: think of this as a constructor / factory method. A new
    /// fragment always starts life in `FragmentStatus::Queued`.
    ///
    /// TODO(step 2): build and return a `Fragment` from the given values,
    /// setting `status` to `FragmentStatus::Queued`.
    ///
    /// Java to port:
    /// ```java
    /// static Fragment of(String id, MessageType mt, FragmentType ft, long ts, String payload) {
    ///     Fragment f = new Fragment(id, mt, ft, ts, payload);
    ///     f.setStatus(FragmentStatus.QUEUED);   // always starts queued
    ///     return f;
    /// }
    /// ```
    pub fn new(
        id: String,
        message_type: MessageType,
        fragment_type: FragmentType,
        message_ts: i64,
        fragment: String,
    ) -> Self {
        todo!("step 2: construct a Fragment with status = Queued")
    }

    /// Java/Spring: like `new` but the caller supplies the `status`. The consumer
    /// uses this because the *dispatch outcome* (e.g. `Processed`,
    /// `DownstreamError`) is what we persist, mirroring how a consumer stores the
    /// dispatch result. `new` stays the "freshly queued" path used by the
    /// REST `POST`.
    ///
    /// Java to port:
    /// ```java
    /// static Fragment of(String id, MessageType mt, FragmentType ft, long ts,
    ///                    String payload, FragmentStatus status) {
    ///     return new Fragment(id, mt, ft, ts, payload, status);  // caller-supplied status
    /// }
    /// ```
    pub fn with_status(
        id: String,
        message_type: MessageType,
        fragment_type: FragmentType,
        message_ts: i64,
        fragment: String,
        status: FragmentStatus,
    ) -> Self {
        todo!("milestone A: construct a Fragment using the caller-provided status")
    }
}

/// Java/Spring: a request DTO (e.g. the `@RequestBody` of `POST /fragments`).
/// No `status` field — the server assigns it (always `Queued`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFragment {
    pub id: String,
    pub message_type: MessageType,
    pub fragment_type: FragmentType,
    pub message_ts: i64,
    pub fragment: String,
}
