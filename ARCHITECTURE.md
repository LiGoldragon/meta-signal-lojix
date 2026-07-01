# meta-signal-lojix — architecture

`meta-signal-lojix` is the owner-only policy signal contract for the
`lojix` deploy orchestrator.

## 0.5 · Direction

`meta-signal-lojix` is the owner-only policy contract for the `lojix` deploy orchestrator. It exists to isolate deploy and retention mutations — operations that can break the cluster — from the peer-callable read surface in `signal-lojix`. The two-contract authority split (Spirit `ssk2`, `vudl`) applies: the meta-naming rule ensures that meta ordering authority requires explicit owner access.

This is a pure contract crate: no daemon runtime, actors, storage, process effects, socket binding, or CLI behavior. All of those live in `lojix`.

The ordinary peer-callable contract is `signal-lojix`. This meta
contract carries deploy and retention mutations: `Deploy`, `Pin`,
`Unpin`, and `Retire`, with typed success and rejection replies.

Shared types are imported from `signal-lojix` by schema dependency:
cluster and node names, deployment and generation identifiers,
generation slots, generation artifacts, host compositions, deploy
actions, proposal and flake references, and database markers. This
keeps the two contracts on the same wire vocabulary without duplicating
nouns.

Non-ownership is strict: daemon runtime, actors, storage, process
effects, socket binding, authorization, and the CLI live in `lojix`.
This repo owns only the meta signal tree and generated wire codecs.
