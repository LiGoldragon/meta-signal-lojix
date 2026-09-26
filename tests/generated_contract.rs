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
        horizon_definition_option: Some(minimal_horizon_definition()),
    })
}

fn minimal_horizon_definition() -> horizon_lib::HorizonDefinition {
    horizon_lib::HorizonDefinition {
        horizon_configuration: horizon_lib::HorizonConfiguration {
            generic_nodes: vec![horizon_lib::NodeDefinition {
                node_name: "opencode-test".into(),
                node_variant: horizon_lib::NodeVariant::Live(horizon_lib::LiveDefinition {}),
                first_magnitude: horizon_lib::Magnitude::Min,
                second_magnitude: horizon_lib::Magnitude::Min,
                machine_definition: horizon_lib::MachineDefinition::Metal(
                    horizon_lib::Metal_Data {
                        architecture: horizon_lib::Architecture::X86_64,
                        hardware: horizon_lib::Hardware {
                            integer: 1,
                            model_name_option: None,
                            mother_board_option: None,
                            first_integer_option: None,
                            second_integer_option: None,
                            location_option: None,
                        },
                    },
                ),
                node_environment: horizon_lib::NodeEnvironment {
                    keyboard: horizon_lib::Keyboard::Qwerty,
                    compressed_swap_option: None,
                },
                node_network: horizon_lib::NodeNetwork {
                    link_local_ip_vector: vec![],
                    node_ip_option: None,
                    wireguard_pub_key_option: None,
                    wireguard_proxy_vector: vec![],
                    router_interfaces_option: Some(horizon_lib::RouterInterfaces {
                        first_interface: "enp1s0".into(),
                        second_interface: "wlp2s0".into(),
                        wlan_band: horizon_lib::WlanBand::FiveG,
                        integer: 36,
                        wlan_standard: horizon_lib::WlanStandard::Wifi6,
                        secret_reference_option: Some(horizon_lib::SecretReference {
                            secret_name: "router-wifi-password".into(),
                        }),
                        backup_wireless_option: None,
                        country_code: "MX".into(),
                    }),
                },
                node_keys: horizon_lib::NodeKeys {
                    ssh_pub_key: "ssh-ed25519 AAAAfixture".into(),
                    nix_pub_key_option: None,
                    yggdrasil_key_option: None,
                },
                boolean_option: None,
                capabilities: vec![
                    horizon_lib::NodeCapability::OpenCodeTesting(horizon_lib::NoSettings {}),
                    horizon_lib::NodeCapability::Router(horizon_lib::NoSettings {}),
                    horizon_lib::NodeCapability::TailnetController(
                        horizon_lib::TailnetController_Data {
                            certificate_authority_option: Some(
                                "-----BEGIN CERTIFICATE-----fixture".into(),
                            ),
                            tls_certificate_reference: horizon_lib::TlsCertificateReference {
                                secret_name: "headscale-tls-certificate".into(),
                            },
                            tls_key_reference: horizon_lib::TlsKeyReference {
                                secret_name: "headscale-tls-key".into(),
                            },
                        },
                    ),
                    horizon_lib::NodeCapability::TailnetClient(horizon_lib::SecretReference {
                        secret_name: "tailnet-preauth-key".into(),
                    }),
                    horizon_lib::NodeCapability::UsbDownlink(horizon_lib::UsbDownlink_Data {
                        ipv4_cidr: "10.47.0.1/24".into(),
                    }),
                ],
                fixed_location_option: None,
            }],
            domain_configuration: horizon_lib::DomainConfiguration {
                string: "internal.invalid".into(),
                domain_name_vector: vec![],
            },
        },
        cluster_definition: horizon_lib::ClusterDefinition {
            cluster_name: "production.eu".into(),
            cluster_nodes: vec![],
            generic_node_names: vec![],
            users: vec![],
            domains: vec![],
            cluster_trust: horizon_lib::ClusterTrust {
                magnitude: horizon_lib::Magnitude::Zero,
                cluster_trust_entry_vector: vec![],
                node_trust_entry_vector: vec![],
                user_trust_entry_vector: vec![],
            },
        },
    }
}

#[cfg(feature = "datom")]
fn gold_horizon_definition_with_decimal_location() -> horizon_lib::HorizonDefinition {
    use datom_codec::Decimal;

    let mut definition = minimal_horizon_definition();
    let node = definition
        .horizon_configuration
        .generic_nodes
        .first_mut()
        .expect("OpenCodeTesting fixture node");
    node.fixed_location_option = Some(horizon_lib::FixedLocation {
        first_decimal: Decimal::try_from(19.4326).expect("finite latitude"),
        second_decimal: Decimal::try_from(-99.1332).expect("finite longitude"),
        third_decimal: Decimal::try_from(2_240.0).expect("finite altitude"),
        fourth_decimal: Decimal::try_from(3.5).expect("finite accuracy"),
    });
    definition
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
fn peer_bytes_restore_gold_opencode_testing_decimal_location() {
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
        horizon_definition_option: Some(gold_horizon_definition_with_decimal_location()),
    });
    let sent = query.signalize().expect("signalize Gold definition");
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore Gold definition"), query);
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore Gold definition from Datom");
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
        assert_eq!(
            received.restore().expect("restore deploy refusal"),
            response
        );
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
