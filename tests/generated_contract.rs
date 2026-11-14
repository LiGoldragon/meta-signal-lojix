use meta_signal_lojix::{Input, Output, PinRejectionReason, PinRequest, RejectedPin};
use signal::{ByteViewable, Restorable, Signal, Signalizable};
use signal_lojix::DatabaseMarker;

fn configuration() -> signal_lojix::LojixNexusConfiguration {
    signal_lojix::LojixNexusConfiguration {
        ordinary_socket_path: signal_lojix::OrdinarySocketPath("/run/lojix/ordinary.sock".into()),
        ordinary_socket_mode: signal_lojix::OrdinarySocketMode(0o660),
        owner_socket_path: signal_lojix::OwnerSocketPath("/run/lojix/meta.sock".into()),
        owner_socket_mode: signal_lojix::OwnerSocketMode(0o600),
        state_directory_path: signal_lojix::StateDirectoryPath("/var/lib/lojix".into()),
        daemon_host: signal_lojix::DaemonHost("deployment.host".into()),
        test_defaults_choice: signal_lojix::TestDefaultsChoice::NoTestDefaults,
    }
}

fn pin_query() -> Input {
    Input::Pin(PinRequest {
        cluster_name: signal_lojix::ClusterName("production.eu".into()),
        node_name: signal_lojix::NodeName(".state/cache".into()),
        generation_identifier: signal_lojix::GenerationIdentifier(11),
        pin_label: signal_lojix::PinLabel("release.candidate".into()),
    })
}

