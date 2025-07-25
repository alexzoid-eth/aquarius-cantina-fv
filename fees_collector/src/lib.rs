#![no_std]

mod contract;
mod interface;
mod storage;
mod test;
mod test_permissions;
mod testutils;

pub use crate::contract::{FeesCollector, FeesCollectorClient};

#[cfg(feature = "certora")]
mod certora_specs;