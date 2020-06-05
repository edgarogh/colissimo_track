#[derive(Debug)]
pub enum Error {
    /// An illegal parcel ID was given, which caused an error while building the request
    IllegalParcelId,

    /// The request couldn't be sent
    Request,

    /// The server sent an unexpected response
    Response,

    /// The server sent an expected response, but signals an error in this response
    Server(String),
}
