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
pub use crate::event::DomainEvent;
pub use crate::event::EventEnvelope;
pub use crate::query::View;
pub use crate::query::Query;
pub use crate::store::EventStore;
pub use crate::store::memory_store::MemoryStore;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
