# Upgrades

# 0.12.0 to 0.13.0

The build reads `ethos/signal.ethos` with ethos-zero 29.0.0
(`f23eacf338a90ce4a6dea011554724fa6488fcc9`, was `c2653dd8…` / 16.0.0), which
reads a Signal's first two sections as `Input` and `Output` and pairs them by
position. 29.0.0 generates the same Rust from this file as 28.0.0 (`970459d`). One wire break carries every change below, together with
`signal-lojix` 0.12.0; clients and daemon move together.

- `Query` is `Input` and `Response` is `Output`. The file also generates
  `Answering`, one capability per input, and `impl Answerable for Input`.
  `ClientQuery`, the privileged client's text root, keeps its name and shape.
- `Output` is ordered so the n-th output answers the n-th input:
  `Configured`, `ConfigurationReversed`, `Retired`, `Pinned`,
  `DeployAccepted`, `Tested`, `Unpinned`; then the rejections in input order,
  `ConfigurationRejected`, `RetireRejected`, `PinRejected`, `DeployRejected`,
  `DeployRefused`, `TestRejected`, `UnpinRejected`; then the reaction
  `DeployTerminal`; then `Refused`. `DeployAccepted` stays the acknowledgement
  of an admitted deployment.
- `Refused.Refusal` answers any input the Nexus store could not carry out,
  with the store's own words. `Refusal`, `[ Store.String ]`, is imported from
  `signal-lojix`; its text is `Refused.Store.«…»`.
- Every enumeration lists its variants in the order Lojix holds them (lojix
  `src/runtime_model.rs` at `da0ad8a`). `DeploySubmission`, stored in the
  deploy job, is `[ Host UserEnvironment ]`, which a test checks.
- `QuickCheck` is a newtype struct over `Vector<NodeName>` in place of a type
  alias, as is every scalar `signal-lojix` declares. Archive and Datom text are
  unchanged, through `datom-codec` 0.32.3
  (`eb792265a67b2c6865f9b8c9ed9d2e5ae5a7174c`, was `4dff16b4…` / 0.32.2); the
  deploy text CriomOS's harness-update sends reads and writes unchanged.
- `signal-lojix` moves to 0.12.0 (`8a776198db368c7f5d39e1615c3d3d319fc2cf5d`,
  was `bd5d250f…` / 0.11.0) and `horizon-lib` to 0.18.0
  (`825a7f09242380a7b3d2609b26e20647db46a8ce`, was `e723ce11…` / 0.17.1).
  `protos` and `signal` are unchanged. `Input` is unchanged in order.

The archived tags that move:

| enumeration | variant | was | now |
|---|---|---|---|
| Output | ConfigurationRejected | 1 | 7 |
| Output | ConfigurationReversed | 2 | 1 |
| Output | PinRejected | 3 | 9 |
| Output | DeployRejected | 4 | 10 |
| Output | DeployRefused | 5 | 11 |
| Output | DeployAccepted | 6 | 4 |
| Output | TestRejected | 7 | 12 |
| Output | Unpinned | 8 | 6 |
| Output | Tested | 9 | 5 |
| Output | UnpinRejected | 10 | 13 |
| Output | DeployTerminal | 11 | 14 |
| Output | Pinned | 12 | 3 |
| Output | RetireRejected | 13 | 8 |
| Output | Retired | 14 | 2 |
| Output | Refused | (new) | 15 |
| PinRejectionReason | PinSlotExhausted | 0 | 3 |
| PinRejectionReason | InternalError | 1 | 4 |
| PinRejectionReason | NodeUnknown | 2 | 1 |
| PinRejectionReason | PinLabelInUse | 3 | 2 |
| PinRejectionReason | GenerationUnknown | 4 | 0 |
| NodeSelection | All | 0 | 1 |
| NodeSelection | Nodes | 1 | 0 |
| RetireRejectionReason | NodeUnknown | 0 | 1 |
| RetireRejectionReason | GenerationUnknown | 1 | 0 |
| RetireRejectionReason | GenerationPinned | 2 | 3 |
| RetireRejectionReason | InternalError | 3 | 4 |
| RetireRejectionReason | GenerationActive | 4 | 2 |
| UnpinRejectionReason | GenerationNotPinned | 0 | 2 |
| UnpinRejectionReason | PinLabelUnknown | 1 | 0 |
| UnpinRejectionReason | InternalError | 2 | 3 |
| UnpinRejectionReason | NodeUnknown | 3 | 1 |
| DeploySubmission | UserEnvironment | 0 | 1 |
| DeploySubmission | Host | 1 | 0 |
| TestRejectionReason | SubstrateUnavailable | 0 | 6 |
| TestRejectionReason | NoTestDefaults | 1 | 4 |
| TestRejectionReason | ClusterUnknown | 2 | 0 |
| TestRejectionReason | LiveNotYetEnabled | 4 | 5 |
| TestRejectionReason | NodeUnknown | 5 | 1 |
| TestRejectionReason | VmHostNotDeclaredForNode | 6 | 2 |

