mod rewrite;
///
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub mod scheme_permission;
pub(crate) use rewrite::Rewrite;
#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
pub(crate) use scheme_permission::SchemePermission;
