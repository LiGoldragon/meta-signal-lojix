use meta_signal_lojix::{PinRejectionReason, PinRequest, Query, RejectedPin, Response};
use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_lojix::DatabaseMarker;

fn configuration() -> signal_lojix::LojixNexusConfiguration {
    signal_lojix::LojixNexusConfiguration {
        ordinary_socket_path: "/run/lojix/ordinary.sock".into(),
        ordinary_socket_mode: 0o660,
        owner_socket_path: "/run/lojix/meta.sock".into(),
        owner_socket_mode: 0o600,
        state_directory_path: "/var/lib/lojix".into(),
        daemon_host: "deployment.host".into(),
        test_defaults_choice: signal_lojix::TestDefaultsChoice::NoTestDefaults,
    }
}

fn pin_query() -> Query {
    Query::Pin(PinRequest {
        cluster_name: "production.eu".into(),
        node_name: ".state/cache".into(),
        generation_identifier: 11,
        pin_label: "release.candidate".into(),
    })
}

fn deployment_with_secret_reference() -> Query {
    Query::Deploy(meta_signal_lojix::ActualizedDeploySubmission {
        deploy_submission: meta_signal_lojix::DeploySubmission::Host(
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
                deployment_input_mode: signal_lojix::DeploymentInputMode::Horizon,
                deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                    flake_attribute: "nixosConfigurations.node.config.system.build.toplevel".into(),
                },
                activation_backend: signal_lojix::ActivationBackend::NixosSystemdBootV1,
                host_deploy_action: signal_lojix::HostDeployAction::Realize,
                source_revision_policy: signal_lojix::SourceRevisionPolicy::ResolveAndRecord,
                nix_builder_spec_option: None,
                extra_substituter_vector: vec![],
            },
        ),
        cluster_proposal_wire_option: Some(usb_cluster_proposal()),
    })
}

fn usb_cluster_proposal() -> signal_lojix::ClusterProposalWire {
    use signal_lojix::horizon_wire_types::*;
    ClusterProposalWire {
        nodes: vec![NodeProposalEntryWire {
            name: NodeNameWire("ouranos".into()),
            proposal: NodeProposalWire {
                species: NodeSpeciesWire::Center,
                size: MagnitudeWire::Min,
                trust: MagnitudeWire::Min,
                machine: MachineWire {
                    species: MachineSpeciesWire::Metal,
                    arch: Some(ArchWire::X86_64),
                    cores: 4,
                    model: None,
                    mother_board: None,
                    super_node: None,
                    super_user: None,
                    chip_gen: None,
                    ram_gb: None,
                    disk_gb: None,
                    location: None,
                    super_nodes: vec![],
                },
                io: IoWire {
                    keyboard: KeyboardWire::Qwerty,
                    bootloader: BootloaderWire::Uefi,
                    disks: vec![],
                    swap_devices: vec![],
                    compressed_swap: None,
                },
                pub_keys: NodePubKeysWire {
                    ssh: SshPubKeyWire("ssh-ed25519 AAAAfixture".into()),
                    nix: None,
                    yggdrasil: None,
                },
                link_local_ips: vec![],
                node_ip: None,
                wireguard_pub_key: None,
                nordvpn: false,
                wifi_cert: false,
                wireguard_untrusted_proxies: vec![],
                wants_printing: false,
                wants_hw_video_accel: false,
                router_interfaces: None,
                online: None,
                services: vec![NodeServiceWire::UsbIpv4Gateway {
                    downstream: InterfaceWire("enp0s20f0u1c2".into()),
                    downstream_mac: MacAddressWire("00:0e:c6:33:4f:97".into()),
                    gateway: Ipv4CidrWire("10.44.0.1/24".into()),
                    uplink: InterfaceWire("enp0s31f6".into()),
                }],
            },
        }],
        users: vec![],
        domains: vec![],
        trust: ClusterTrustWire {
            cluster: MagnitudeWire::Zero,
            clusters: vec![],
            nodes: vec![],
            users: vec![],
        },
        domain_configuration: DomainConfigurationWire {
            internal_suffix: InternalDomainSuffixWire("criome".into()),
            public_cluster_domains: vec![],
        },
    }
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
fn peer_bytes_restore_usb_cluster_proposal() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Query::Deploy(meta_signal_lojix::ActualizedDeploySubmission {
        deploy_submission: meta_signal_lojix::DeploySubmission::Host(
            meta_signal_lojix::HostDeployment {
                cluster_name: "production.eu".into(),
                node_name: "opencode-test".into(),
                host_composition: signal_lojix::HostComposition::BaseHost,
                proposal_source: "proposal.datom".into(),
                secrets_input: signal_lojix::SecretsInput::NoSecrets,
                flake_reference: "github:example/system".into(),
                deployment_transport: signal_lojix::DeploymentTransport {
                    nix_store_uri: "ssh-ng://builder.invalid".into(),
                    ssh_destination: "root@node.invalid".into(),
                },
                deployment_input_mode: signal_lojix::DeploymentInputMode::Horizon,
                deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                    flake_attribute: "checks.x86_64-linux.contract".into(),
                },
                activation_backend: signal_lojix::ActivationBackend::NixosSystemdBootV1,
                host_deploy_action: signal_lojix::HostDeployAction::Realize,
                source_revision_policy: signal_lojix::SourceRevisionPolicy::RequireImmutable,
                nix_builder_spec_option: None,
                extra_substituter_vector: vec![],
            },
        ),
        cluster_proposal_wire_option: Some(usb_cluster_proposal()),
    });
    let sent = query.signalize().expect("signalize USB proposal");
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(
        received.restore().expect("restore USB proposal"),
        query
    );
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore USB proposal from Datom");
    assert_eq!(restored, query);
}

