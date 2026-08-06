//! Owner Lojix Interface.
//!
//! `schema/lib.schema` is a strict role-free bootstrap Interface. Its imports
//! resolve through the ordinary producer's published Ethos source directory,
//! and its Rust projection uses only authority-verified encoded coordinates.

pub mod bootstrap_manifest;
pub mod schema;

pub const META_LOJIX_INTERFACE_SOURCE: &str = include_str!("../schema/lib.schema");
pub const META_LOJIX_INTERFACE_RUST: &str = include_str!("schema/lib/generated.rs");
