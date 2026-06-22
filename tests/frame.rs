use meta_signal_lojix::schema::lib::{
    AcceptedDeploy, DatabaseMarker, DeployRequest, Input, Output, ProductionNode,
    SystemDeployment,
};

fn marker() -> DatabaseMarker {
    DatabaseMarker {
        commit_sequence: 1.into(),
        state_digest: 1.into(),
    }
}

fn deploy_request() -> DeployRequest {
    DeployRequest::System(SystemDeployment {
        production_node: ProductionNode {
            cluster_name: "goldragon".to_string().into(),
            node_name: "ouranos".to_string().into(),
        },
        deployment_kind: signal_lojix::schema::lib::DeploymentKind::OsOnly,
        proposal_source: "/git/github.com/LiGoldragon/goldragon/datom.nota"
            .to_string()
            .into(),
        flake_reference: "github:LiGoldragon/CriOMOS/main".to_string().into(),
        system_action: signal_lojix::schema::lib::SystemAction::Eval,
        builder_override: None.into(),
        extra_substituters: Vec::new().into(),
        build_attribute: None.into(),
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
    let output = Output::Deployed(
        AcceptedDeploy {
            deployment_identifier: 1.into(),
            database_marker: marker(),
        }
        .into(),
    );
    let frame = output.encode_signal_frame().expect("encode reply");
    let (_route, decoded) = Output::decode_signal_frame(&frame).expect("decode reply");

    assert_eq!(decoded, output);
}
