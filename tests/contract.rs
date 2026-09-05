use datom_codec::{Actualizable, IncorporationBudget, Potential, Textualizable};
use meta_signal_lojix::{MetaLojixWire, Request, RequestWire, RetireRequest, WireConversion};
use signal_frame::{
    BoundExchangeFrame, ExchangeFrameBody, ExchangeIdentifier, ExchangeLane, LaneSequence,
    RootCode, SessionEpoch, VariantCode, WireRoute,
};

fn request() -> Request {
    Request::Retire(RetireRequest(
        protos::Text::try_from("goldragon").unwrap(),
        protos::Text::try_from("ouranos").unwrap(),
        7,
    ))
}

#[test]
fn owner_request_crosses_datom_and_bound_structural_frame() {
    let expected = request();
    let text = <Request as Textualizable<datom_codec::Datom>>::textualize(&expected);
    let recovered = Potential::<Request>::from(text.as_str())
        .actualize(IncorporationBudget::try_from(128).unwrap())
        .unwrap();
    assert_eq!(recovered, expected);

    let frame = BoundExchangeFrame::<MetaLojixWire, RequestWire, meta_signal_lojix::ResponseWire>::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        ExchangeFrameBody::Request {
            exchange: ExchangeIdentifier::new(SessionEpoch::new(3), ExchangeLane::Connector, LaneSequence::new(7)),
            request: signal_frame::Request::from_payload(expected.clone().into_wire()),
        },
    );
    let decoded = BoundExchangeFrame::<MetaLojixWire, RequestWire, meta_signal_lojix::ResponseWire>::decode_length_prefixed(
        &frame.encode_length_prefixed().unwrap(),
    ).unwrap();
    let ExchangeFrameBody::Request { request, .. } = decoded.into_body() else { panic!("request body") };
    assert_eq!(Request::try_from_wire(request.payloads().clone().into_head()).unwrap(), expected);
}
