use crate::aggregate::Aggregate;

/// `EventEnvelope` encapsulates an event with pertinent information.
///
/// Event identity and event data is persisted together and can be replayed.
///
/// Each event must be unique based on the compound key composed of its:
/// - `aggregate_id`
/// - `event_type`
/// - `sequence`
/// - `system_id`
///
/// The `EventEnvelope` provides a uniqueness value along with
/// the event `payload` and `metadata`.
#[derive(Debug)]
pub struct Envelope<A>
where
    A: Aggregate,
{
    /// The id of the aggregate instance.
    pub aggregate_id: String,
    /// The id of the system, e.g. a source control hash or version number.
    pub system_id: String,
    /// The sequence number for an aggregate instance.
    pub sequence: usize,
    /// The event type is one of: `create`, `update`, `delete`
    pub event_type: String,
    /// The event payload with all business information.
    pub payload: A::Event,
    /// Additional metadata for use in auditing, logging or debugging purposes.
    /// Example, relevant environment variable names and values.
    pub metadata: std::collections::HashMap<String, String>,
}
