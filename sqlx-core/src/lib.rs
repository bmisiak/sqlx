#![recursion_limit = "256"]
// When compiling with support for SQLite we must allow some unsafe code in order to
// interface with the inherently unsafe C module. This unsafe code is contained
// to the sqlite module.
#![cfg_attr(feature = "sqlite", deny(unsafe_code))]
#![cfg_attr(not(feature = "sqlite"), forbid(unsafe_code))]

#[macro_use]
pub mod error;

#[cfg(any(feature = "mysql", feature = "postgres"))]
#[macro_use]
mod io;

#[cfg(any(feature = "mysql", feature = "postgres"))]
mod cache;

mod connection;
mod database;
mod executor;
mod query;
mod query_as;
mod transaction;
mod url;

#[doc(hidden)]
pub mod runtime;

#[macro_use]
pub mod arguments;

#[doc(hidden)]
pub mod decode;

pub mod describe;
pub mod encode;
pub mod pool;
pub mod types;

#[macro_use]
pub mod row;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "sqlite")]
pub mod sqlite;

pub use database::Database;

#[doc(inline)]
pub use error::{Error, Result};

pub use connection::{Connect, Connection};
pub use executor::Executor;
pub use query::{query, Query};
pub use query_as::{query_as, QueryAs};
pub use transaction::Transaction;

#[doc(hidden)]
pub use query_as::query_as_mapped;

#[doc(inline)]
pub use pool::Pool;

#[doc(inline)]
pub use row::{FromRow, Row};

#[cfg(feature = "mysql")]
#[doc(inline)]
pub use mysql::MySql;

#[cfg(feature = "postgres")]
#[doc(inline)]
pub use postgres::Postgres;