#[test]
fn meta_configure_and_reversal_cross_fresh_peer_bytes() {
    for query in [
        Query::Configure(configuration()),
        Query::ReverseConfiguration,
    ] {
        let sent = query.signalize().expect("signalize lifecycle query");
        let received = Signal::<Query>::from(sent.bytes().to_vec());
        assert_eq!(received.restore().expect("restore lifecycle query"), query);
    }

    let response = Response::ConfigurationReversed(signal_lojix::ConfigurationReceipt {
        lojix_nexus_configuration: configuration(),
        meta_configure_occurred: false,
    });
    let sent = response.signalize().expect("signalize reversal receipt");
    let received = Signal::<Response>::from(sent.bytes().to_vec());
    assert_eq!(
        received.restore().expect("restore reversal receipt"),
        response
    );
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

#[cfg(feature = "datom")]
#[test]
fn client_query_keeps_the_authored_pre_actualization_shape() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let wire = deployment_with_secret_reference();
    let Query::Deploy(actualized) = wire else {
        unreachable!()
    };
    let client = meta_signal_lojix::ClientQuery::Deploy(actualized.deploy_submission);
    let rendered = client.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<meta_signal_lojix::ClientQuery>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore client deployment");
    assert_eq!(restored, client);

    for client in [
        meta_signal_lojix::ClientQuery::Configure(configuration()),
        meta_signal_lojix::ClientQuery::ReverseConfiguration,
    ] {
        let rendered = client.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<meta_signal_lojix::ClientQuery>::from(rendered)
            .actualize(&mut Budget {
                remaining: 4_096,
                reader: ReaderBudget { remaining: 4_096 },
                depth: 0,
                maximum_depth: 256,
            })
            .expect("restore client configuration transition");
        assert_eq!(restored, client);
    }
}

#[test]
fn a_deploy_refusal_that_names_no_deployment_crosses_peer_bytes() {
    use meta_signal_lojix::{DeployRefusalReason, RefusedDeploy};

    for deploy_refusal_reason in [
        DeployRefusalReason::ContinuationBudgetExhausted,
        DeployRefusalReason::NoCorrelatedDeployment,
        DeployRefusalReason::DurableWriteFailed,
    ] {
        let response = Response::DeployRefused(RefusedDeploy {
            deploy_refusal_reason,
            database_marker: DatabaseMarker {
                commit_sequence: 7,
                state_digest: 3,
            },
        });
        let sent = response.signalize().expect("signalize deploy refusal");
        let received = Signal::<Response>::from(sent.bytes().to_vec());
        assert_eq!(received.restore().expect("restore deploy refusal"), response);
    }
}

#[cfg(feature = "datom")]
#[test]
fn a_deploy_refusal_renders_its_reason_in_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use meta_signal_lojix::{DeployRefusalReason, RefusedDeploy};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let response = Response::DeployRefused(RefusedDeploy {
        deploy_refusal_reason: DeployRefusalReason::NoCorrelatedDeployment,
        database_marker: DatabaseMarker {
            commit_sequence: 7,
            state_digest: 3,
        },
    });
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    assert!(
        rendered.contains("DeployRefused") && rendered.contains("NoCorrelatedDeployment"),
        "an uncorrelated refusal names itself: {rendered}"
    );
    let restored = Potential::<Response>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore deploy refusal");
    assert_eq!(restored, response);
}