# 0.11.1 to 0.12.0

`signal-lojix` moves from 0.10.1 (`ba4d4ac9fa0d189e78c912be8bb188b3b529c071`)
to 0.11.0 (`bd5d250f6923c85ee785f0e9bb4b4b8320a67c11`), whose `Response`
gains `DeploymentPhaseObserved.DeploymentPhaseEvent`. None of the types this
contract imports from `signal-lojix` change, and `src/generated/signal.rs` is
byte-identical; the number moves with the ordinary contract so that clients
and daemon take the two together.

# 0.11.0 to 0.11.1

`horizon-lib` moves from 0.17.0 (`187b028a1c1ec6c8926a618a3ca435440d077ae7`)
to 0.17.1 (`e723ce117104557a6eb88555008a68627adae313`), and `signal-lojix`
moves from 0.10.0 (`a6ccf3f4e125f9651bc17b2f54ca8135a8aadbfd`) to 0.10.1
(`ba4d4ac9fa0d189e78c912be8bb188b3b529c071`). Both are revisions with no type
changes: the `isGitServer` gate no longer requires the node's base public keys.
No meta-signal-lojix type changes, and `src/generated/signal.rs` is
byte-identical.

# 0.10.0 to 0.11.0

`horizon-lib` moves from 0.16.0 (`bd52627b24bf553de45eb639571c1127e44e7f1d`)
to 0.17.0 (`187b028a1c1ec6c8926a618a3ca435440d077ae7`), and `signal-lojix`
moves from 0.9.0 (`e192aea5955692946e9bf7643f41be9fb5db15e7`) to 0.10.0
(`a6ccf3f4e125f9651bc17b2f54ca8135a8aadbfd`).
`NodeCapability` gains `GitServer.NoSettings`, appended last:
`NodeCapability.[ … UsbDownlink.{ Ipv4Cidr } GitServer.NoSettings ]`.

Existing records decode unchanged; a cluster definition naming `GitServer` is
carried by any message holding a `HorizonDefinition`
(`ActualizedDeploySubmission` carries `Option<HorizonDefinition>`), so clients
and daemon move together. `signal`, `protos`, `datom-codec` and `ethos-zero`
are unchanged, and `src/generated/signal.rs` is byte-identical.

# 0.9.0 to 0.10.0

`horizon-lib` moves from 0.15.0 (`91abfd754fe09e296bb4de074204885b1e209b0d`)
to 0.16.0 (`bd52627b24bf553de45eb639571c1127e44e7f1d`), and `signal-lojix`
moves from 0.8.0 (`a55516571f7fa58aca92c973738d1788b6bd3604`) to 0.9.0
(`e192aea5955692946e9bf7643f41be9fb5db15e7`).
`NodeCapability::OpenCodeTesting` carries a `UserName`, the server's owner,
where it carried `NoSettings`; `UserCapability` gains `StableCodex`:
`UserCapability.[ AgentStack CodexCandidate StableCodex ]`.

Any message holding a `HorizonDefinition` changes archive layout because a
`NodeCapability` payload changed and a `UserCapability` variant was added;
`ActualizedDeploySubmission` carries `Option<HorizonDefinition>`. Clients and
daemon move together. `signal`, `protos`, `datom-codec` and `ethos-zero` are
unchanged, and `src/generated/signal.rs` is byte-identical.

# 8.0.0 to 0.9.0

