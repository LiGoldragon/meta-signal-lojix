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
        cluster_name: "goldragon".to_string().into(),
        node_name: "ouranos".to_string().into(),
        host_composition: signal_lojix::schema::lib::HostComposition::BaseHost,
        proposal_source: "/git/github.com/LiGoldragon/goldragon/datom.dotos"
            .to_string()
            .into(),
        flake_reference: "github:LiGoldragon/CriOMOS/main".to_string().into(),
        host_deploy_action: signal_lojix::schema::lib::HostDeployAction::Evaluate,
        source_revision_policy: SourceRevisionPolicy::ResolveAndRecord,
        optional_builder: None,
        extra_substituter_vector: Vec::new(),
        optional_flake_attribute: None,
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
