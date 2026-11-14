# meta-signal-lojix

The privileged Signal contract for the Lojix deployment Nexus. The authored
contract is `ethos/signal.ethos`; Ethos Zero 28 generates its `Input` and
`Output` roots, the named payload types, and `Answering`, one capability per
input, which a Nexus answers through `input.answer(&nexus)`. The build rejects
stale generated Rust.

The default crate provides portable rkyv `Signal<T>` bytes without Datom. The
optional `datom` feature enables the final text chain for the privileged CLI.
Transport framing and authorization remain runtime responsibilities.
