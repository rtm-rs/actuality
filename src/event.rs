pub mod envelope;

pub use envelope::Envelope as EventEnvelope;

use crate::aggregate::Aggregate;

// use std::collections::HashMap;
use std::fmt;
// use std::future::Future;
// use std::pin::Pin;

// use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

pub type SequenceNumber = i32;

pub trait Event {

}

/// [StoreEvent] `payload` contains the aggregate data, after the command
///  completes, alongside event metadata: `id`, `occured_on`, `sequence_number`.
pub struct StoreEvent<Event: Serialize + DeserializeOwned + Send + Sync> {
    /// Uniquely identifies an event among all events emitted from all aggregates.
    pub id: Uuid,
    /// The aggregate instance that emitted the event.
    pub aggregate_id: Uuid,
    /// The original, emitted, event.
    pub payload: Event,
    /// The timestamp of when the event is persisted.
    pub occurred_on: DateTime<Utc>,
    /// The sequence number of the event, within its specific aggregate instance.
    pub sequence_number: SequenceNumber,
}

/// A `DomainEvent` represents any business change in the state of an `Aggregate`. `DomainEvent`s
/// are immutable, and when
/// [event sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)
/// is used they are the single source of truth.
///
/// The name of a `DomainEvent` should always be in the past tense, e.g.,
/// - AdminPrivilegesGranted
/// - EmailAddressChanged
/// - DependencyAdded
///
/// To simplify serialization, an event should be an enum, and each variant should carry any
/// important information.
///
/// Though the `DomainEvent` trait only has a single function, the events must also derive a
/// number of standard traits.
/// - `Clone` - events may be cloned throughout the framework, particularly when applied to queries
/// - `Serialize` and `Deserialize` - required for persistence
/// - `PartialEq` and `Debug` - needed for effective testing
///
/// # Examples
/// ```
/// # use cqrs_es::doc::Customer;
/// # use cqrs_es::{Aggregate,DomainEvent};
/// # use serde::{Serialize,Deserialize};
/// #[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
/// pub enum CustomerEvent {
///     NameChanged{ changed_name: String },
///     EmailUpdated{ new_email: String },
/// }
/// ```
pub trait DomainEvent:
    Serialize + DeserializeOwned + Clone + PartialEq + fmt::Debug + Sync + Send
{
    /// A name specifying the event, used for event upcasting.
    fn event_type(&self) -> String;
    /// A version of the `event_type`, used for event upcasting.
    fn event_version(&self) -> String;
}

impl<A: Aggregate> Clone for EventEnvelope<A> {
    fn clone(&self) -> Self {
        EventEnvelope {
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            payload: self.payload.clone(),
            metadata: self.metadata.clone(),
        }
    }
}
