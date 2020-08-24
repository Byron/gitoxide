use crate::{Protocol, Service};
use bstr::{BString, ByteVec};

pub fn connect(
    service: Service,
    version: Protocol,
    path: &[u8],
    virtual_host: Option<&(String, Option<u16>)>,
) -> BString {
    let mut out = bstr::BString::from(service.as_str());
    out.push(b' ');
    out.extend_from_slice(&path);
    out.push(0);
    if let Some((host, port)) = virtual_host {
        out.push_str("host=");
        out.extend_from_slice(host.as_bytes());
        if let Some(port) = port {
            out.push_byte(b':');
            out.push_str(&format!("{}", port));
        }
        out.push(0);
    }
    // We only send the version when needed, as otherwise a V2 server who is asked for V1 will respond with 'version 1'
    // as extra lines in the reply, which we don't want to handle. Especially since an old server will not respond with that
    // line (is what I assume, at least), so it's an optional part in the response to understand and handle. There is no value
    // in that, so let's help V2 servers to respond in a way that assumes V1.
    if version != Protocol::V1 {
        out.push(0);
        out.push_str(format!("version={}", version as usize));
        out.push(0);
    }
    out
}
