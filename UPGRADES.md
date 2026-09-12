# Upgrades

# 3.0.1 to 4.0.0

No type in this crate changed. `DeployTerminal` carries `signal-lojix`'s
`DeploymentRecord`, and that record's `DeploymentFailure` gained a third
field — `Option<FailureEvidence>`, the failed command, its exit code and the
bounded redacted detail it printed. `DeploymentTerminalReason` gained
`EvaluationFailed` and `BuildFailed`.

The owner wire therefore changes with the ordinary one. Pin
`signal-lojix` 3.0.0 (`f915541bf1a15e0f8851d4ba5d99d2a376fed45b`) alongside
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