The numbering rule changes: the major number is only for a full rewrite, and
the past majors were wrong. A change to a stored or archived layout is a
subversion. This contract is now numbered in the 0.x scheme; 0.9.0 is the next
subversion after 8.0.0, and the next full rewrite is 1.0.0.

`horizon-lib` moves from 0.14.0 (`2e09ebbdc725f64fd9ee102b5758bab85be217e1`)
to 0.15.0 (`91abfd754fe09e296bb4de074204885b1e209b0d`), and `signal-lojix`
moves from 7.0.0 (`0a83f2d6d89b473ce63ff10bf05982df314894ad`) to 0.8.0
(`a55516571f7fa58aca92c973738d1788b6bd3604`). `UserPubKey` gains a fourth
position, `Vector<UserCapability>`, with `UserCapability.[ AgentStack CodexCandidate ]`.

Any message holding a `HorizonDefinition` changes archive layout because
`UserPubKey` gained a position; `ActualizedDeploySubmission` carries
`Option<HorizonDefinition>`. Clients and daemon move together. `signal`,
`protos`, `datom-codec` and `ethos-zero` are unchanged, and
`src/generated/signal.rs` is byte-identical.

# 7.0.0 to 8.0.0

One step onto ethos-zero 16.0.0 and the 0.32.2 substrate:

- `signal` 3.0.2 (`8f9a0deb…`) to 8.0.0 (`f35460de930943ea1a8a972453ccf3a092a655b5`);
- `signal-lojix` 6.0.0 (`cd164896…`) to 7.0.0 (`0a83f2d6d89b473ce63ff10bf05982df314894ad`);
- `horizon-lib` 0.13.0 (`a3ddaf86…`) to 0.14.0 (`2e09ebbdc725f64fd9ee102b5758bab85be217e1`);
- `datom-codec` 0.31.0 (`09e2a9d5…`) to 0.32.2 (`4dff16b4f7412febc3b71aac8b49680cd20988cb`);
- `protos` 0.31.0 (`1febca78…`) to 0.32.2 (`15b41da8f2579e73ead59bc0c2b97529b8ac32d3`);
- `ethos-zero` 10.0.0 (`4bf73cae…`) to 16.0.0 (`c2653dd82adbdb1f1f2f654405c6620e0d06fd58`).

`src/generated/signal.rs` is regenerated (options as `std::option::Option`,
`#[rustfmt::skip]` on every item); no field, variant or archived layout of
this contract changed. What breaks is the trait identity under `datom`: every
type now implements datom-codec 0.32.2's `Datomizable` and `Composing`. Lojix
repins this revision together with signal-lojix 7.0.0, signal 8.0.0,
horizon-lib 0.14.0, datom-codec and protos 0.32.2 in one step; `cargo tree -d`
then shows one datom-codec.

# 6.0.0 to 7.0.0

`horizon-lib` moves from 0.12.0 (`ee8d6f8d27eb6e200504807971ffdd26aaca7ed1`)
to 0.13.0 (`a3ddaf8685b920093a2328b85ba350a04e11477a`), and `signal-lojix`
moves from 5.0.0 (`f7866bf013499503d21491bcdaadaf88ea6c810c`) to 6.0.0
(`cd164896311af9849e2ddf1cdbdd35b5feedcdd1`).

`ActualizedDeploySubmission` carries `Option<HorizonDefinition>`, and the
ordinary contract embeds it too. Horizon 0.13.0 changes its archived layout:
`NodeCapability::TailnetClient` carries a `SecretReference`,
`NodeCapability::TailnetController` carries the certificate authority and the
TLS certificate and key references, `NodeCapability` gains `UsbDownlink`, and
`RouterInterfaces` gains an eighth field, `country_code`.

Breaking: the rkyv archive of every message that can hold a
`HorizonDefinition` changes. Consumers repin and bump. `ethos-zero` and
`datom-codec` are unchanged, and the generated contract is byte-identical.
The contract fixture now carries a router with a country, a tailnet
controller, a tailnet client and a USB downlink.

# 5.1.1 to 6.0.0

