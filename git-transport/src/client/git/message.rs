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
    if version != Protocol::V1 {
        out.push(0);
        out.push_str(format!("version={}", version as usize));
        out.push(0);
    }
    out
}