fn deployment_with_secret_reference() -> Input {
    Input::Deploy(meta_signal_lojix::ActualizedDeploySubmission {
        deploy_submission: meta_signal_lojix::DeploySubmission::Host(
            meta_signal_lojix::HostDeployment {
                cluster_name: signal_lojix::ClusterName("production.eu".into()),
                node_name: signal_lojix::NodeName("node-a".into()),
                host_composition: signal_lojix::HostComposition::BaseHost,
                proposal_source: signal_lojix::ProposalSource("proposal.datom".into()),
                secrets_input: signal_lojix::SecretsInput::SecretsDirectory(
                    signal_lojix::SecretsDirectory("fixture-secret-directory".into()),
                ),
                flake_reference: signal_lojix::FlakeReference("github:example/system".into()),
                deployment_transport: signal_lojix::DeploymentTransport {
                    nix_store_uri: signal_lojix::NixStoreUri("ssh-ng://builder.invalid".into()),
                    ssh_destination: signal_lojix::SshDestination("root@node.invalid".into()),
                },
                deployment_input_mode: signal_lojix::DeploymentInputMode::Horizon,
                deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                    flake_attribute: signal_lojix::FlakeAttribute(
                        "nixosConfigurations.node.config.system.build.toplevel".into(),
                    ),
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
            generic_nodes: horizon_lib::GenericNodes(vec![horizon_lib::NodeDefinition {
                node_name: horizon_lib::NodeName("opencode-test".into()),
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
                        first_interface: horizon_lib::Interface("enp1s0".into()),
                        second_interface: horizon_lib::Interface("wlp2s0".into()),
                        wlan_band: horizon_lib::WlanBand::FiveG,
                        integer: 36,
                        wlan_standard: horizon_lib::WlanStandard::Wifi6,
                        secret_reference_option: Some(horizon_lib::SecretReference {
                            secret_name: horizon_lib::SecretName("router-wifi-password".into()),
                        }),
                        backup_wireless_option: None,
                        country_code: horizon_lib::CountryCode("MX".into()),
                    }),
                },
                node_keys: horizon_lib::NodeKeys {
                    ssh_pub_key: horizon_lib::SshPubKey("ssh-ed25519 AAAAfixture".into()),
                    nix_pub_key_option: None,
                    yggdrasil_key_option: None,
                },
                boolean_option: None,
                capabilities: horizon_lib::Capabilities(vec![
                    horizon_lib::NodeCapability::OpenCodeTesting(horizon_lib::UserName(
                        "li".into(),
                    )),
                    horizon_lib::NodeCapability::Router(horizon_lib::NoSettings {}),
                    horizon_lib::NodeCapability::TailnetController(
                        horizon_lib::TailnetController_Data {
                            certificate_authority_option: Some(horizon_lib::CertificateAuthority(
                                "-----BEGIN CERTIFICATE-----fixture".into(),
                            )),
                            tls_certificate_reference: horizon_lib::TlsCertificateReference {
                                secret_name: horizon_lib::SecretName(
                                    "headscale-tls-certificate".into(),
                                ),
                            },
                            tls_key_reference: horizon_lib::TlsKeyReference {
                                secret_name: horizon_lib::SecretName("headscale-tls-key".into()),
                            },
                        },
                    ),
                    horizon_lib::NodeCapability::TailnetClient(horizon_lib::SecretReference {
                        secret_name: horizon_lib::SecretName("tailnet-preauth-key".into()),
                    }),
                    horizon_lib::NodeCapability::UsbDownlink(horizon_lib::UsbDownlink_Data {
                        ipv4_cidr: horizon_lib::Ipv4Cidr("10.47.0.1/24".into()),
                    }),
                ]),
                fixed_location_option: None,
            }]),
            domain_configuration: horizon_lib::DomainConfiguration {
                string: "internal.invalid".into(),
                domain_name_vector: vec![],
            },
        },
        cluster_definition: horizon_lib::ClusterDefinition {
            cluster_name: horizon_lib::ClusterName("production.eu".into()),
            cluster_nodes: horizon_lib::ClusterNodes(vec![]),
            generic_node_names: horizon_lib::GenericNodeNames(vec![]),
            users: horizon_lib::Users(vec![]),
            domains: horizon_lib::Domains(vec![]),
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
        .0
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
fn peer_bytes_restore_typed_input_and_output() {
    let query = pin_query();
    let sent = query.signalize().expect("signalize query");
    assert!(!sent.bytes().is_empty());
    let received = Signal::<Input>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore query"), query);

    let response = Output::PinRejected(RejectedPin {
        pin_rejection_reason: PinRejectionReason::PinLabelInUse,
        database_marker: DatabaseMarker {
            commit_sequence: signal_lojix::CommitSequence(7),
            state_digest: signal_lojix::StateDigest(13),
        },
    });
    let sent = response.signalize().expect("signalize response");
    let received = Signal::<Output>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore response"), response);
}

#[test]
fn peer_bytes_preserve_nonempty_secret_reference() {
    let query = deployment_with_secret_reference();
    let sent = query.signalize().expect("signalize deployment query");
    let received = Signal::<Input>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore deployment query"), query);
}

#[cfg(feature = "datom")]
#[test]
fn peer_bytes_restore_gold_opencode_testing_decimal_location() {
    assert_deployment_crosses_archive_and_datom(gold_horizon_definition_with_decimal_location());
}

#[cfg(feature = "datom")]
#[test]
fn a_user_key_carrying_agent_stack_and_stable_codex_crosses_archive_and_datom() {
    let mut definition = minimal_horizon_definition();
    definition.cluster_definition.users = horizon_lib::Users(vec![horizon_lib::UserDefinition {
        user_name: horizon_lib::UserName("li".into()),
        user_role: horizon_lib::UserRole::Code,
        magnitude: horizon_lib::Magnitude::Max,
        keyboard: horizon_lib::Keyboard::Colemak,
        style: horizon_lib::Style::Emacs,
        github_id_option: Some(horizon_lib::GithubId("li".into())),
        boolean_option: Some(true),
        user_pub_key_vector: vec![horizon_lib::UserPubKey {
            node_name: horizon_lib::NodeName("opencode-test".into()),
            ssh_pub_key: horizon_lib::SshPubKey("AAAAuser".into()),
            keygrip: horizon_lib::Keygrip("keygrip".into()),
            user_capability_vector: vec![
                horizon_lib::UserCapability::AgentStack,
                horizon_lib::UserCapability::StableCodex,
            ],
        }],
        editor_option: Some(horizon_lib::Editor::Emacs),
        text_size_option: Some(horizon_lib::TextSize::Large),
    }]);
    assert_deployment_crosses_archive_and_datom(definition);
}

#[cfg(feature = "datom")]
#[test]
fn a_git_server_capability_crosses_archive_and_datom() {
    let mut definition = minimal_horizon_definition();
    definition.horizon_configuration.generic_nodes.0[0]
        .capabilities
        .0
        .push(horizon_lib::NodeCapability::GitServer(
            horizon_lib::NoSettings {},
        ));
    assert_deployment_crosses_archive_and_datom(definition);
}

#[cfg(feature = "datom")]
fn assert_deployment_crosses_archive_and_datom(definition: horizon_lib::HorizonDefinition) {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Input::Deploy(meta_signal_lojix::ActualizedDeploySubmission {
        deploy_submission: meta_signal_lojix::DeploySubmission::Host(
            meta_signal_lojix::HostDeployment {
                cluster_name: signal_lojix::ClusterName("production.eu".into()),
                node_name: signal_lojix::NodeName("opencode-test".into()),
                host_composition: signal_lojix::HostComposition::BaseHost,
                proposal_source: signal_lojix::ProposalSource("proposal.datom".into()),
                secrets_input: signal_lojix::SecretsInput::NoSecrets,
                flake_reference: signal_lojix::FlakeReference("github:example/system".into()),
                deployment_transport: signal_lojix::DeploymentTransport {
                    nix_store_uri: signal_lojix::NixStoreUri("ssh-ng://builder.invalid".into()),
                    ssh_destination: signal_lojix::SshDestination("root@node.invalid".into()),
                },
                deployment_input_mode: signal_lojix::DeploymentInputMode::Horizon,
                deployment_output_selector: signal_lojix::DeploymentOutputSelector {
                    flake_attribute: signal_lojix::FlakeAttribute(
                        "checks.x86_64-linux.contract".into(),
                    ),
                },
                activation_backend: signal_lojix::ActivationBackend::NixosSystemdBootV1,
                host_deploy_action: signal_lojix::HostDeployAction::Realize,
                source_revision_policy: signal_lojix::SourceRevisionPolicy::RequireImmutable,
                nix_builder_spec_option: None,
                extra_substituter_vector: vec![],
            },
        ),
        horizon_definition_option: Some(definition),
    });
    let sent = query.signalize().expect("signalize Gold definition");
    let received = Signal::<Input>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore Gold definition"), query);
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Input>::from(rendered)
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
        Input::Configure(configuration()),
        Input::ReverseConfiguration,
    ] {
        let sent = query.signalize().expect("signalize lifecycle query");
        let received = Signal::<Input>::from(sent.bytes().to_vec());
        assert_eq!(received.restore().expect("restore lifecycle query"), query);
    }

    let response = Output::ConfigurationReversed(signal_lojix::ConfigurationReceipt {
        lojix_nexus_configuration: configuration(),
        meta_configure_occurred: signal_lojix::MetaConfigureOccurred(false),
    });
    let sent = response.signalize().expect("signalize reversal receipt");
    let received = Signal::<Output>::from(sent.bytes().to_vec());
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
    let mut pending = Potential::<Input>::from(rendered);
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
    let restored = Potential::<Input>::from(rendered)
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
    let Input::Deploy(actualized) = wire else {
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
        let response = Output::DeployRefused(RefusedDeploy {
            deploy_refusal_reason,
            database_marker: DatabaseMarker {
                commit_sequence: signal_lojix::CommitSequence(7),
                state_digest: signal_lojix::StateDigest(3),
            },
        });
        let sent = response.signalize().expect("signalize deploy refusal");
        let received = Signal::<Output>::from(sent.bytes().to_vec());
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

    let response = Output::DeployRefused(RefusedDeploy {
        deploy_refusal_reason: DeployRefusalReason::NoCorrelatedDeployment,
        database_marker: DatabaseMarker {
            commit_sequence: signal_lojix::CommitSequence(7),
            state_digest: signal_lojix::StateDigest(3),
        },
    });
    let rendered = response.clone().datomize(vec![]).protosize().textualize();
    assert!(
        rendered.contains("DeployRefused") && rendered.contains("NoCorrelatedDeployment"),
        "an uncorrelated refusal names itself: {rendered}"
    );
    let restored = Potential::<Output>::from(rendered)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("restore deploy refusal");
    assert_eq!(restored, response);
}

fn marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: signal_lojix::CommitSequence(3),
        state_digest: signal_lojix::StateDigest(5),
    }
}

