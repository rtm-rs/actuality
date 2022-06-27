pub mod aggregate;
pub mod event;
pub mod store;

pub use crate::aggregate::Aggregate;
pub use crate::aggregate::context::AggregateContext;
pub use crate::aggregate::error::AggregateError;
pub use crate::event::DomainEvent;
pub use crate::store::Store;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
