use exc_core::{
    types::{
        instrument::SubscribeInstruments, utils::Reconnect, CancelOrder, GetOrder, PlaceOrder,
        QueryLastCandles, SubscribeTickers,
    },
    Adaptor, ExchangeError, Request,
};

use crate::http::types::request::HttpRequest;
use crate::websocket::Request as WsRequest;

use super::OkxRequest;

impl Adaptor<SubscribeInstruments> for OkxRequest {
    fn from_request(req: SubscribeInstruments) -> Result<Self, ExchangeError> {
        let req = WsRequest::from_request(req)?;
        Ok(Self::Ws(req))
    }

    fn into_response(
        resp: Self::Response,
    ) -> Result<<SubscribeInstruments as Request>::Response, ExchangeError> {
        let res = resp.ws()?;
        <WsRequest as Adaptor<SubscribeInstruments>>::into_response(res)
    }
}

impl Adaptor<SubscribeTickers> for OkxRequest {
    fn from_request(req: SubscribeTickers) -> Result<Self, ExchangeError> {
        let req = WsRequest::from_request(req)?;
        Ok(Self::Ws(req))
    }

    fn into_response(
        resp: Self::Response,
    ) -> Result<<SubscribeTickers as Request>::Response, ExchangeError> {
        let res = resp.ws()?;
        <WsRequest as Adaptor<SubscribeTickers>>::into_response(res)
    }
}

impl Adaptor<QueryLastCandles> for OkxRequest {
    fn from_request(req: QueryLastCandles) -> Result<Self, ExchangeError> {
        let req = HttpRequest::from_request(req)?;
        Ok(Self::Http(req))
    }

    fn into_response(
        resp: Self::Response,
    ) -> Result<<QueryLastCandles as Request>::Response, ExchangeError> {
        let res = resp.http()?;
        <HttpRequest as Adaptor<QueryLastCandles>>::into_response(res)
    }
}

impl Adaptor<GetOrder> for OkxRequest {
    fn from_request(req: GetOrder) -> Result<Self, ExchangeError> {
        let req = HttpRequest::from_request(req)?;
        Ok(Self::Http(req))
    }

    fn into_response(
        resp: Self::Response,
    ) -> Result<<GetOrder as Request>::Response, ExchangeError> {
        let res = resp.http()?;
        <HttpRequest as Adaptor<GetOrder>>::into_response(res)
    }
}

impl Adaptor<PlaceOrder> for OkxRequest {
    fn from_request(req: PlaceOrder) -> Result<Self, ExchangeError> {
        let req = WsRequest::from_request(req)?;
        Ok(Self::Ws(req))
    }

    fn into_response(
        resp: Self::Response,
    ) -> Result<<PlaceOrder as Request>::Response, ExchangeError> {
        let res = resp.ws()?;
        <WsRequest as Adaptor<PlaceOrder>>::into_response(res)
    }
}

impl Adaptor<CancelOrder> for OkxRequest {
    fn from_request(req: CancelOrder) -> Result<Self, ExchangeError> {
        let req = WsRequest::from_request(req)?;
        Ok(Self::Ws(req))
    }

    fn into_response(
        resp: Self::Response,
    ) -> Result<<CancelOrder as Request>::Response, ExchangeError> {
        let res = resp.ws()?;
        <WsRequest as Adaptor<CancelOrder>>::into_response(res)
    }
}

impl Adaptor<Reconnect> for OkxRequest {
    fn from_request(_req: Reconnect) -> Result<Self, ExchangeError> {
        Ok(Self::Ws(WsRequest::reconnect()))
    }

    fn into_response(
        _resp: Self::Response,
    ) -> Result<<Reconnect as Request>::Response, ExchangeError> {
        Ok(())
    }
}