fn receipt() -> signal_lojix::ConfigurationReceipt {
    signal_lojix::ConfigurationReceipt {
        lojix_nexus_configuration: configuration(),
        meta_configure_occurred: signal_lojix::MetaConfigureOccurred(true),
    }
}

/// A Nexus whose every capability succeeds with a fixed value, so the
/// generated dispatch shows which output each input is answered by.
struct AnsweringNexus;

impl meta_signal_lojix::Answering for AnsweringNexus {
    fn configure(
        &self,
        _configuration: &signal_lojix::LojixNexusConfiguration,
    ) -> Result<signal_lojix::ConfigurationReceipt, Output> {
        Ok(receipt())
    }

    fn reverse_configuration(&self) -> Result<signal_lojix::ConfigurationReceipt, Output> {
        Ok(receipt())
    }

    fn retire(
        &self,
        request: &meta_signal_lojix::RetireRequest,
    ) -> Result<meta_signal_lojix::AppliedRetire, Output> {
        Ok(meta_signal_lojix::AppliedRetire {
            generation_identifier: request.generation_identifier,
            generation_slot: signal_lojix::GenerationSlot::Recent,
            database_marker: marker(),
        })
    }

    fn pin(&self, request: &PinRequest) -> Result<meta_signal_lojix::AppliedPin, Output> {
        Ok(meta_signal_lojix::AppliedPin {
            generation_identifier: request.generation_identifier,
            pin_label: request.pin_label.clone(),
            first_generation_slot: signal_lojix::GenerationSlot::Recent,
            second_generation_slot: signal_lojix::GenerationSlot::Pinned,
            database_marker: marker(),
        })
    }