`Answer` gains `DeployRefused.RefusedDeploy`, with
`RefusedDeploy.{ DeployRefusalReason DatabaseMarker }` and
`DeployRefusalReason.[ ContinuationBudgetExhausted NoCorrelatedDeployment
DurableWriteFailed ]`. `signal-lojix` moves to 5.0.0
(`4271b5ced31ea02f11f29b602301832e83cfe6c2`).

`RejectedDeploy.{ DeploymentRecord }` names the deployment it rejects, and it
is the only deploy refusal this contract had. Three refusals name no
deployment, because at the moment of refusal there is none to name: the
Nexus's continuation budget ran out, an effect or write completion arrived
with no correlated deployment cursor, or the durable write that would have
produced the record failed. Each of those was previously answered with a
fabricated record or not answered at all. `DeployRefused` is their honest
answer and carries the same `reason + marker` shape the four other meta
refusals (`RejectedPin`, `RejectedUnpin`, `RejectedRetire`, `RejectedTest`)
already carry.

`DeployRejected` is unchanged and keeps its meaning: a refusal that names the
deployment record it rejected.

Breaking: `Answer` gains a variant, so its rkyv archive changes. A client
matching on `Answer` adds the arm; a client that only submits sees nothing new
unless the Nexus refuses uncorrelated.

# 5.1.0 to 5.1.1

A repin only. The producer chain settles on its final heads: `protos` 0.30.1
(`171b21f65337983ab624b7b906397a4f1f92c5a3`), `datom-codec` 0.26.3
(`627db67f2655efd9f786864009955005fd8ab2ad`), `ethos-zero` 8.0.1
(`de3d9928b156f2e1a92d060b7817af201abfdbef`), `signal` 3.0.2
(`8f9a0deb701cebbea518679548df4a795affc918`), `horizon-lib` 0.10.1
(`40d04d2504fee619e9b2b2564b8a769a3a9d6049`), `signal-lojix` 4.1.1
(`5c94485c84d20d5b1496d867a2b40f2d908a02e3`).

No type in this contract changed, and `src/generated/signal.rs` regenerates
byte-identical under ethos-zero 8.0.1 — `build.rs` asserts it on every build.
`Cargo.lock` carries exactly one revision of each of our crates.

Consumers repin the revision and change nothing else.

# 5.0.0 to 5.1.0

`horizon-lib` moves to 0.10.0 (`a56330451934d682ae15612acd49924356ec0205`) and
`signal-lojix` to 4.1.0 (`d0f5c70d437add1df16055dcb760b7ef9a140ef0`). No type
in this crate changed; see horizon-rs UPGRADES.md for the trait rehoming a
consumer must import.

# 3.0.1 to 5.0.0

No type in this crate changed. `DeployTerminal` carries `signal-lojix`'s
`DeploymentRecord`, and that record's `DeploymentFailure` gained a third
field — `Option<FailureEvidence>`, the failed command, its exit code and the
bounded redacted detail it printed. `DeploymentTerminalReason` gained
`EvaluationFailed` and `BuildFailed`.

4.0.0 carried an earlier shape of that field, in which the failed command's
program and arguments were not optional; a stage failing without a subprocess
had nothing honest to put there. The command now sits in its own optional
`FailedCommand` inside the evidence. Pin
`signal-lojix` 4.0.0 (`4dfcc34e1164b125318b29e3f0794e145e94f855`) alongside
this release; see that crate's UPGRADES.md.

# 2.3.0 to 3.0.0

The Signal frame type and its three kinds left this crate. `Signal<T>`,
`Signalizable`, `ByteViewable`, and `Restorable<T>` were defined here, in a
copy byte-identical to the one in every other contract crate. They now live
once, generically, in the `signal` repository, and this crate depends on it.

There is no compatibility path. A consumer changes its imports:

```rust
-use meta_signal_lojix::{ByteViewable, Restorable, Signal, Signalizable, Query, Response};
+use signal::{ByteViewable, Restorable, Signal, Signalizable};
+use meta_signal_lojix::{Query, Response};
```

and adds the dependency:

```toml
signal = { git = "https://github.com/LiGoldragon/signal", rev = "626e407be520a7a12f39b1d06c56ec423f3b3d09" }
```

The behavior is unchanged: the same rkyv bytes, the same validation on
restore. `Signalizable` and `Restorable<T>` are blanket implementations now,
so every contract type has them without the crate writing anything.
