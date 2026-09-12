# meta-signal-lojix architecture

`ethos/signal.ethos` is the structural authority and imports shared ordinary
Lojix types from `signal-lojix`. Ethos Zero generates all contract types.

`Signalizable`, `ByteViewable`, and `Restorable` own serialization behavior
and live in `signal`, implemented once for every contract. `Signal<T>`
contains raw portable archive bytes; `signal` also owns the stream length
framing. Datom derives are client-only through the optional `datom` feature.
No compatibility aliases, old envelopes, or alternate readers are retained.
