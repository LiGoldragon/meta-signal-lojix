#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
    pub nix_builder_spec_option: Option<signal_lojix::NixBuilderSpec>,
    pub extra_substituter_vector: std::vec::Vec<ExtraSubstituter>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
    pub nix_builder_spec_option: Option<signal_lojix::NixBuilderSpec>,
    pub extra_substituter_vector: std::vec::Vec<ExtraSubstituter>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ActualizedDeploySubmission {
    pub deploy_submission: DeploySubmission,
    pub horizon_definition_option: Option<horizon_lib::HorizonDefinition>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
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
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AppliedPin {
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub pin_label: signal_lojix::PinLabel,
    pub first_generation_slot: signal_lojix::GenerationSlot,
    pub second_generation_slot: signal_lojix::GenerationSlot,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum PinRejectionReason {
    PinSlotExhausted,
    InternalError,
    NodeUnknown,
    PinLabelInUse,
    GenerationUnknown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum NodeSelection {
    All,
    Nodes(std::vec::Vec<signal_lojix::NodeName>),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum RetireRejectionReason {
    NodeUnknown,
    GenerationUnknown,
    GenerationPinned,
    InternalError,
    GenerationActive,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedTest {
    pub test_rejection_reason: TestRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedDeploy {
    pub deployment_record: signal_lojix::DeploymentRecord,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RefusedDeploy {
    pub deploy_refusal_reason: DeployRefusalReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeployRefusalReason {
    ContinuationBudgetExhausted,
    NoCorrelatedDeployment,
    DurableWriteFailed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnpinRejectionReason {
    GenerationNotPinned,
    PinLabelUnknown,
    InternalError,
    NodeUnknown,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PinRequest {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub pin_label: signal_lojix::PinLabel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedUnpin {
    pub unpin_rejection_reason: UnpinRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TestRequest {
    Run(TestRun),
    Check(QuickCheck),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedPin {
    pub pin_rejection_reason: PinRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct UnpinRequest {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub pin_label: signal_lojix::PinLabel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ExtraSubstituter {
    pub first_string: String,
    pub second_string: String,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct DeployHandle {
    pub deployment_identifier: signal_lojix::DeploymentIdentifier,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RetireRequest {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_name: signal_lojix::NodeName,
    pub generation_identifier: signal_lojix::GenerationIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DeploySubmission {
    UserEnvironment(UserEnvironmentDeployment),
    Host(HostDeployment),
}
#[rustfmt::skip]
pub type QuickCheck = std::vec::Vec<signal_lojix::NodeName>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AcceptedTest {
    pub test_run_identifier: signal_lojix::TestRunIdentifier,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AppliedUnpin {
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub pin_label: signal_lojix::PinLabel,
    pub first_generation_slot: signal_lojix::GenerationSlot,
    pub second_generation_slot: signal_lojix::GenerationSlot,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum TestRejectionReason {
    SubstrateUnavailable,
    NoTestDefaults,
    ClusterUnknown,
    HostDeclaresNoVmHost,
    LiveNotYetEnabled,
    NodeUnknown,
    VmHostNotDeclaredForNode,
    InternalError,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct RejectedRetire {
    pub retire_rejection_reason: RetireRejectionReason,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct AppliedRetire {
    pub generation_identifier: signal_lojix::GenerationIdentifier,
    pub generation_slot: signal_lojix::GenerationSlot,
    pub database_marker: signal_lojix::DatabaseMarker,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TestRun {
    pub cluster_name: signal_lojix::ClusterName,
    pub node_selection: NodeSelection,
    pub host_selection: signal_lojix::HostSelection,
    pub test_execution_profile: signal_lojix::TestExecutionProfile,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Configure(signal_lojix::LojixNexusConfiguration),
    ReverseConfiguration,
    Retire(RetireRequest),
    Pin(PinRequest),
    Deploy(ActualizedDeploySubmission),
    Test(TestRequest),
    Unpin(UnpinRequest),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Configured(signal_lojix::ConfigurationReceipt),
    ConfigurationRejected(signal_lojix::ConfigurationRejection),
    ConfigurationReversed(signal_lojix::ConfigurationReceipt),
    PinRejected(RejectedPin),
    DeployRejected(RejectedDeploy),
    DeployRefused(RefusedDeploy),
    DeployAccepted(DeployHandle),
    TestRejected(RejectedTest),
    Unpinned(AppliedUnpin),
    Tested(AcceptedTest),
    UnpinRejected(RejectedUnpin),
    DeployTerminal(signal_lojix::DeploymentRecord),
    Pinned(AppliedPin),
    RetireRejected(RejectedRetire),
    Retired(AppliedRetire),
}