    fn deploy(
        &self,
        _submission: &meta_signal_lojix::ActualizedDeploySubmission,
    ) -> Result<meta_signal_lojix::DeployHandle, Output> {
        Ok(meta_signal_lojix::DeployHandle {
            deployment_identifier: signal_lojix::DeploymentIdentifier(1),
            database_marker: marker(),
        })
    }

    fn test(
        &self,
        _request: &meta_signal_lojix::TestRequest,
    ) -> Result<meta_signal_lojix::AcceptedTest, Output> {
        Ok(meta_signal_lojix::AcceptedTest {
            test_run_identifier: signal_lojix::TestRunIdentifier(1),
            database_marker: marker(),
        })
    }

    fn unpin(
        &self,
        request: &meta_signal_lojix::UnpinRequest,
    ) -> Result<meta_signal_lojix::AppliedUnpin, Output> {
        Ok(meta_signal_lojix::AppliedUnpin {
            generation_identifier: signal_lojix::GenerationIdentifier(11),
            pin_label: request.pin_label.clone(),
            first_generation_slot: signal_lojix::GenerationSlot::Pinned,
            second_generation_slot: signal_lojix::GenerationSlot::Recent,
            database_marker: marker(),
        })
    }
}

#[test]
fn each_input_is_answered_by_its_own_output() {
    use meta_signal_lojix::Answerable;

    let inputs = [
        Input::Configure(configuration()),
        Input::ReverseConfiguration,
        Input::Retire(meta_signal_lojix::RetireRequest {
            cluster_name: signal_lojix::ClusterName("production.eu".into()),
            node_name: signal_lojix::NodeName("node-a".into()),
            generation_identifier: signal_lojix::GenerationIdentifier(11),
        }),
        pin_query(),
        deployment_with_secret_reference(),
        Input::Test(meta_signal_lojix::TestRequest::Check(
            meta_signal_lojix::QuickCheck(vec![signal_lojix::NodeName("node-a".into())]),
        )),
        Input::Unpin(meta_signal_lojix::UnpinRequest {
            cluster_name: signal_lojix::ClusterName("production.eu".into()),
            node_name: signal_lojix::NodeName("node-a".into()),
            pin_label: signal_lojix::PinLabel("release.candidate".into()),
        }),
    ];
    let answered = inputs
        .iter()
        .map(|input| match input.answer(&AnsweringNexus) {
            Output::Configured(_) => "Configured",
            Output::ConfigurationReversed(_) => "ConfigurationReversed",
            Output::Retired(_) => "Retired",
            Output::Pinned(_) => "Pinned",
            Output::DeployAccepted(_) => "DeployAccepted",
            Output::Tested(_) => "Tested",
            Output::Unpinned(_) => "Unpinned",
            _ => "a refusal",
        })
        .collect::<Vec<_>>();
    assert_eq!(
        answered,
        [
            "Configured",
            "ConfigurationReversed",
            "Retired",
            "Pinned",
            "DeployAccepted",
            "Tested",
            "Unpinned",
        ]
    );
}

