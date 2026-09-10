# meta-signal-lojix

The privileged Signal contract for the Lojix deployment Nexus. The authored
contract is `ethos/signal.ethos`; Ethos Zero generates its `Query`, `Response`,
and named payload types and the build rejects stale generated Rust.

The default crate provides portable rkyv `Signal<T>` bytes without Datom. The
optional `datom` feature enables the final text chain for the privileged CLI.
Transport framing and authorization remain runtime responsibilities.
