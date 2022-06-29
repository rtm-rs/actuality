pub mod aggregate;
pub mod cqrs;
pub mod doc;
pub mod event;
pub mod persist;
pub mod query;
pub mod store;
pub mod test;

pub use crate::aggregate::Aggregate;
pub use crate::aggregate::context::AggregateContext;
pub use crate::aggregate::error::AggregateError;
pub use crate::cqrs::Cqrs;
pub use crate::event::DomainEvent;
pub use crate::event::EventEnvelope;
pub use crate::persist::event_stream::ReplayStream;
pub use crate::query::View;
pub use crate::query::Query;
pub use crate::store::EventStore;
pub use crate::store::memory_store::MemoryStore;

lazy_static::lazy_static!(
    static ref RTM_SYSTEM_ID: String = std::env::var("RTM_SYSTEM_ID").expect("RTM_SYSTEM_ID environment variable must be set.");
);

#[cfg(test)]
mod tests {

    #[test]
    fn system_id_from_env() {
        std::env::set_var("RTM_SYSTEM_ID", "1ead13j");
        assert_eq!(crate::RTM_SYSTEM_ID.clone().to_string(), "1ead13j")
    }
}
