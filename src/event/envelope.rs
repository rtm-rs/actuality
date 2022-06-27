use crate::aggregate::Aggregate;

use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

use async_trait::async_trait;

/// `EventEnvelope` is a data structure that encapsulates an event with its pertinent
/// information.
/// All of the associated data will be transported and persisted together and will be available
/// for queries.
///
/// Within any system an event must be unique based on the compound key composed of its:
/// - [`aggregate_type`](https://docs.rs/cqrs-es/latest/cqrs_es/trait.Aggregate.html#tymethod.aggregate_type)
/// - `aggregate_id`
/// - `sequence`
///
/// Thus an `EventEnvelope` provides a uniqueness value along with an event `payload` and
/// `metadata`.
#[derive(Debug)]
pub struct Envelope<A>
where
    A: Aggregate,
{
    /// The id of the aggregate instance.
    pub aggregate_id: String,
    /// The sequence number for an aggregate instance.
    pub sequence: usize,
    /// The event payload with all business information.
    pub payload: A::Event,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    pub metadata: HashMap<String, String>,
}
