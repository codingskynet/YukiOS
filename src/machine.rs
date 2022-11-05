#[cfg(all(feature = "rv64", feature = "virt"))]
#[path = "machine/rv64/virt/mod.rs"]
mod hw;

pub use hw::*;
