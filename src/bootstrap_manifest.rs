//! Explicit producer-owned bootstrap authority state for the owner Lojix Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    139, 54, 208, 230, 19, 191, 101, 242, 83, 42, 222, 130, 43, 86, 253, 132, 253, 32, 243, 79, 18,
    223, 64, 54, 106, 46, 248, 166, 5, 166, 160, 201,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 52859;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 43964;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 19572, 0x643ac7ddaee60480);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 17804, 0x78cc3576603db1fe);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 34297, 0x415c054e0d5bcf60);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 22647, 0x203f476f808f901d);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 3734, 0x85983599620a5927);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 18397, 0xe329e0aede92df92);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 33026, 0x6efcebb5291baacd);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 6525, 0x962be719e9bd6957);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 28590, 0x1dfb013641f72a83);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 13857, 0xb9cc904f40367d24);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 46717, 0x21e2f47e1db2b3f3);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 24397, 0xd1cb8ed3672acb7c);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 19255, 0x7f37653381b290c8);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 49400, 0x028b33056d3655f3);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 29110, 0x6e4ded7af5064cfd);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 60788, 0x92c3708e7af4bae4);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    50144, 31177, 50415, 32797, 21450, 49061, 38637, 20429, 20438, 34380,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "OwnerRequest", 34807, 0xe39d7f818c3b045c),
    DeclarationSeat::new(Some(34807), "Deploy", 38387, 0x618b94a96632bb2f),
    DeclarationSeat::new(Some(34807), "Pin", 64447, 0x50afc2132432a58c),
    DeclarationSeat::new(Some(34807), "Unpin", 54898, 0xf28e6ca49dd5c9d5),
    DeclarationSeat::new(Some(34807), "Retire", 4616, 0x281f2fd3f38e327b),
    DeclarationSeat::new(Some(34807), "Test", 5175, 0x6ddb50d68057b58f),
    DeclarationSeat::new(None, "OwnerReply", 62017, 0xa5583a7478f4a598),
    DeclarationSeat::new(Some(62017), "DeployAccepted", 45417, 0x23d80761426be149),
    DeclarationSeat::new(Some(62017), "DeployRejected", 32312, 0x0d6136e7edd5919b),
    DeclarationSeat::new(Some(62017), "DeployTerminal", 30234, 0x8e17725a00b8ed58),
    DeclarationSeat::new(Some(62017), "Pinned", 25129, 0xa5eb458d12d314b9),
    DeclarationSeat::new(Some(62017), "PinRejected", 40024, 0x0522d06489d1c592),
    DeclarationSeat::new(Some(62017), "Unpinned", 27920, 0x2fe0257d9583f6ae),
    DeclarationSeat::new(Some(62017), "UnpinRejected", 33507, 0x565d9aa0ac291e2b),
    DeclarationSeat::new(Some(62017), "Retired", 23314, 0xe41c50bdeed7b6ec),
    DeclarationSeat::new(Some(62017), "RetireRejected", 45880, 0xaa2bed4f6845f05f),
    DeclarationSeat::new(Some(62017), "Tested", 40030, 0x555dffe49497b94b),
    DeclarationSeat::new(Some(62017), "TestRejected", 3699, 0x2d62ee4aae0246cc),
    DeclarationSeat::new(None, "DeployPayload", 51140, 0xa8700b9e92558535),
    DeclarationSeat::new(None, "PinPayload", 3134, 0x58c40437b135c626),
    DeclarationSeat::new(None, "UnpinPayload", 21626, 0xad9b70e943388668),
    DeclarationSeat::new(None, "RetirePayload", 39124, 0x573b357346915f15),
    DeclarationSeat::new(None, "TestPayload", 9272, 0x4548c4db0ab6c25d),
    DeclarationSeat::new(None, "DeployAcceptedPayload", 49371, 0xec94e8d89d1904bd),
    DeclarationSeat::new(None, "DeployRejectedPayload", 8518, 0xfe8241b5b6a74a66),
    DeclarationSeat::new(None, "DeployTerminalPayload", 50176, 0x53b4333c70a36ed7),
    DeclarationSeat::new(None, "PinnedPayload", 23487, 0x4c8b0716b3120aff),
    DeclarationSeat::new(None, "PinRejectedPayload", 64622, 0x3a1e95d8462ee04b),
    DeclarationSeat::new(None, "UnpinnedPayload", 27023, 0x4224d59eb15da5e0),
    DeclarationSeat::new(None, "UnpinRejectedPayload", 27490, 0xc076651ebf3aea84),
    DeclarationSeat::new(None, "RetiredPayload", 32988, 0x9fefbdbf195f0e50),
    DeclarationSeat::new(None, "RetireRejectedPayload", 59621, 0xd21ebb9a7717046a),
    DeclarationSeat::new(None, "TestedPayload", 28111, 0xaddaf725f54787bb),
    DeclarationSeat::new(None, "TestRejectedPayload", 34312, 0x008c1999986d3117),
    DeclarationSeat::new(None, "TestRequest", 54187, 0x7c332acba662b999),
    DeclarationSeat::new(Some(54187), "Run", 33714, 0x37b437bf3f86cffc),
    DeclarationSeat::new(Some(54187), "Check", 49954, 0xb5a09f6d64704094),
    DeclarationSeat::new(None, "NodeSelection", 16464, 0x1dbc0ae1505571ea),
    DeclarationSeat::new(Some(16464), "Nodes", 21563, 0xe81bb89c8cd735cb),
    DeclarationSeat::new(Some(16464), "All", 52583, 0x1e270f00ccdcf3c8),
    DeclarationSeat::new(None, "TestRun", 29587, 0xfd1bd03bbdf5e6be),
    DeclarationSeat::new(None, "QuickCheck", 31135, 0xc878580d4b7e3d59),
    DeclarationSeat::new(None, "ExtraSubstituter", 64967, 0xa2b5107fbf334944),
    DeclarationSeat::new(None, "HostDeployment", 39123, 0x0a4e7d78b6888a5d),
    DeclarationSeat::new(None, "UserEnvironmentDeployment", 23815, 0x0a116fc2910c1f45),
    DeclarationSeat::new(None, "DeployRequest", 20286, 0xc6d2997478154618),
    DeclarationSeat::new(Some(20286), "Host", 5867, 0xe47b351dd5ae6bc7),
    DeclarationSeat::new(Some(20286), "UserEnvironment", 37512, 0x4e1828f104d93c1a),
    DeclarationSeat::new(None, "PinRequest", 58996, 0x5e00fe868094eb6f),
    DeclarationSeat::new(None, "UnpinRequest", 837, 0x8be7ff6d798e0e22),
    DeclarationSeat::new(None, "RetireRequest", 18852, 0xbbc7d15621bc3060),
    DeclarationSeat::new(None, "DeployHandle", 38018, 0xb8d9aa62de527f36),
    DeclarationSeat::new(None, "AcceptedTest", 9221, 0xd4bc2c2d7c3c9a51),
    DeclarationSeat::new(None, "AppliedPin", 36752, 0x0e424132abf38372),
    DeclarationSeat::new(None, "AppliedUnpin", 57986, 0xd72766b99b6f5bcc),
    DeclarationSeat::new(None, "AppliedRetire", 2857, 0xf4f08d1e2bec6102),
    DeclarationSeat::new(None, "PinRejectionReason", 60454, 0x109e01902b3d0d7a),
    DeclarationSeat::new(Some(60454), "GenerationUnknown", 51683, 0xac206a45861abfc9),
    DeclarationSeat::new(Some(60454), "NodeUnknown", 28352, 0x73a8915bf4e97a23),
    DeclarationSeat::new(Some(60454), "PinLabelInUse", 54417, 0x85d7268feef77f77),
    DeclarationSeat::new(Some(60454), "PinSlotExhausted", 37977, 0x49abdff3ec29bb54),
    DeclarationSeat::new(Some(60454), "InternalError", 60734, 0x69923a7881af246f),
    DeclarationSeat::new(None, "UnpinRejectionReason", 2770, 0x52608b3a6177cade),
    DeclarationSeat::new(Some(2770), "PinLabelUnknown", 28009, 0x60eea580666f2aac),
    DeclarationSeat::new(Some(2770), "NodeUnknown", 40577, 0xbb7acf71bdbc593b),
    DeclarationSeat::new(Some(2770), "GenerationNotPinned", 61932, 0x146ed6ac5ccf93e0),
    DeclarationSeat::new(Some(2770), "InternalError", 6121, 0x983cdb24e4cb0c3f),
    DeclarationSeat::new(None, "RetireRejectionReason", 15769, 0x2465e448cb05e696),
    DeclarationSeat::new(Some(15769), "GenerationUnknown", 55669, 0x70d03e714e9a7309),
    DeclarationSeat::new(Some(15769), "NodeUnknown", 55599, 0x05101e409d9bd02d),
    DeclarationSeat::new(Some(15769), "GenerationActive", 35709, 0xf8e98f0fa40b0768),
    DeclarationSeat::new(Some(15769), "GenerationPinned", 40485, 0x7ed9b5db48e3ed5f),
    DeclarationSeat::new(Some(15769), "InternalError", 31907, 0xa85916588a194d6b),
    DeclarationSeat::new(None, "TestRejectionReason", 42360, 0xd865c1a298d349a1),
    DeclarationSeat::new(Some(42360), "ClusterUnknown", 51488, 0x3be607722ea2566e),
    DeclarationSeat::new(Some(42360), "NodeUnknown", 64415, 0xc8e11313994b878a),
    DeclarationSeat::new(
        Some(42360),
        "VmHostNotDeclaredForNode",
        51014,
        0xe947b3f51a59e8c0,
    ),
    DeclarationSeat::new(
        Some(42360),
        "HostDeclaresNoVmHost",
        2205,
        0x5f730550406e2578,
    ),
    DeclarationSeat::new(Some(42360), "NoTestDefaults", 2973, 0x243d352c3fd19319),
    DeclarationSeat::new(Some(42360), "LiveNotYetEnabled", 10931, 0xbad7a645854a62f7),
    DeclarationSeat::new(
        Some(42360),
        "SubstrateUnavailable",
        31348,
        0x0e9f2d6c2baee171,
    ),
    DeclarationSeat::new(Some(42360), "InternalError", 710, 0xf32470682bad646a),
    DeclarationSeat::new(None, "RejectedDeploy", 58041, 0x497bfa6fcb71a08c),
    DeclarationSeat::new(None, "RejectedPin", 49593, 0x7cc74cfd07fe8931),
    DeclarationSeat::new(None, "RejectedUnpin", 8164, 0x75959dfb48deab6e),
    DeclarationSeat::new(None, "RejectedRetire", 60440, 0xda49cc08884a65c6),
    DeclarationSeat::new(None, "RejectedTest", 43867, 0x3760a47898cef37e),
];
