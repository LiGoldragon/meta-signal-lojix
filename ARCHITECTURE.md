# meta-signal-lojix architecture

`meta-signal-lojix` owns the owner Lojix Interface and its current Rust runtime
projection. The Interface is the authority boundary; Rust and the present
binary frame substrate are consumers rather than permanent assumptions in its
schema.

## Source and imported authority

`schema/lib.schema` is the canonical `Interface.{1 0 0}` document. Imports are
explicitly drawn from `signal_lojix:lib`. Nexus, Sema, and role sections are
empty; all owner-local structure lives in Types under `OwnerRequest` and
`OwnerReply`.

The ordinary contract owns every imported type once. Cargo's
`ethos-source-dir` metadata locates the exact producer source directory. The
owner build reads `lib.schema` there and requires it to equal the canonical
source compiled into the pinned `signal-lojix`. Imported textual metadata,
opaque identities, canonical order, and Rust type paths come from that
producer's authority manifest. There is no copied schema model or second alias
policy.

`src/bootstrap_manifest.rs` owns only this Interface's authority identity and
local declaration seats. The build assembles and verifies the complete source,
then `schema-rust` writes `src/schema/lib/generated.rs` with encoded local and
imported object coordinates. Ordinary builds check exact freshness; updates
require `META_SIGNAL_LOJIX_UPDATE_INTERFACE_ARTIFACTS=1`.

The build publishes this repository's explicit `schema/` directory through the
same metadata protocol for its consumers.

## Current-stage behavior

The bootstrap file kind does not yet express operational roles or wire
behavior. `src/schema/lib/behavior.rs` supplies that missing layer by hand:

- local structural behavior extending the ordinary producer's structural
  representation for imported values;
- readable Dotos heads for human, agent, harness, and GUI surfaces;
- owner request and reply role seating;
- the allocated `signal-frame` boundary.

The handwritten layer does not mint structural types and does not reinterpret
empty role sections. Its request heads are `Deploy`, `Pin`, `Unpin`, `Retire`,
and `Test`; reply heads name their typed outcomes and rejections.

## Dependency boundary

Normal runtime dependencies contain `signal-lojix`, `signal-frame`, `rkyv`, and
`thiserror`, with Dotos behind `dotos-text`. Authority assembly, translation,
and Rust projection crates are build-only. This repository owns no process,
transport selection, deployment execution, storage, or operating-system
policy.

Structural change begins in the Interface and its producer-owned authority
state, then regenerates the encoded Rust projection. Wire-breaking changes
allocate a new wire revision and crate version. Alternate readers, permissive
fallbacks, and compatibility aliases are not retained.
