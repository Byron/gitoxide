use git_transport::client;
use std::io;

/// This types sole purpose is to 'disable' the destructor on the Box provided in the `SetServiceResponse` type
/// by leaking the box. We provide a method to restore the box and drop it right away to not actually leak.
/// However, we do leak in error cases because we don't call the manual destructor then.
pub struct LeakedSetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: git_transport::Protocol,
    pub capabilities: client::Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<&'a mut dyn io::BufRead>,
}

impl<'a> From<client::SetServiceResponse<'a>> for LeakedSetServiceResponse<'a> {
    fn from(v: client::SetServiceResponse<'a>) -> Self {
        LeakedSetServiceResponse {
            actual_protocol: v.actual_protocol,
            capabilities: v.capabilities,
            refs: v.refs.map(Box::leak),
        }
    }
}

impl<'a> From<LeakedSetServiceResponse<'a>> for client::SetServiceResponse<'a> {
    fn from(v: LeakedSetServiceResponse<'a>) -> Self {
        client::SetServiceResponse {
            actual_protocol: v.actual_protocol,
            capabilities: v.capabilities,
            refs: v.refs.map(|b| {
                // SAFETY: We are bound to lifetime 'a, which is the lifetime of the thing pointed to by the trait object in the box.
                // Thus we can only drop the box while that thing is indeed valid, due to Rusts standard lifetime rules.
                // The box itself was leaked by us.
                // Note that this is only required because Drop scopes are the outer ones in the match, not the match arms, making them
                // too broad to be usable intuitively. I consider this a technical shortcoming and hope there is a way to resolve it.
                #[allow(unsafe_code)]
                unsafe {
                    Box::from_raw(b as *mut _)
                }
            }),
        }
    }
}
