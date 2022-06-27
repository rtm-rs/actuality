use crate::aggregate::Aggregate;

/// Returns the aggregate as well as the context around it.
/// This is used internally within an `EventStore` to persist an aggregate instance and events
/// with the correct context after it has been loaded and modified.
pub trait AggregateContext<A>
where
    A: Aggregate,
{
    /// The aggregate instance with all state loaded.
    fn aggregate(&self) -> &A;
}
