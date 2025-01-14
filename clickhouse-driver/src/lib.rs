//! ## Clickhouse-driver
//! Asynchronous pure rust tokio-based  Clickhouse client library
//!
//! ## Quick start
//! add next lines in dependencies section of `Cargo.toml`
//!  ```toml
//!   clickhouse-driver = { version="0.1.0-alpha.1", path="../path_to_package/clickhouse-driver"}
//!   naive-cityhash = { version="0.2.0"}
//!   ```
//! ## Supported Clickhouse data types
//! * Date | DateTime | DateTime64- read/write
//! * (U)Int(8|16|32|64) - read/write
//! * Float32 | Float64 - read/write
//! * UUID - read/write
//! * String | FixedString- read/write
//! * Ipv4 | Ipv6 - read/write
//! * Nullable(*) - read/write
//! * Decimal - read/write
//! * Enum8, Enum16 - read/write
//!
//! * LowCardinality(String) - read
//!
//! ## Connection url
//! ```url
//! tcp://[username:password@]host.name[:port]/database?paramname=paramvalue&...
//! ```
//! for example
//! ```url
//! tcp://user:default@localhost/log?ping_timout=200ms&execute_timeout=5s&query_timeout=20s&pool_max=4&compression=lz4
//! ```
//! - default port: 9000
//! - default username: "default"
//! - default database: "default"
//!
#![recursion_limit = "128"]
#![allow(unknown_lints)]
extern crate byteorder;
extern crate chrono;
extern crate chrono_tz;

extern crate core;
#[macro_use]
extern crate futures;
#[cfg(lz4)]
extern crate clickhouse_driver_lz4a;
extern crate hostname;
extern crate log;
extern crate once_cell;
extern crate parking_lot;
#[cfg(test)]
extern crate rand;
extern crate thiserror;
extern crate tokio;
extern crate url;
extern crate uuid;

use once_cell::sync::Lazy;
use pool::options::Options;

#[cfg(not(target_endian = "little"))]
compile_error!("only little-endian platforms supported");

mod client;
mod compression;
mod errors;
mod pool;
pub mod prelude;
#[macro_use]
mod protocol;
mod sync;
mod types;

#[allow(dead_code)]
const MAX_STRING_LEN: usize = 64 * 1024;
/// Max number of rows in server block, 640K is default value
const MAX_BLOCK_SIZE: usize = 640 * 1024;
/// Max size of server block, bytes, 1M is default value
const MAX_BLOCK_SIZE_BYTES: usize = 10 * 1024 * 1024;

pub static CLIENT_NAME: &str = "Rust Native Driver";
pub const CLICK_HOUSE_REVISION: u64 = 54405;
pub const CLICK_HOUSE_DBMSVERSION_MAJOR: u64 = 1;
pub const CLICK_HOUSE_DBMSVERSION_MINOR: u64 = 1;

static HOSTNAME: Lazy<String> = Lazy::new(|| {
    hostname::get().map_or_else(
        |_orig| String::new(),
        |s| s.into_string().unwrap_or_default(),
    )
});
static DEF_OPTIONS: Lazy<Options> = Lazy::new(|| pool::options::Options::default());

pub fn description() -> String {
    format!(
        "{} {}.{}.{}",
        CLIENT_NAME,
        CLICK_HOUSE_DBMSVERSION_MAJOR,
        CLICK_HOUSE_DBMSVERSION_MINOR,
        CLICK_HOUSE_REVISION
    )
}

#[test]
fn test_encoder() {
    assert_eq!(description(), "Rust Native Driver 1.1.54405");
}
