use adapter::AbsAdapter;

pub trait ServiceDelegate {
    type Wrapped: ?Sized;

    /// Create an instance of the service trait from the given `Adapter`
    fn from_adapter<A>(adpt: ::std::sync::Arc<A>) -> ::std::sync::Arc<Self::Wrapped>
    where A: AbsAdapter;
}