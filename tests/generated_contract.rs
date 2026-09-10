use meta_signal_lojix::{
    ByteViewable, PinRejectionReason, PinRequest, Query, RejectedPin, Response, Restorable, Signal,
    Signalizable,
};
use signal_lojix::StateMarker;

fn pin_query() -> Query {
    Query::Pin(PinRequest {
        cluster_name: "production.eu".into(),
        node_name: ".state/cache".into(),
        generation_identifier: 11,
        pin_label: "release.candidate".into(),
    })
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
        database_marker: StateMarker {
            commit_sequence: 7,
            state_digest: 13,
        },
    });
    let sent = response.signalize().expect("signalize response");
    let received = Signal::<Response>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore response"), response);
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
}
