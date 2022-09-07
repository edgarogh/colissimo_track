#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An illegal parcel ID was given, which caused an error while building the request
    #[error("invalid shipping id format")]
    IllegalParcelId,

    /// The request couldn't be sent
    #[error("http error: {0}")]
    Request(#[from] hyper::Error),

    /// The server sent an unexpected response
    #[error("malformed response")]
    Response,

    /// The server sent an expected response, but signals an error in this response
    #[error("the server sent an error message that was understood: {0:?}")]
    Server(String),
}
