# INTENT — meta-signal-lojix

`meta-signal-lojix` is the meta policy signal contract for the
`lojix` deploy orchestrator. It owns the owner-only mutation
vocabulary: deploy submission and generation-retention changes.

This repo is a pure contract crate. It owns schema-derived typed
records, rkyv frames, NOTA codecs, and contract tests. It does not
own daemon runtime, actors, process effects, storage, or CLI behavior;
those live in `lojix`.

Shared deploy nouns come from the ordinary `signal-lojix` contract and
are imported once. This contract must not redefine shared identifiers,
markers, slots, node names, or deployment kinds.
