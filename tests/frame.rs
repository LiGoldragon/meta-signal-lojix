use meta_signal_lojix::schema::lib::{
    DatabaseMarker, DeployHandle, DeployRequest, HostDeployment, Input, Output,
    SourceRevisionPolicy,
};

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

#[test]
fn default_build_round_trips_meta_request_without_nota_text() {
    let input = deploy_input();
    let frame = input.encode_signal_frame().expect("encode request");
    let (_route, decoded) = Input::decode_signal_frame(&frame).expect("decode request");

    assert_eq!(decoded, input);
}

#[test]
fn default_build_round_trips_meta_reply_without_nota_text() {
    let output = Output::DeployAccepted(
        DeployHandle {
            deployment_identifier: 1.into(),
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output.encode_signal_frame().expect("encode reply");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");

    assert_eq!(decoded, output);
}
