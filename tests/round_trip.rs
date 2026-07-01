#![cfg(feature = "nota-text")]

use meta_signal_lojix::schema::lib::{
    DatabaseMarker, DeployHandle, DeployRequest, HostDeployment, Input, Output, PinRequest,
    SourceRevisionPolicy,
};
use nota_next::{NotaDecode, NotaEncode, NotaSource};

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
        source: "/git/github.com/LiGoldragon/goldragon/datom.nota"
            .to_string()
            .into(),
        flake: "github:LiGoldragon/CriOMOS/main".to_string().into(),
        host_deploy_action: signal_lojix::schema::lib::HostDeployAction::Evaluate,
        source_revision_policy: SourceRevisionPolicy::ResolveAndRecord,
        builder: None,
        substituters: Vec::new(),
        build_attribute: None,
    })
}

fn deploy_input() -> Input {
    Input::Deploy(deploy_request().into())
}

fn pin_input() -> Input {
    Input::Pin(
        PinRequest {
            cluster_name: "goldragon".to_string().into(),
            node_name: "ouranos".to_string().into(),
            generation_identifier: 1.into(),
            pin_label: "known-good".to_string().into(),
        }
        .into(),
    )
}

fn deploy_accepted_output() -> Output {
    Output::DeployAccepted(
        DeployHandle {
            deployment_identifier: 1.into(),
            database_marker: marker(),
        }
        .into(),
    )
}

fn round_trip_nota<Value>(value: Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    let recovered = NotaSource::new(&encoded)
        .parse::<Value>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn meta_requests_round_trip_through_rkyv_frames() {
    for request in [deploy_input(), pin_input()] {
        let frame = request.encode_signal_frame().expect("encode request");
        let (_route, decoded) = Input::decode_signal_frame(&frame).expect("decode request");
        assert_eq!(decoded, request);
    }
}

#[test]
fn meta_replies_round_trip_through_rkyv_frames() {
    let reply = deploy_accepted_output();
    let frame = reply.encode_signal_frame().expect("encode reply");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");
    assert_eq!(decoded, reply);
}

#[test]
fn meta_roots_round_trip_through_nota_text() {
    round_trip_nota(deploy_input());
    round_trip_nota(pin_input());
    round_trip_nota(deploy_accepted_output());
}

#[test]
fn meta_nota_heads_are_owner_policy_verbs() {
    assert!(deploy_input().to_nota().starts_with("(Deploy "));
    assert!(pin_input().to_nota().starts_with("(Pin "));
    assert!(
        deploy_accepted_output()
            .to_nota()
            .starts_with("(DeployAccepted ")
    );
}
