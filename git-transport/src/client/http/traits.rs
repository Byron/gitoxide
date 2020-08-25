use quick_error::quick_error;
use std::io;
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Detail(description: String) {
            display("{}", description)
        }
        PostBody(err: io::Error) {
            display("An IO error occurred while uploading the body of a POST request")
            from()
            source(err)
        }
    }
}

pub struct GetResponse<H, B> {
    pub headers: H,
    pub body: B,
}

pub struct PostResponse<H, B, PB> {
    /// **Note**: Implementations should drop the handle to avoid deadlocks
    pub post_body: PB,
    pub headers: H,
    pub body: B,
}

impl<A, B, C> From<PostResponse<A, B, C>> for GetResponse<A, B> {
    fn from(v: PostResponse<A, B, C>) -> Self {
        GetResponse {
            headers: v.headers,
            body: v.body,
        }
    }
}

#[allow(clippy::type_complexity)]
pub trait Http {
    type Headers: io::BufRead;
    type ResponseBody: io::BufRead;
    type PostBody: io::Write;

    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<GetResponse<Self::Headers, Self::ResponseBody>, Error>;
    fn post(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<PostResponse<Self::Headers, Self::ResponseBody, Self::PostBody>, Error>;
}
