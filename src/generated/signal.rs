#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserEnvironmentDeployment {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub user_name: signal_lojix::UserName,
    pub proposal_source: signal_lojix::ProposalSource,
    pub secrets_input: signal_lojix::SecretsInput,
    pub flake_reference: signal_lojix::FlakeReference,
    pub deployment_transport: signal_lojix::DeploymentTransport,
    pub deployment_input_mode: signal_lojix::DeploymentInputMode,
    pub deployment_output_selector: signal_lojix::DeploymentOutputSelector,
    pub activation_backend: signal_lojix::ActivationBackend,
    pub user_environment_action: signal_lojix::UserEnvironmentAction,
    pub source_revision_policy: signal_lojix::SourceRevisionPolicy,
    pub nix_builder_spec_option: std::option::Option<signal_lojix::NixBuilderSpec>,
    pub extra_substituter_vector: std::vec::Vec<ExtraSubstituter>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HostDeployment {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub host_composition: signal_lojix::HostComposition,
    pub proposal_source: signal_lojix::ProposalSource,
    pub secrets_input: signal_lojix::SecretsInput,
    pub flake_reference: signal_lojix::FlakeReference,
    pub deployment_transport: signal_lojix::DeploymentTransport,
    pub deployment_input_mode: signal_lojix::DeploymentInputMode,
    pub deployment_output_selector: signal_lojix::DeploymentOutputSelector,
    pub activation_backend: signal_lojix::ActivationBackend,
    pub host_deploy_action: signal_lojix::HostDeployAction,
    pub source_revision_policy: signal_lojix::SourceRevisionPolicy,
    pub nix_builder_spec_option: std::option::Option<signal_lojix::NixBuilderSpec>,
    pub extra_substituter_vector: std::vec::Vec<ExtraSubstituter>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ActualizedDeploySubmission {
    pub deploy_submission: DeploySubmission,
    pub horizon_definition_option: std::option::Option<horizon_lib::HorizonDefinition>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ClientQuery {
    Configure(signal_lojix::LojixNexusConfiguration),
    ReverseConfiguration,
    Retire(RetireRequest),
    Pin(PinRequest),
    Deploy(DeploySubmission),
    Test(TestRequest),
    Unpin(UnpinRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AppliedPin {
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub pin_label: signal_lojix::PinLabel,
    pub first_generation_slot: signal_lojix::GenerationSlot,
    pub second_generation_slot: signal_lojix::GenerationSlot,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum PinRejectionReason {
    GenerationUnknown,
    NodeUnknown,
    PinLabelInUse,
    PinSlotExhausted,
    InternalError,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NodeSelection {
    Nodes(std::vec::Vec<signal_lojix::NodeName>),
    All,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetireRejectionReason {
    GenerationUnknown,
    NodeUnknown,
    GenerationActive,
    GenerationPinned,
    InternalError,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RejectedTest {
    pub test_rejection_reason: TestRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RejectedDeploy {
    pub deployment_record: signal_lojix::DeploymentRecord,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RefusedDeploy {
    pub deploy_refusal_reason: DeployRefusalReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeployRefusalReason {
    ContinuationBudgetExhausted,
    NoCorrelatedDeployment,
    DurableWriteFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UnpinRejectionReason {
    PinLabelUnknown,
    NodeUnknown,
    GenerationNotPinned,
    InternalError,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PinRequest {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub pin_label: signal_lojix::PinLabel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RejectedUnpin {
    pub unpin_rejection_reason: UnpinRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TestRequest {
    Run(TestRun),
    Check(QuickCheck),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RejectedPin {
    pub pin_rejection_reason: PinRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UnpinRequest {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub pin_label: signal_lojix::PinLabel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExtraSubstituter {
    pub first_string: String,
    pub second_string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeployHandle {
    pub deployment_identifier: signal_lojix::DeploymentIdentifier,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RetireRequest {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub generation_identifier: signal_lojix::GenerationIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeploySubmission {
    Host(HostDeployment),
    UserEnvironment(UserEnvironmentDeployment),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuickCheck(pub std::vec::Vec<signal_lojix::NodeName>);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AcceptedTest {
    pub test_run_identifier: signal_lojix::TestRunIdentifier,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AppliedUnpin {
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub pin_label: signal_lojix::PinLabel,
    pub first_generation_slot: signal_lojix::GenerationSlot,
    pub second_generation_slot: signal_lojix::GenerationSlot,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TestRejectionReason {
    ClusterUnknown,
    NodeUnknown,
    VmHostNotDeclaredForNode,
    HostDeclaresNoVmHost,
    NoTestDefaults,
    LiveNotYetEnabled,
    SubstrateUnavailable,
    InternalError,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RejectedRetire {
    pub retire_rejection_reason: RetireRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AppliedRetire {
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub generation_slot: signal_lojix::GenerationSlot,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TestRun {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_selection: NodeSelection,
    pub host_selection: signal_lojix::HostSelection,
    pub test_execution_profile: signal_lojix::TestExecutionProfile,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Configure(signal_lojix::LojixNexusConfiguration),
    ReverseConfiguration,
    Retire(RetireRequest),
    Pin(PinRequest),
    Deploy(ActualizedDeploySubmission),
    Test(TestRequest),
    Unpin(UnpinRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Configured(signal_lojix::ConfigurationReceipt),
    ConfigurationReversed(signal_lojix::ConfigurationReceipt),
    Retired(AppliedRetire),
    Pinned(AppliedPin),
    DeployAccepted(DeployHandle),
    Tested(AcceptedTest),
    Unpinned(AppliedUnpin),
    ConfigurationRejected(signal_lojix::ConfigurationRejection),
    RetireRejected(RejectedRetire),
    PinRejected(RejectedPin),
    DeployRejected(RejectedDeploy),
    DeployRefused(RefusedDeploy),
    TestRejected(RejectedTest),
    UnpinRejected(RejectedUnpin),
    DeployTerminal(signal_lojix::DeploymentRecord),
    Refused(signal_lojix::StoreFailure),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn configure(
        &self,
        lojix_nexus_configuration: &signal_lojix::LojixNexusConfiguration,
    ) -> std::result::Result<signal_lojix::ConfigurationReceipt, Output>;
    fn reverse_configuration(
        &self,
    ) -> std::result::Result<signal_lojix::ConfigurationReceipt, Output>;
    fn retire(
        &self,
        retire_request: &RetireRequest,
    ) -> std::result::Result<AppliedRetire, Output>;
    fn pin(&self, pin_request: &PinRequest) -> std::result::Result<AppliedPin, Output>;
    fn deploy(
        &self,
        actualized_deploy_submission: &ActualizedDeploySubmission,
    ) -> std::result::Result<DeployHandle, Output>;
    fn test(
        &self,
        test_request: &TestRequest,
    ) -> std::result::Result<AcceptedTest, Output>;
    fn unpin(
        &self,
        unpin_request: &UnpinRequest,
    ) -> std::result::Result<AppliedUnpin, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output;
}
#[rustfmt::skip]
impl Answerable for Input {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Configure(carried) => {
                nexus
                    .configure(carried)
                    .map_or_else(|refusal| refusal, Output::Configured)
            }
            Input::ReverseConfiguration => {
                nexus
                    .reverse_configuration()
                    .map_or_else(|refusal| refusal, Output::ConfigurationReversed)
            }
            Input::Retire(carried) => {
                nexus.retire(carried).map_or_else(|refusal| refusal, Output::Retired)
            }
            Input::Pin(carried) => {
                nexus.pin(carried).map_or_else(|refusal| refusal, Output::Pinned)
            }
            Input::Deploy(carried) => {
                nexus
                    .deploy(carried)
                    .map_or_else(|refusal| refusal, Output::DeployAccepted)
            }
            Input::Test(carried) => {
                nexus.test(carried).map_or_else(|refusal| refusal, Output::Tested)
            }
            Input::Unpin(carried) => {
                nexus.unpin(carried).map_or_else(|refusal| refusal, Output::Unpinned)
            }
        }
    }
}
