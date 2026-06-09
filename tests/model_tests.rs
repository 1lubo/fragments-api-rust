//! Step 1 & 2 tests — enums + the `Fragment` constructor.
//!
//! Java/Spring parallel: these are your JUnit unit tests for the model layer.
//! Run just this file with:  `cargo test --test model_tests`

use fragments_api::model::{
    CreateFragment, Fragment, FragmentStatus, FragmentType, MessageType,
};

// ---- Step 1: FragmentType methods (match expressions) ---------------------

#[test]
fn fragment_type_type_str_matches_java() {
    assert_eq!(FragmentType::Character.type_str(), "character");
    assert_eq!(FragmentType::CreditContentMakers.type_str(), "creditcontentmarkers");
    assert_eq!(FragmentType::Credit.type_str(), "credit");
    assert_eq!(FragmentType::Movie.type_str(), "movie");
    assert_eq!(FragmentType::Person.type_str(), "person");
    assert_eq!(FragmentType::Programme.type_str(), "programme");
    assert_eq!(FragmentType::Rights.type_str(), "rights");
    assert_eq!(FragmentType::Season.type_str(), "season");
    assert_eq!(FragmentType::Series.type_str(), "series");
    assert_eq!(FragmentType::Sport.type_str(), "sport");
    assert_eq!(FragmentType::Taxonomy.type_str(), "taxonomy");
    assert_eq!(FragmentType::Title.type_str(), "title");
}

#[test]
fn fragment_type_table_name_matches_java() {
    assert_eq!(FragmentType::Character.table_name(), "character_fragments");
    assert_eq!(
        FragmentType::CreditContentMakers.table_name(),
        "credit_content_markers_fragments"
    );
    assert_eq!(FragmentType::Credit.table_name(), "credit_fragments");
    assert_eq!(FragmentType::Movie.table_name(), "movie_fragments");
    assert_eq!(FragmentType::Person.table_name(), "person_fragments");
    assert_eq!(FragmentType::Programme.table_name(), "programme_fragments");
    assert_eq!(FragmentType::Rights.table_name(), "rights_fragments");
    assert_eq!(FragmentType::Season.table_name(), "season_fragments");
    assert_eq!(FragmentType::Series.table_name(), "series_fragments");
    assert_eq!(FragmentType::Sport.table_name(), "sport_fragments");
    assert_eq!(FragmentType::Taxonomy.table_name(), "taxonomy_fragments");
    assert_eq!(FragmentType::Title.table_name(), "title_fragments");
}

// ---- Step 1: serde wire formats (already wired via derives) ---------------

#[test]
fn enums_serialize_to_java_constant_names() {
    assert_eq!(serde_json::to_string(&MessageType::Update).unwrap(), "\"UPDATE\"");
    assert_eq!(
        serde_json::to_string(&FragmentStatus::DownstreamConnectionError).unwrap(),
        "\"DOWNSTREAM_CONNECTION_ERROR\""
    );
    assert_eq!(serde_json::to_string(&FragmentType::Movie).unwrap(), "\"movie\"");
}

// ---- Step 2: Fragment::new constructor ------------------------------------

#[test]
fn fragment_new_defaults_to_queued() {
    let f = Fragment::new(
        "abc-123".to_string(),
        MessageType::Update,
        FragmentType::Title,
        1_700_000_000,
        "{\"some\":\"payload\"}".to_string(),
    );

    assert_eq!(f.id, "abc-123");
    assert_eq!(f.message_type, MessageType::Update);
    assert_eq!(f.fragment_type, FragmentType::Title);
    assert_eq!(f.message_ts, 1_700_000_000);
    assert_eq!(f.fragment, "{\"some\":\"payload\"}");
    assert_eq!(f.status, FragmentStatus::Queued);
}

#[test]
fn fragment_json_round_trips() {
    let f = Fragment::new(
        "id-1".to_string(),
        MessageType::Delete,
        FragmentType::Series,
        42,
        "payload".to_string(),
    );

    let json = serde_json::to_string(&f).unwrap();
    // camelCase keys, matching the Java field names.
    assert!(json.contains("\"messageType\":\"DELETE\""));
    assert!(json.contains("\"fragmentType\":\"series\""));
    assert!(json.contains("\"messageTs\":42"));

    let back: Fragment = serde_json::from_str(&json).unwrap();
    assert_eq!(back, f);
}

#[test]
fn create_fragment_deserializes_from_json() {
    let body = r#"{
        "id": "id-9",
        "messageType": "UPDATE",
        "fragmentType": "movie",
        "messageTs": 123,
        "fragment": "data"
    }"#;

    let dto: CreateFragment = serde_json::from_str(body).unwrap();
    assert_eq!(dto.id, "id-9");
    assert_eq!(dto.message_type, MessageType::Update);
    assert_eq!(dto.fragment_type, FragmentType::Movie);
    assert_eq!(dto.message_ts, 123);
    assert_eq!(dto.fragment, "data");
}
