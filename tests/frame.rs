use meta_signal_lojix::schema::lib::{
    ContractMarker, FrameBody, InputRoute, OutputRoute, z2VLhK, z2VW7Q, z2VX4m, z2VaSW, z2VdJT,
    z2VeCY,
};
use signal_lojix::schema::lib::{z2VMFV, z2VR89, z2VU8F, z2VXGN, z2VXtV, z2VaUx, z2Vdkm, z2VebC};

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(9),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::new(3),
    )
}

fn marker() -> z2VaUx {
    z2VaUx {
        field_0: z2VR89::new(1),
        field_1: z2VebC::new(2),
    }
}

fn pin_request() -> z2VW7Q {
    z2VW7Q::z2VevS(z2VLhK::new(z2VdJT {
        field_0: z2VXtV::new("goldragon".to_owned()),
        field_1: z2VXGN::new("ouranos".to_owned()),
        field_2: z2VU8F::new(7),
        field_3: z2VMFV::new("known-good".to_owned()),
    }))
}

fn deploy_accepted_reply() -> z2VeCY {
    z2VeCY::z2VZGL(z2VaSW::new(z2VX4m {
        field_0: z2Vdkm::new(11),
        field_1: marker(),
    }))
}

#[test]
fn handwritten_input_role_round_trips_the_encoded_request() {
    let input = pin_request();
    assert_eq!(input.route(), InputRoute::Pin);
    let frame = input
        .clone()
        .encode_request_frame(exchange())
        .expect("encode request");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&frame).expect("decode request");
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, input);
}

#[test]
fn handwritten_output_role_round_trips_the_encoded_reply() {
    let output = deploy_accepted_reply();
    assert_eq!(output.route(), OutputRoute::DeployAccepted);
    let frame = output
        .clone()
        .encode_reply_frame(exchange())
        .expect("encode reply");
    let decoded = ContractMarker::decode_frame(&frame).expect("decode reply");
    assert_eq!(
        decoded.into_body(),
        FrameBody::Reply {
            exchange: exchange(),
            reply: signal_frame::Reply::committed(signal_frame::NonEmpty::single(
                signal_frame::SubReply::Ok(output),
            )),
        }
    );
}
