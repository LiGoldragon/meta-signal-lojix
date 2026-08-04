use meta_signal_lojix::schema::lib::{
    DatabaseMarker, DeployHandle, DeployRequest, FrameBody, HostDeployment, Input, Output,
    SourceRevisionPolicy,
};

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(9),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(3),
    )
}

fn marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: 1.into(),
        state_digest: 1.into(),
    }
}

fn deploy_request() -> DeployRequest {
    DeployRequest::Host(HostDeployment {
        cluster_name: "fixture-cluster".to_string().into(),
        node_name: "fixture-node".to_string().into(),
        host_composition: signal_lojix::schema::lib::HostComposition::BaseHost,
        proposal_source: "/tmp/fixture-cluster.dotos".to_string().into(),
        flake_reference: "github:example/fixture?rev=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_string()
            .into(),
        deployment_transport: signal_lojix::schema::lib::DeploymentTransport {
            nix_store_uri: "ssh-ng://fixture-copy-a.invalid".to_string().into(),
            ssh_destination: "fixture-login-a@fixture-activate-a.invalid"
                .to_string()
                .into(),
        },
        deployment_input_mode: signal_lojix::schema::lib::DeploymentInputMode::Direct,
        deployment_output_selector: signal_lojix::schema::lib::DeploymentOutputSelector::new(
            signal_lojix::schema::lib::FlakeAttribute::new("checks.fixture-a"),
        ),
        activation_backend: signal_lojix::schema::lib::ActivationBackend::NixosSystemdBootV1,
        host_deploy_action: signal_lojix::schema::lib::HostDeployAction::Evaluate,
        source_revision_policy: SourceRevisionPolicy::ResolveAndRecord,
        optional_nix_builder_spec: None,
        extra_substituter_vector: Vec::new(),
    })
}

fn deploy_input() -> Input {
    Input::Deploy(deploy_request().into())
}

#[test]
fn default_build_round_trips_meta_request_without_dotos_text() {
    let input = deploy_input();
    let frame = input
        .clone()
        .encode_request_frame(exchange())
        .expect("encode request");
    let (decoded_exchange, decoded) =
        meta_signal_lojix::schema::lib::ContractMarker::decode_single_request(&frame)
            .expect("decode request");

    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, input);
}

#[test]
fn default_build_round_trips_meta_reply_without_dotos_text() {
    let output = Output::DeployAccepted(
        DeployHandle {
            deployment_identifier: 1.into(),
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output
        .clone()
        .encode_reply_frame(exchange())
        .expect("encode reply");
    let decoded =
        meta_signal_lojix::schema::lib::ContractMarker::decode_frame(&frame).expect("decode reply");

    assert_eq!(
        decoded.into_body(),
        FrameBody::Reply {
            exchange: exchange(),
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(output),
            )),
        },
    );
}
