use quick_error::quick_error;
use std::io;

quick_error! {
    /// The error used by the [Http] trait.
    #[derive(Debug)]
    #[allow(missing_docs)]
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

/// The return value of [Http::get()].
pub struct GetResponse<H, B> {
    /// The response headers.
    pub headers: H,
    /// The response body.
    pub body: B,
}

/// The return value of [Http::post()].
pub struct PostResponse<H, B, PB> {
    /// The body to post to the server as part of the request.
    ///
    /// **Note**: Implementations should drop the handle to avoid deadlocks.
    pub post_body: PB,
    /// The headers of the post response.
    pub headers: H,
    /// The body of the post response.
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

/// A trait to abstract the HTTP operations needed to power all git interactions: read via GET and write via POST.
#[allow(clippy::type_complexity)]
pub trait Http {
    /// A type providing headers line by line.
    type Headers: io::BufRead + Unpin;
    /// A type providing the response.
    type ResponseBody: io::BufRead;
    /// A type allowing to write the content to post.
    type PostBody: io::Write;

    /// Initiate a `GET` request to `url` provided the given `headers`.
    ///
    /// The `headers` are provided verbatim and include both the key as well as the value.
    fn get(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<GetResponse<Self::Headers, Self::ResponseBody>, Error>;

    /// Initiate a `POST` request to `url` providing with the given `headers`.
    ///
    /// The `headers` are provided verbatim and include both the key as well as the value.
    /// Note that the [`PostResponse`] contains the [`post_body`][PostResponse::post_body] field which implements [`std::io::Write`]
    /// and is expected to receive the body to post to the server. **It must be dropped** before reading the response
    /// to prevent deadlocks.
    fn post(
        &mut self,
        url: &str,
        headers: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<PostResponse<Self::Headers, Self::ResponseBody, Self::PostBody>, Error>;
}