#[test]
fn a_store_refusal_is_an_output_of_the_contract() {
    let refusal = Output::Refused(signal_lojix::Refusal::Store("the commit failed".into()));
    let received = Signal::<Output>::from(
        refusal
            .signalize()
            .expect("signalize refusal")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("restore refusal"), refusal);
}

#[cfg(feature = "datom")]
#[test]
fn a_store_refusal_is_written_by_its_name() {
    use datom_codec::Datomizable;
    use protos::{Protosizable, Textualizable};

    let refusal = Output::Refused(signal_lojix::Refusal::Store("the commit failed".into()));
    assert_eq!(
        refusal.datomize(vec![]).protosize().textualize(),
        "Refused.Store.«the commit failed»"
    );
}

/// The deploy text CriomOS's harness-update sends, its shell variables filled
/// in, reads as it did before the scalar declarations became newtypes and
/// writes back unchanged.
#[cfg(feature = "datom")]
#[test]
fn the_deploy_text_criomos_sends_reads_and_writes_unchanged() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};

    let text = "Deploy.UserEnvironment.{ goldragon ouranos li /tmp/work/proposal.datom SecretsDirectory./tmp/work/cluster/secrets github:LiGoldragon/CriomOS-home?rev=0123456789abcdef { ssh-ng://li@ouranos li@ouranos } Horizon { homeConfigurations.li.activationPackage } HomeManagerNixProfileV1 ActivateNow RequireImmutable Some.@/etc/nix/machines [] }";
    let query = Potential::<meta_signal_lojix::ClientQuery>::from(text)
        .actualize(&mut Budget {
            remaining: 4_096,
            reader: ReaderBudget { remaining: 4_096 },
            depth: 0,
            maximum_depth: 256,
        })
        .expect("read the deploy text");
    let meta_signal_lojix::ClientQuery::Deploy(
        meta_signal_lojix::DeploySubmission::UserEnvironment(deployment),
    ) = &query
    else {
        panic!("a user-environment deploy: {query:?}");
    };
    assert_eq!(deployment.user_name, signal_lojix::UserName("li".into()));
    assert_eq!(
        deployment.user_environment_action,
        signal_lojix::UserEnvironmentAction::ActivateNow
    );
    assert_eq!(
        deployment.nix_builder_spec_option,
        Some(signal_lojix::NixBuilderSpec("@/etc/nix/machines".into()))
    );
    assert_eq!(query.datomize(vec![]).protosize().compact(), text);
}

/// DeploySubmission is stored inside Lojix's deploy job; it lists its
/// variants in the stored order, Host then UserEnvironment.
#[test]
fn a_deploy_submission_archives_in_the_stored_order() {
    let Input::Deploy(actualized) = deployment_with_secret_reference() else {
        unreachable!("the fixture is a deploy");
    };
    let meta_signal_lojix::DeploySubmission::Host(host) = actualized.deploy_submission else {
        unreachable!("the fixture is a host deploy");
    };
    let user = meta_signal_lojix::DeploySubmission::UserEnvironment(
        meta_signal_lojix::UserEnvironmentDeployment {
            cluster_name: host.cluster_name.clone(),
            node_name: host.node_name.clone(),
            user_name: signal_lojix::UserName("li".into()),
            proposal_source: host.proposal_source.clone(),
            secrets_input: host.secrets_input.clone(),
            flake_reference: host.flake_reference.clone(),
            deployment_transport: host.deployment_transport.clone(),
            deployment_input_mode: host.deployment_input_mode.clone(),
            deployment_output_selector: host.deployment_output_selector.clone(),
            activation_backend: signal_lojix::ActivationBackend::HomeManagerNixProfileV1,
            user_environment_action: signal_lojix::UserEnvironmentAction::Realize,
            source_revision_policy: host.source_revision_policy.clone(),
            nix_builder_spec_option: None,
            extra_substituter_vector: vec![],
        },
    );
    let host = meta_signal_lojix::DeploySubmission::Host(host);
    let tag = |submission: &meta_signal_lojix::DeploySubmission| {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(submission).expect("archive");
        bytes[bytes.len()
            - core::mem::size_of::<rkyv::Archived<meta_signal_lojix::DeploySubmission>>()]
    };
    assert_eq!([tag(&host), tag(&user)], [0, 1]);
}
