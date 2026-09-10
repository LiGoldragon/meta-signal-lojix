use meta_signal_lojix::{
    ByteViewable, PinRejectionReason, PinRequest, Query, RejectedPin, Response, Restorable, Signal,
    Signalizable,
};
use signal_lojix::DatabaseMarker;

fn pin_query() -> Query {
    Query::Pin(PinRequest {
        cluster_name: "production.eu".into(),
        node_name: ".state/cache".into(),
        generation_identifier: 11,
        pin_label: "release.candidate".into(),
    })
}

fn deployment_with_secret_reference() -> Query {
    Query::Deploy(meta_signal_lojix::DeploySubmission::Host(
        meta_signal_lojix::HostDeployment {
            cluster_name: "production.eu".into(),
            node_name: "node-a".into(),
            host_composition: signal_lojix::HostComposition::BaseHost,
            proposal_source: "proposal.datom".into(),
            secrets_input: signal_lojix::SecretsInput::SecretsDirectory(
                "fixture-secret-directory".into(),
            ),
            flake_reference: "github:example/system".into(),
            deployment_transport: signal_lojix::DeploymentTransport {
                nix_store_uri: "ssh-ng://builder.invalid".into(),
                ssh_destination: "root@node.invalid".into(),
            },
            deployment_input_mode: signal_lojix::DeploymentInputMode::Direct,
            deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                flake_attribute: "nixosConfigurations.node.config.system.build.toplevel".into(),
            },
            activation_backend: signal_lojix::ActivationBackend::NixosSystemdBootV1,
            host_deploy_action: signal_lojix::HostDeployAction::Realize,
            source_revision_policy: signal_lojix::SourceRevisionPolicy::ResolveAndRecord,
            nix_builder_spec_option: None,
            extra_substituter_vector: vec![],
        },
    ))
}

#[test]
fn peer_bytes_restore_typed_query_and_response() {
    let query = pin_query();
    let sent = query.signalize().expect("signalize query");
    assert!(!sent.bytes().is_empty());
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore query"), query);

    let response = Response::PinRejected(RejectedPin {
        pin_rejection_reason: PinRejectionReason::PinLabelInUse,
        database_marker: DatabaseMarker {
            commit_sequence: 7,
            state_digest: 13,
        },
    });
    let sent = response.signalize().expect("signalize response");
    let received = Signal::<Response>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore response"), response);
}

#[test]
fn peer_bytes_preserve_nonempty_secret_reference() {
    let query = deployment_with_secret_reference();
    let sent = query.signalize().expect("signalize deployment query");
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore deployment query"), query);
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_named_privileged_query() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = pin_query();
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    assert!(rendered.contains("production.eu"));
    assert!(rendered.contains(".state/cache"));
    let mut pending = Potential::<Query>::from(rendered);
    let restored = pending
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore datom query");
    assert_eq!(restored, query);

    let deployment = deployment_with_secret_reference();
    let rendered = deployment.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore secret-reference deployment");
    assert_eq!(restored, deployment);
}
