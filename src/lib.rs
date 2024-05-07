#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "cli")]
pub mod cli;
pub mod constants;
pub mod continents;
pub mod countries;
pub mod designated_market_area;
pub mod errors;
pub mod geo_ip_reader;
pub mod time_zones;
pub mod utils;
