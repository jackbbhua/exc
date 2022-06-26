use futures::{Stream, TryStreamExt};

use crate::{
    http::response::{Data, RestResponse},
    websocket::{error::WsError, protocol::frame::StreamFrame, response::WsResponse},
    Error,
};

/// Binance response.
#[derive(Debug)]
pub enum Response {
    /// Http resposne.
    Http(RestResponse<Data>),
    /// Websocket response.
    Ws(WsResponse),
}

impl Response {
    /// As a stream of the given type.
    pub fn as_stream<T>(self) -> Option<impl Stream<Item = Result<T, Error>>>
    where
        T: TryFrom<StreamFrame, Error = WsError>,
    {
        match self {
            Self::Http(_) => None,
            Self::Ws(resp) => resp.as_stream().map(|stream| stream.map_err(Error::from)),
        }
    }
}