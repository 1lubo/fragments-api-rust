//! Inbound message model — what the consumer pulls off the "topic".
//!
//! Java/Spring parallel: this is the Rust stand-in for a Kafka
//! `ConsumerRecord<String, String>`. We only keep the bits the business logic
//! needs: the key (`id`), which `FragmentType` topic it came from, the broker
//! timestamp, and the payload.
//!
//! The payload is an `Option<String>`: `Some(json)` is a normal UPDATE, while
//! `None` is a *tombstone* — Kafka's convention for "this key was deleted".
//! That single `Option` is how we tell UPDATE from DELETE, exactly like a
//! typical Kafka consumer's `record.value() != null` check.

use crate::model::FragmentType;

/// Java/Spring: ≈ the `ConsumerRecord` handed to a `@KafkaListener` method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsumedMessage {
    /// The Kafka record key — our fragment id.
    pub id: String,
    /// Which fragment topic/type this came from.
    pub fragment_type: FragmentType,
    /// The broker timestamp (`message.timestamp()` in the Java consumer).
    pub message_ts: i64,
    /// The record value: `Some` = UPDATE payload, `None` = delete tombstone.
    pub value: Option<String>,
}

impl ConsumedMessage {
    /// Convenience constructor for an UPDATE (payload present).
    pub fn update(
        id: impl Into<String>,
        fragment_type: FragmentType,
        message_ts: i64,
        value: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            fragment_type,
            message_ts,
            value: Some(value.into()),
        }
    }

    /// Convenience constructor for a DELETE tombstone (`value == None`).
    pub fn delete(id: impl Into<String>, fragment_type: FragmentType, message_ts: i64) -> Self {
        Self {
            id: id.into(),
            fragment_type,
            message_ts,
            value: None,
        }
    }
}
