//! Exc: Abstractions for exchanges.

#![deny(missing_docs)]

/// Exchange.
pub mod exchange;

/// Transport.
pub mod transport;

/// Types.
pub mod types;

/// Errors.
pub mod error;

pub use error::ExchangeError;
pub use exchange::{subscribe_tickers::SubscribeTickersService, Exchange};
