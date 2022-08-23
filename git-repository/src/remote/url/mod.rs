mod rewrite;
///
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
pub mod scheme_permission;
pub(crate) use rewrite::Rewrite;
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client-async-std"))]
pub(crate) use scheme_permission::SchemePermission;
