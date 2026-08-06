#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use meta_signal_lojix::schema::lib::{z2VLhK, z2VW7Q, z2VX4m, z2VaSW, z2VdJT, z2VeCY};
use signal_lojix::schema::lib::{z2VMFV, z2VR89, z2VU8F, z2VXGN, z2VXtV, z2VaUx, z2Vdkm, z2VebC};

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
        field_1: z2VaUx {
            field_0: z2VR89::new(1),
            field_1: z2VebC::new(2),
        },
    }))
}

fn round_trip<Value>(value: Value) -> String
where
    Value: DotosDecode + DotosEncode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_dotos();
    let recovered = DotosSource::new(&encoded)
        .parse::<Value>()
        .expect("decode Dotos text");
    assert_eq!(recovered, value);
    encoded
}

#[test]
fn encoded_roots_round_trip_through_readable_dotos_roles() {
    assert!(round_trip(pin_request()).contains("Pin"));
    assert!(round_trip(deploy_accepted_reply()).contains("DeployAccepted"));
}
