#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use meta_signal_lojix::schema::lib::{
    DatabaseMarker, DeployHandle, DeployRequest, HostDeployment, Input, Output, PinRequest,
    RejectedDeploy, SourceRevisionPolicy,
};
use signal_lojix::schema::lib::{
    ActivationEffect, AdmissionMarker, DeploymentEnvironment, DeploymentLifecycle,
    DeploymentRecord, DeploymentRequestIdentity, DeploymentTerminal, DeploymentTerminalReason,
    GenerationArtifact, RequestedDeploymentAction, TerminalMarker,
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

fn deploy_rejected_activation_failed() -> Output {
    Output::DeployRejected(
        RejectedDeploy::new(DeploymentRecord {
            deployment_identifier: 1.into(),
            generation_identifier: 1.into(),
            deployment_request_identity: DeploymentRequestIdentity {
                deployment_environment: DeploymentEnvironment::HostEnvironment,
                cluster_name: "goldragon".to_string().into(),
                node_name: "ouranos".to_string().into(),
                generation_artifact: GenerationArtifact::CompleteHost,
                requested_deployment_action: RequestedDeploymentAction::Host(
                    signal_lojix::schema::lib::HostDeployAction::ActivateNow,
                ),
                activation_effect: ActivationEffect::LiveActivation,
                source_revision_policy: SourceRevisionPolicy::RequireImmutable,
                optional_immutable_revision: Some(
                    signal_lojix::schema::lib::ImmutableRevision::new(
                        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    ),
                ),
            },
            optional_admission_marker: Some(AdmissionMarker::new(marker())),
            deployment_lifecycle: DeploymentLifecycle::Rejected,
            optional_terminal_marker: Some(TerminalMarker::new(marker())),
            optional_deployment_terminal: Some(DeploymentTerminal::Rejected(
                DeploymentTerminalReason::ActivationFailed,
            )),
        })
        .into(),
    )
}

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(9),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(3),
    )
}

fn round_trip_dotos<Value>(value: Value)
where
    Value: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_dotos();
    let recovered = DotosSource::new(&encoded)
        .parse::<Value>()
        .expect("decode dotos text");
    assert_eq!(recovered, value);
}

#[test]
fn meta_requests_round_trip_through_rkyv_frames() {
    for request in [deploy_input(), pin_input()] {
        let frame = request
            .clone()
            .encode_request_frame(exchange())
            .expect("encode request");
        let (decoded_exchange, decoded) =
            meta_signal_lojix::schema::lib::ContractMarker::decode_single_request(&frame)
                .expect("decode request");
        assert_eq!(decoded_exchange, exchange());
        assert_eq!(decoded, request);
    }
}

#[test]
fn meta_replies_round_trip_through_rkyv_frames() {
    let reply = deploy_accepted_output();
    let frame = reply
        .clone()
        .encode_reply_frame(exchange())
        .expect("encode reply");
    let decoded =
        meta_signal_lojix::schema::lib::ContractMarker::decode_frame(&frame).expect("decode reply");
    let meta_signal_lojix::schema::lib::FrameBody::Reply {
        exchange: decoded_exchange,
        reply: decoded_reply,
    } = decoded.into_body()
    else {
        panic!("decoded frame must retain a reply body");
    };
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(
        decoded_reply,
        signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(
            reply
        ),)),
    );
}

#[test]
fn meta_roots_round_trip_through_dotos_text() {
    round_trip_dotos(deploy_input());
    round_trip_dotos(pin_input());
    round_trip_dotos(deploy_accepted_output());
}

#[test]
fn activation_failed_reason_round_trips_through_dotos_text() {
    round_trip_dotos(deploy_rejected_activation_failed());
    assert!(
        deploy_rejected_activation_failed()
            .to_dotos()
            .contains("ActivationFailed")
    );
}

#[test]
fn meta_dotos_heads_are_owner_policy_verbs() {
    assert!(deploy_input().to_dotos().contains("Deploy"));
    assert!(pin_input().to_dotos().contains("Pin"));
    assert!(
        deploy_accepted_output()
            .to_dotos()
            .contains("DeployAccepted")
    );
}
