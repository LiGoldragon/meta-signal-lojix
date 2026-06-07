# meta-signal-lojix — architecture

`meta-signal-lojix` is the owner-only policy signal contract for the
`lojix` deploy orchestrator.

The ordinary peer-callable contract is `signal-lojix`. This meta
contract carries deploy and retention mutations: `Deploy`, `Pin`,
`Unpin`, and `Retire`, with typed success and rejection replies.

Shared types are imported from `signal-lojix` by schema dependency:
cluster and node names, deployment and generation identifiers,
generation slots, deployment kinds, system actions, proposal and flake
references, and database markers. This keeps the two contracts on the
same wire vocabulary without duplicating nouns.

Non-ownership is strict: daemon runtime, actors, storage, process
effects, socket binding, authorization, and the CLI live in `lojix`.
This repo owns only the meta signal tree and generated wire codecs.
