// Handwritten operational behavior for the authority-verified owner Lojix Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_lojix::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = [$(stringify!($field)),*].len();
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_newtype!(z2VVxs, z2VYoc);
wire_struct!(z2VSqt { field_0: signal_lojix::schema::lib::z2VXtV, field_1: signal_lojix::schema::lib::z2VXGN, field_2: signal_lojix::schema::lib::z2VXcT, field_3: signal_lojix::schema::lib::z2VYbZ, field_4: signal_lojix::schema::lib::z2VXKF, field_5: signal_lojix::schema::lib::z2VSn8, field_6: signal_lojix::schema::lib::z2VQgC, field_7: signal_lojix::schema::lib::z2VXma, field_8: signal_lojix::schema::lib::z2VWva, field_9: signal_lojix::schema::lib::z2VbPz, field_10: signal_lojix::schema::lib::z2VLic, field_11: Option<signal_lojix::schema::lib::z2VatQ>, field_12: Vec<z2Vf5Q> });
wire_struct!(z2VXPp { field_0: signal_lojix::schema::lib::z2VXtV, field_1: signal_lojix::schema::lib::z2VXGN, field_2: signal_lojix::schema::lib::z2VU8v, field_3: signal_lojix::schema::lib::z2VYbZ, field_4: signal_lojix::schema::lib::z2VXKF, field_5: signal_lojix::schema::lib::z2VSn8, field_6: signal_lojix::schema::lib::z2VQgC, field_7: signal_lojix::schema::lib::z2VXma, field_8: signal_lojix::schema::lib::z2VWva, field_9: signal_lojix::schema::lib::z2VZH3, field_10: signal_lojix::schema::lib::z2VLic, field_11: Option<signal_lojix::schema::lib::z2VatQ>, field_12: Vec<z2Vf5Q> });
wire_struct!(z2VWgw { field_0: signal_lojix::schema::lib::z2VU8F, field_1: signal_lojix::schema::lib::z2VMFV, field_2: signal_lojix::schema::lib::z2VVKC, field_3: signal_lojix::schema::lib::z2VVKC, field_4: signal_lojix::schema::lib::z2VaUx });
wire_enum!(z2Vdjb { unit { 0 => z2VX44 : "PinSlotExhausted", 1 => z2VdpR : "InternalError", 2 => z2VUC7 : "NodeUnknown", 3 => z2VbwW : "PinLabelInUse", 4 => z2Vb8N : "GenerationUnknown" } unary {  } });
wire_enum!(z2VQf9 { unit { 0 => z2VbPt : "All" } unary { 1 => z2VSB4(Vec<signal_lojix::schema::lib::z2VXGN>) : "Nodes" } });
wire_enum!(z2VQTA { unit { 0 => z2VcHt : "NodeUnknown", 1 => z2VcK6 : "GenerationUnknown", 2 => z2VXoJ : "GenerationPinned", 3 => z2VVFQ : "InternalError", 4 => z2VWNx : "GenerationActive" } unary {  } });
wire_struct!(z2VYoc { field_0: z2VYMd, field_1: signal_lojix::schema::lib::z2VaUx });
wire_newtype!(z2VeyT, z2VaWL);
wire_newtype!(z2VToC, z2Vd13);
wire_newtype!(z2VNX9, z2VbsY);
wire_struct!(z2Vd1z { field_0: signal_lojix::schema::lib::z2Vbti });
wire_newtype!(z2VSkE, z2VWgw);
wire_enum!(z2VLb3 { unit { 0 => z2VeB5 : "GenerationNotPinned", 1 => z2VU6C : "PinLabelUnknown", 2 => z2VMap : "InternalError", 3 => z2VXpt : "NodeUnknown" } unary {  } });
wire_external_newtype!(z2VagP, signal_lojix::schema::lib::z2Vbti);
wire_newtype!(z2VXPq, z2VRNK);
wire_newtype!(z2VLhK, z2VdJT);
wire_struct!(z2VdJT { field_0: signal_lojix::schema::lib::z2VXtV, field_1: signal_lojix::schema::lib::z2VXGN, field_2: signal_lojix::schema::lib::z2VU8F, field_3: signal_lojix::schema::lib::z2VMFV });
wire_struct!(z2VNC3 { field_0: z2VLb3, field_1: signal_lojix::schema::lib::z2VaUx });
wire_enum!(z2VbsY { unit {  } unary { 0 => z2VVnZ(z2VUZQ) : "Run", 1 => z2VacZ(z2VV26) : "Check" } });
wire_struct!(z2VaWL { field_0: z2Vdjb, field_1: signal_lojix::schema::lib::z2VaUx });
wire_struct!(z2VL1i { field_0: signal_lojix::schema::lib::z2VXtV, field_1: signal_lojix::schema::lib::z2VXGN, field_2: signal_lojix::schema::lib::z2VMFV });
wire_newtype!(z2VVa3, z2VLcY);
wire_struct!(z2Vf5Q { field_0: std::string::String, field_1: std::string::String });
wire_enum!(z2VeCY { unit {  } unary { 0 => z2VXfM(z2VeyT) : "PinRejected", 1 => z2VVNP(z2VNJ9) : "DeployRejected", 2 => z2VZGL(z2VaSW) : "DeployAccepted", 3 => z2VLs4(z2VVxs) : "TestRejected", 4 => z2VU4f(z2VToC) : "Unpinned", 5 => z2VXfT(z2VU7x) : "Tested", 6 => z2VViz(z2VTwF) : "UnpinRejected", 7 => z2VUkZ(z2VagP) : "DeployTerminal", 8 => z2VTEY(z2VSkE) : "Pinned", 9 => z2VZQK(z2VdVE) : "RetireRejected", 10 => z2VShF(z2VVa3) : "Retired" } });
wire_newtype!(z2Vay1, z2VRo3);
wire_newtype!(z2VSC9, z2VL1i);
wire_newtype!(z2VU7x, z2VNWG);
wire_struct!(z2VX4m { field_0: signal_lojix::schema::lib::z2Vdkm, field_1: signal_lojix::schema::lib::z2VaUx });
wire_struct!(z2VRNK { field_0: signal_lojix::schema::lib::z2VXtV, field_1: signal_lojix::schema::lib::z2VXGN, field_2: signal_lojix::schema::lib::z2VU8F });
wire_newtype!(z2VTwF, z2VNC3);
wire_enum!(z2VRo3 { unit {  } unary { 0 => z2VWv3(z2VSqt) : "UserEnvironment", 1 => z2VMWS(z2VXPp) : "Host" } });
wire_external_newtype!(z2VV26, Vec<signal_lojix::schema::lib::z2VXGN>);
wire_newtype!(z2VdVE, z2VdjM);
wire_struct!(z2VNWG { field_0: signal_lojix::schema::lib::z2VNzL, field_1: signal_lojix::schema::lib::z2VaUx });
wire_struct!(z2Vd13 { field_0: signal_lojix::schema::lib::z2VU8F, field_1: signal_lojix::schema::lib::z2VMFV, field_2: signal_lojix::schema::lib::z2VVKC, field_3: signal_lojix::schema::lib::z2VVKC, field_4: signal_lojix::schema::lib::z2VaUx });
wire_enum!(z2VYMd { unit { 0 => z2VV5m : "SubstrateUnavailable", 1 => z2VLeY : "NoTestDefaults", 2 => z2Vb51 : "ClusterUnknown", 3 => z2VLRJ : "HostDeclaresNoVmHost", 4 => z2VP1k : "LiveNotYetEnabled", 5 => z2Veut : "NodeUnknown", 6 => z2Vavq : "VmHostNotDeclaredForNode", 7 => z2VKyX : "InternalError" } unary {  } });
wire_struct!(z2VdjM { field_0: z2VQTA, field_1: signal_lojix::schema::lib::z2VaUx });
wire_enum!(z2VW7Q { unit {  } unary { 0 => z2VM8s(z2VXPq) : "Retire", 1 => z2VevS(z2VLhK) : "Pin", 2 => z2VXB8(z2Vay1) : "Deploy", 3 => z2VMJW(z2VNX9) : "Test", 4 => z2Vc5o(z2VSC9) : "Unpin" } });
wire_newtype!(z2VaSW, z2VX4m);
wire_struct!(z2VLcY { field_0: signal_lojix::schema::lib::z2VU8F, field_1: signal_lojix::schema::lib::z2VVKC, field_2: signal_lojix::schema::lib::z2VaUx });
wire_struct!(z2VUZQ { field_0: signal_lojix::schema::lib::z2VXtV, field_1: z2VQf9, field_2: signal_lojix::schema::lib::z2Vbmn, field_3: signal_lojix::schema::lib::z2VL3w });
wire_newtype!(z2VNJ9, z2Vd1z);

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_lojix::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_lojix::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_lojix::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VW7Q);
archive_root!(z2VeCY);

pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(6) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason {
    Rejected,
    Unavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal {
    pub reason: EngineRefusalReason,
    pub detail: std::string::String,
}

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self {
        Self {
            reason: EngineRefusalReason::Rejected,
            detail,
        }
    }

    pub fn unavailable(detail: std::string::String) -> Self {
        Self {
            reason: EngineRefusalReason::Unavailable,
            detail,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")]
    FrameEncode,
    #[error("failed to decode bound signal frame")]
    ArchiveDecode,
    #[error("unexpected signal frame body")]
    UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")]
    OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute {
    Deploy,
    Pin,
    Unpin,
    Retire,
    Test,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    DeployAccepted,
    DeployRejected,
    DeployTerminal,
    Pinned,
    PinRejected,
    Unpinned,
    UnpinRejected,
    Retired,
    RetireRejected,
    Tested,
    TestRejected,
}

impl z2VW7Q {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VXB8(_) => InputRoute::Deploy,
            Self::z2VevS(_) => InputRoute::Pin,
            Self::z2Vc5o(_) => InputRoute::Unpin,
            Self::z2VM8s(_) => InputRoute::Retire,
            Self::z2VMJW(_) => InputRoute::Test,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(0),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(
            route,
            FrameBody::Request {
                exchange,
                request: signal_frame::Request::from_payload(self),
            },
        )
    }

    pub fn encode_request_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VeCY {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VZGL(_) => OutputRoute::DeployAccepted,
            Self::z2VVNP(_) => OutputRoute::DeployRejected,
            Self::z2VUkZ(_) => OutputRoute::DeployTerminal,
            Self::z2VTEY(_) => OutputRoute::Pinned,
            Self::z2VXfM(_) => OutputRoute::PinRejected,
            Self::z2VU4f(_) => OutputRoute::Unpinned,
            Self::z2VViz(_) => OutputRoute::UnpinRejected,
            Self::z2VShF(_) => OutputRoute::Retired,
            Self::z2VZQK(_) => OutputRoute::RetireRejected,
            Self::z2VXfT(_) => OutputRoute::Tested,
            Self::z2VLs4(_) => OutputRoute::TestRejected,
        }
    }

    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(
            signal_frame::RootCode::new(1),
            signal_frame::VariantCode::new(self.route() as u8),
        )
    }

    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(signal_frame::NonEmpty::single(
            signal_frame::SubReply::Ok(self),
        ));
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }

    pub fn encode_reply_frame(
        self,
        exchange: signal_frame::ExchangeIdentifier,
    ) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange)
            .encode()
            .map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2VW7Q {}

impl signal_frame::SignalOperationHeads for z2VW7Q {
    const HEADS: &'static [&'static str] = &["Deploy", "Pin", "Unpin", "Retire", "Test"];
}

impl signal_frame::LogVariant for z2VW7Q {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VW7Q, z2VeCY>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VW7Q, z2VeCY>;
pub type Request = signal_frame::Request<z2VW7Q>;
pub type ReplyEnvelope = signal_frame::Reply<z2VeCY>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VW7Q>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }

    pub fn decode_single_request(
        bytes: &[u8],
    ) -> Result<(signal_frame::ExchangeIdentifier, z2VW7Q), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 {
                    return Err(SignalFrameError::OperationCount { found });
                }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
