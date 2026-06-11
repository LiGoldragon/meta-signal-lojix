//! Meta (owner-only policy) Signal contract for the lojix deploy orchestrator.
//!
//! Wire-only: the rkyv + NOTA codec and the signal-frame mail envelope for the
//! owner-only mutation surface — submitting a deploy and rewriting the GC-roots
//! retention policy (Deploy/Pin/Unpin/Retire). The peer-callable
//! read/observe/subscribe surface lives in the ordinary `signal-lojix`
//! contract.
//!
//! The shared record types are DEFINED ONCE in `signal-lojix`; this crate
//! cross-imports them via `signal-lojix:lib:TypeName`. The generated module
//! references them through `pub use signal_lojix::schema::lib::*` aliases, so
//! the path dependency on `signal-lojix` is load-bearing for both schema
//! generation and compilation.

pub mod schema;
