//! Privileged Lojix Signal contract generated from current Ethos.

#[path = "generated.rs"]
mod generated;
pub use generated::*;

pub const META_LOJIX_SIGNAL_SOURCE: &str = include_str!("../ethos/lib.ethos");

/// The allocated privileged Lojix wire contract: seat 2, structural revision 3.
pub enum MetaLojixWire {}

impl signal_frame::WireContract for MetaLojixWire {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        signal_frame::ContractId::new(core::num::NonZeroU32::new(2).expect("meta seat is nonzero")),
        signal_frame::WireRevision::new(core::num::NonZeroU16::new(3).expect("meta revision is nonzero")),
    );
}
