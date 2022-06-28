// Sources:
// - https://github.com/serverlesstechnology/cqrs/blob/master/src/mem_store.rs
// - https://github.com/Joatin/eventific/blob/master/eventific/src/store/memory_store.rs
//
pub mod memory_store;

use async_trait::async_trait;
use std::collections::HashMap;

use crate::aggregate::Aggregate;
use crate::aggregate::context::AggregateContext;
use crate::event::EventEnvelope;
use crate::AggregateError;

#[derive(Clone, Debug)]
pub struct StoreContext {
    pub service_name: String,
}

/// The abstract central source for loading past events and committing new events.
#[async_trait]
pub trait EventStore<A>: Send + Sync
where
    A: Aggregate,
{
    /// Provides the current state of an aggregate along with surrounding context.
    /// This is used by the [Cqrs](struct.Cqrs.html) when loading
    /// an aggregate in order to handle incoming commands.
    type AC: AggregateContext<A>;

    /// Called as part of the setup process
    ///
    /// All your setup such as setting up connection pools, verify that targets
    /// exists, etc., should be implemented here.
    async fn init(&mut self, _context: StoreContext) -> Result<(), AggregateError<A::Error>> {
        println!("Setting up a new MemoryStore");
        println!("The MemoryStore does not persist events longer than the lifetime of the process. It is recommended that you set up a more accurate store.");
        Ok(())
    }

    /// Load all events for a particular `aggregate_id`
    async fn load_events(
        &self,
        aggregate_id: &str,
    ) -> Result<Vec<EventEnvelope<A>>, AggregateError<A::Error>>;
    /// Load aggregate at current state
    async fn load_aggregate(
        &self,
        aggregate_id: &str,
    ) -> Result<Self::AC, AggregateError<A::Error>>;
    /// Commit new events
    async fn commit(
        &self,
        events: Vec<A::Event>,
        context: Self::AC,
        metadata: HashMap<String, String>,
    ) -> Result<Vec<EventEnvelope<A>>, AggregateError<A::Error>>;
}
