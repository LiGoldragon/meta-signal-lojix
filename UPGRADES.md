# Upgrades

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
