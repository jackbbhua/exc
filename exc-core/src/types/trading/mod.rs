/// Place: the order builder.
pub mod place;

/// Order.
pub mod order;

use std::{collections::BTreeMap, fmt, sync::Arc};

use futures::{future::BoxFuture, stream::BoxStream};
use indicator::{Tick, TickValue, Tickable};
pub use order::{Order, OrderId, OrderKind, OrderState, OrderStatus, OrderTrade, TimeInForce};
pub use place::Place;
use time::OffsetDateTime;

use crate::{ExchangeError, Request};

/// Options for order placement.
#[derive(Debug, Clone)]
pub struct PlaceOrderOptions {
    /// Instrument.
    instrument: String,
    /// Client id.
    client_id: Option<String>,
    /// Margin currency perferred to use.
    margin: Option<String>,
    /// Exchange-defined options.
    custom: BTreeMap<String, String>,
}

impl PlaceOrderOptions {
    /// Create a new options with the given instrument.
    pub fn new(inst: &str) -> Self {
        Self {
            instrument: inst.to_string(),
            client_id: None,
            margin: None,
            custom: BTreeMap::default(),
        }
    }

    /// Set the client id to place.
    pub fn with_client_id(&mut self, id: Option<&str>) -> &mut Self {
        self.client_id = id.map(|s| s.to_string());
        self
    }

    /// Set the margin currency preffered to use.
    /// # Warning
    /// It is up to the exchange to decide if this option applies,
    /// so please check the documents of the exchange you use.
    pub fn with_margin(&mut self, currency: &str) -> &mut Self {
        self.margin = Some(currency.to_string());
        self
    }

    /// Insert an exchange-defined custom option.
    pub fn insert<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.custom
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Get the instrument name to trade.
    pub fn instrument(&self) -> &str {
        &self.instrument
    }

    /// Get the client id to use.
    pub fn client_id(&self) -> Option<&str> {
        self.client_id.as_deref()
    }

    /// Get the margin currency perferred to use.
    pub fn margin(&self) -> Option<&str> {
        self.margin.as_deref()
    }

    /// Get the exchange-defined custom options.
    pub fn custom(&self) -> &BTreeMap<String, String> {
        &self.custom
    }
}

/// Place order.
#[derive(Debug, Clone)]
pub struct PlaceOrder {
    /// Place.
    pub place: Place,
    /// Options.
    pub opts: Arc<PlaceOrderOptions>,
}

impl PlaceOrder {
    /// Create a new request to place order.
    pub fn new(place: Place, opts: &PlaceOrderOptions) -> Self {
        Self {
            place,
            opts: Arc::new(opts.clone()),
        }
    }
}

/// Place order response.
#[derive(Clone)]
pub struct Placed {
    /// Order id.
    pub id: OrderId,
    /// The placed order.
    pub order: Option<Order>,
    /// Timestamp.
    pub ts: OffsetDateTime,
}

impl fmt::Debug for Placed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Placed")
            .field("ts", &self.ts.to_string())
            .field("id", &self.id.as_str())
            .field("order", &self.order)
            .finish()
    }
}

impl Request for PlaceOrder {
    type Response = BoxFuture<'static, Result<Placed, ExchangeError>>;
}

/// Cancel order.
#[derive(Debug, Clone)]
pub struct CancelOrder {
    /// Instrument.
    pub instrument: String,
    /// Id.
    pub id: OrderId,
}

/// Cancel order response.
#[derive(Clone)]
pub struct Cancelled {
    /// The placed order.
    pub order: Option<Order>,
    /// Timestamp.
    pub ts: OffsetDateTime,
}

impl fmt::Debug for Cancelled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cancelled")
            .field("ts", &self.ts.to_string())
            .field("order", &self.order)
            .finish()
    }
}

impl Request for CancelOrder {
    type Response = BoxFuture<'static, Result<Cancelled, ExchangeError>>;
}

/// Get order.
#[derive(Debug, Clone)]
pub struct GetOrder {
    /// Instrument.
    pub instrument: String,
    /// Id.
    pub id: OrderId,
}

/// Order update.
#[derive(Clone)]
pub struct OrderUpdate {
    /// Timestamp.
    pub ts: OffsetDateTime,
    /// Order.
    pub order: Order,
}

impl fmt::Debug for OrderUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OrderUpdate")
            .field("ts", &self.ts.to_string())
            .field("order", &self.order)
            .finish()
    }
}

impl Tickable for OrderUpdate {
    type Value = Order;

    fn tick(&self) -> Tick {
        Tick::new(self.ts)
    }

    fn value(&self) -> &Self::Value {
        &self.order
    }

    fn into_tick_value(self) -> TickValue<Self::Value> {
        TickValue::new(self.ts, self.order)
    }
}

impl Request for GetOrder {
    type Response = BoxFuture<'static, Result<OrderUpdate, ExchangeError>>;
}

/// Orders Stream.
pub type OrderStream = BoxStream<'static, Result<OrderUpdate, ExchangeError>>;

/// Subscribe to order updates.
#[derive(Debug, Clone)]
pub struct SubscribeOrders {
    /// Instrument.
    pub instrument: String,
}

impl Request for SubscribeOrders {
    type Response = OrderStream;
}
