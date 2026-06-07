# meta-signal-lojix — agent instructions

You MUST read `~/primary/repos/lore/AGENTS.md` — the canonical agent
contract.

Read `INTENT.md` before editing. This repo is a pure meta signal
contract under `triad-port/`.

- Keep this repo wire-only: schema, generated typed records, codecs,
  and contract witnesses.
- Do not add daemon runtime, actors, storage, process effects, CLI
  behavior, or deployment logic here.
- Shared types are imported from `signal-lojix`; do not duplicate
  shared deploy nouns locally.
