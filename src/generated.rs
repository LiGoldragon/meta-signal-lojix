#![allow(dead_code)]
#![allow(clippy::redundant_closure)]
pub type TestRejectedPayload = RejectedTest;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserEnvironmentDeployment(
    pub signal_lojix::ClusterName,
    pub signal_lojix::NodeName,
    pub signal_lojix::UserName,
    pub signal_lojix::ProposalSource,
    pub signal_lojix::SecretsInput,
    pub signal_lojix::FlakeReference,
    pub signal_lojix::DeploymentTransport,
    pub signal_lojix::DeploymentInputMode,
    pub signal_lojix::DeploymentOutputSelector,
    pub signal_lojix::ActivationBackend,
    pub signal_lojix::UserEnvironmentAction,
    pub signal_lojix::SourceRevisionPolicy,
    pub std::option::Option<signal_lojix::NixBuilderSpec>,
    pub std::vec::Vec<ExtraSubstituter>,
);
impl datom_codec::Datomic for UserEnvironmentDeployment {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 14)?;
        let p0: signal_lojix::ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::UserName = datom_codec::Positional::position(&mut p)?;
        let p3: signal_lojix::ProposalSource = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: signal_lojix::SecretsInput = datom_codec::Positional::position(&mut p)?;
        let p5: signal_lojix::FlakeReference = datom_codec::Positional::position(
            &mut p,
        )?;
        let p6: signal_lojix::DeploymentTransport = datom_codec::Positional::position(
            &mut p,
        )?;
        let p7: signal_lojix::DeploymentInputMode = datom_codec::Positional::position(
            &mut p,
        )?;
        let p8: signal_lojix::DeploymentOutputSelector = datom_codec::Positional::position(
            &mut p,
        )?;
        let p9: signal_lojix::ActivationBackend = datom_codec::Positional::position(
            &mut p,
        )?;
        let p10: signal_lojix::UserEnvironmentAction = datom_codec::Positional::position(
            &mut p,
        )?;
        let p11: signal_lojix::SourceRevisionPolicy = datom_codec::Positional::position(
            &mut p,
        )?;
        let p12: std::option::Option<signal_lojix::NixBuilderSpec> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p13: std::vec::Vec<ExtraSubstituter> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(
            Self(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13),
        )
    }
}
impl protos::Conceivable<datom_codec::Datom> for UserEnvironmentDeployment {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.8)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.9)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.10)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.11)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.12)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.13)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostDeployment(
    pub signal_lojix::ClusterName,
    pub signal_lojix::NodeName,
    pub signal_lojix::HostComposition,
    pub signal_lojix::ProposalSource,
    pub signal_lojix::SecretsInput,
    pub signal_lojix::FlakeReference,
    pub signal_lojix::DeploymentTransport,
    pub signal_lojix::DeploymentInputMode,
    pub signal_lojix::DeploymentOutputSelector,
    pub signal_lojix::ActivationBackend,
    pub signal_lojix::HostDeployAction,
    pub signal_lojix::SourceRevisionPolicy,
    pub std::option::Option<signal_lojix::NixBuilderSpec>,
    pub std::vec::Vec<ExtraSubstituter>,
);
impl datom_codec::Datomic for HostDeployment {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 14)?;
        let p0: signal_lojix::ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::HostComposition = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: signal_lojix::ProposalSource = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: signal_lojix::SecretsInput = datom_codec::Positional::position(&mut p)?;
        let p5: signal_lojix::FlakeReference = datom_codec::Positional::position(
            &mut p,
        )?;
        let p6: signal_lojix::DeploymentTransport = datom_codec::Positional::position(
            &mut p,
        )?;
        let p7: signal_lojix::DeploymentInputMode = datom_codec::Positional::position(
            &mut p,
        )?;
        let p8: signal_lojix::DeploymentOutputSelector = datom_codec::Positional::position(
            &mut p,
        )?;
        let p9: signal_lojix::ActivationBackend = datom_codec::Positional::position(
            &mut p,
        )?;
        let p10: signal_lojix::HostDeployAction = datom_codec::Positional::position(
            &mut p,
        )?;
        let p11: signal_lojix::SourceRevisionPolicy = datom_codec::Positional::position(
            &mut p,
        )?;
        let p12: std::option::Option<signal_lojix::NixBuilderSpec> = datom_codec::Positional::position(
            &mut p,
        )?;
        let p13: std::vec::Vec<ExtraSubstituter> = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(
            Self(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13),
        )
    }
}
impl protos::Conceivable<datom_codec::Datom> for HostDeployment {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.5)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.6)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.7)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.8)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.9)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.10)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.11)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.12)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.13)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppliedPin(
    pub signal_lojix::GenerationIdentifier,
    pub signal_lojix::PinLabel,
    pub signal_lojix::GenerationSlot,
    pub signal_lojix::GenerationSlot,
    pub signal_lojix::DatabaseMarker,
);
impl datom_codec::Datomic for AppliedPin {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 5)?;
        let p0: signal_lojix::GenerationIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: signal_lojix::PinLabel = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::GenerationSlot = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: signal_lojix::GenerationSlot = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4))
    }
}
impl protos::Conceivable<datom_codec::Datom> for AppliedPin {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinRejectionReason {
    PinSlotExhausted,
    InternalError,
    NodeUnknown,
    PinLabelInUse,
    GenerationUnknown,
}
impl datom_codec::Datomic for PinRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "PinSlotExhausted" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::PinSlotExhausted)
            }
            "InternalError" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InternalError)
            }
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            "PinLabelInUse" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::PinLabelInUse)
            }
            "GenerationUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GenerationUnknown)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for PinRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::PinSlotExhausted => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("PinSlotExhausted")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InternalError => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InternalError")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::PinLabelInUse => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("PinLabelInUse")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::GenerationUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GenerationUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeSelection {
    All,
    Nodes(std::vec::Vec<signal_lojix::NodeName>),
}
impl datom_codec::Datomic for NodeSelection {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "All" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::All)
            }
            "Nodes" => {
                std::result::Result::Ok(Self::Nodes(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for NodeSelection {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::All => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("All").expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::Nodes(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Nodes").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetireRejectionReason {
    NodeUnknown,
    GenerationUnknown,
    GenerationPinned,
    InternalError,
    GenerationActive,
}
impl datom_codec::Datomic for RetireRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            "GenerationUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GenerationUnknown)
            }
            "GenerationPinned" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GenerationPinned)
            }
            "InternalError" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InternalError)
            }
            "GenerationActive" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GenerationActive)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for RetireRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::GenerationUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GenerationUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::GenerationPinned => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GenerationPinned")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InternalError => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InternalError")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::GenerationActive => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GenerationActive")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedTest(pub TestRejectionReason, pub signal_lojix::DatabaseMarker);
impl datom_codec::Datomic for RejectedTest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: TestRejectionReason = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedTest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type PinRejectedPayload = RejectedPin;
pub type UnpinnedPayload = AppliedUnpin;
pub type TestPayload = TestRequest;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedDeploy(pub signal_lojix::DeploymentRecord);
impl datom_codec::Datomic for RejectedDeploy {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 1)?;
        let p0: signal_lojix::DeploymentRecord = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedDeploy {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type PinnedPayload = AppliedPin;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnpinRejectionReason {
    GenerationNotPinned,
    PinLabelUnknown,
    InternalError,
    NodeUnknown,
}
impl datom_codec::Datomic for UnpinRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "GenerationNotPinned" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::GenerationNotPinned)
            }
            "PinLabelUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::PinLabelUnknown)
            }
            "InternalError" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InternalError)
            }
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for UnpinRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::GenerationNotPinned => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("GenerationNotPinned")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::PinLabelUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("PinLabelUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InternalError => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InternalError")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
pub type DeployTerminalPayload = signal_lojix::DeploymentRecord;
pub type RetirePayload = RetireRequest;
pub type PinPayload = PinRequest;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinRequest(
    pub signal_lojix::ClusterName,
    pub signal_lojix::NodeName,
    pub signal_lojix::GenerationIdentifier,
    pub signal_lojix::PinLabel,
);
impl datom_codec::Datomic for PinRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 4)?;
        let p0: signal_lojix::ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::GenerationIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: signal_lojix::PinLabel = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2, p3))
    }
}
impl protos::Conceivable<datom_codec::Datom> for PinRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedUnpin(pub UnpinRejectionReason, pub signal_lojix::DatabaseMarker);
impl datom_codec::Datomic for RejectedUnpin {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: UnpinRejectionReason = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedUnpin {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TestRequest {
    Run(TestRun),
    Check(QuickCheck),
}
impl datom_codec::Datomic for TestRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Run" => std::result::Result::Ok(Self::Run(datom_codec::Carrying::body(v)?)),
            "Check" => {
                std::result::Result::Ok(Self::Check(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Run(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Run").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Check(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Check").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedPin(pub PinRejectionReason, pub signal_lojix::DatabaseMarker);
impl datom_codec::Datomic for RejectedPin {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: PinRejectionReason = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedPin {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnpinRequest(
    pub signal_lojix::ClusterName,
    pub signal_lojix::NodeName,
    pub signal_lojix::PinLabel,
);
impl datom_codec::Datomic for UnpinRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: signal_lojix::ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::PinLabel = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for UnpinRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type RetiredPayload = AppliedRetire;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtraSubstituter(pub protos::Text, pub protos::Text);
impl datom_codec::Datomic for ExtraSubstituter {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: protos::Text = datom_codec::Positional::position(&mut p)?;
        let p1: protos::Text = datom_codec::Positional::position(&mut p)?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for ExtraSubstituter {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type DeployPayload = DeployRequest;
pub type UnpinPayload = UnpinRequest;
pub type TestedPayload = AcceptedTest;
pub type DeployRejectedPayload = RejectedDeploy;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeployHandle(
    pub signal_lojix::DeploymentIdentifier,
    pub signal_lojix::DatabaseMarker,
);
impl datom_codec::Datomic for DeployHandle {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: signal_lojix::DeploymentIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeployHandle {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetireRequest(
    pub signal_lojix::ClusterName,
    pub signal_lojix::NodeName,
    pub signal_lojix::GenerationIdentifier,
);
impl datom_codec::Datomic for RetireRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: signal_lojix::ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::NodeName = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::GenerationIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RetireRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type UnpinRejectedPayload = RejectedUnpin;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeployRequest {
    UserEnvironment(UserEnvironmentDeployment),
    Host(HostDeployment),
}
impl datom_codec::Datomic for DeployRequest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "UserEnvironment" => {
                std::result::Result::Ok(
                    Self::UserEnvironment(datom_codec::Carrying::body(v)?),
                )
            }
            "Host" => {
                std::result::Result::Ok(Self::Host(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for DeployRequest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::UserEnvironment(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("UserEnvironment")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Host(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Host").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub type QuickCheck = std::vec::Vec<signal_lojix::NodeName>;
pub type RetireRejectedPayload = RejectedRetire;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedTest(
    pub signal_lojix::TestRunIdentifier,
    pub signal_lojix::DatabaseMarker,
);
impl datom_codec::Datomic for AcceptedTest {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: signal_lojix::TestRunIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for AcceptedTest {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppliedUnpin(
    pub signal_lojix::GenerationIdentifier,
    pub signal_lojix::PinLabel,
    pub signal_lojix::GenerationSlot,
    pub signal_lojix::GenerationSlot,
    pub signal_lojix::DatabaseMarker,
);
impl datom_codec::Datomic for AppliedUnpin {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 5)?;
        let p0: signal_lojix::GenerationIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: signal_lojix::PinLabel = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::GenerationSlot = datom_codec::Positional::position(
            &mut p,
        )?;
        let p3: signal_lojix::GenerationSlot = datom_codec::Positional::position(
            &mut p,
        )?;
        let p4: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3, p4))
    }
}
impl protos::Conceivable<datom_codec::Datom> for AppliedUnpin {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.4)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl datom_codec::Datomic for TestRejectionReason {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "SubstrateUnavailable" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::SubstrateUnavailable)
            }
            "NoTestDefaults" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NoTestDefaults)
            }
            "ClusterUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::ClusterUnknown)
            }
            "HostDeclaresNoVmHost" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::HostDeclaresNoVmHost)
            }
            "LiveNotYetEnabled" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::LiveNotYetEnabled)
            }
            "NodeUnknown" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::NodeUnknown)
            }
            "VmHostNotDeclaredForNode" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::VmHostNotDeclaredForNode)
            }
            "InternalError" => {
                datom_codec::Headed::nothing(v)?;
                std::result::Result::Ok(Self::InternalError)
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRejectionReason {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::SubstrateUnavailable => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("SubstrateUnavailable")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NoTestDefaults => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NoTestDefaults")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::ClusterUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("ClusterUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::HostDeclaresNoVmHost => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("HostDeclaresNoVmHost")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::LiveNotYetEnabled => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("LiveNotYetEnabled")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::NodeUnknown => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("NodeUnknown")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::VmHostNotDeclaredForNode => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("VmHostNotDeclaredForNode")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                    Self::InternalError => {
                        datom_codec::Datom::Word(
                            datom_codec::DatomWord::try_from(
                                    protos::Word::try_from("InternalError")
                                        .expect("static variant"),
                                )
                                .expect("stable variant"),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedRetire(pub RetireRejectionReason, pub signal_lojix::DatabaseMarker);
impl datom_codec::Datomic for RejectedRetire {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 2)?;
        let p0: RetireRejectionReason = datom_codec::Positional::position(&mut p)?;
        let p1: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1))
    }
}
impl protos::Conceivable<datom_codec::Datom> for RejectedRetire {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
pub type DeployAcceptedPayload = DeployHandle;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppliedRetire(
    pub signal_lojix::GenerationIdentifier,
    pub signal_lojix::GenerationSlot,
    pub signal_lojix::DatabaseMarker,
);
impl datom_codec::Datomic for AppliedRetire {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 3)?;
        let p0: signal_lojix::GenerationIdentifier = datom_codec::Positional::position(
            &mut p,
        )?;
        let p1: signal_lojix::GenerationSlot = datom_codec::Positional::position(
            &mut p,
        )?;
        let p2: signal_lojix::DatabaseMarker = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2))
    }
}
impl protos::Conceivable<datom_codec::Datom> for AppliedRetire {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestRun(
    pub signal_lojix::ClusterName,
    pub NodeSelection,
    pub signal_lojix::HostSelection,
    pub signal_lojix::TestExecutionProfile,
);
impl datom_codec::Datomic for TestRun {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let mut p = datom_codec::Sited::positions(site, 4)?;
        let p0: signal_lojix::ClusterName = datom_codec::Positional::position(&mut p)?;
        let p1: NodeSelection = datom_codec::Positional::position(&mut p)?;
        let p2: signal_lojix::HostSelection = datom_codec::Positional::position(&mut p)?;
        let p3: signal_lojix::TestExecutionProfile = datom_codec::Positional::position(
            &mut p,
        )?;
        std::result::Result::Ok(Self(p0, p1, p2, p3))
    }
}
impl protos::Conceivable<datom_codec::Datom> for TestRun {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                datom_codec::Datom::Struct(
                    vec![
                        protos::Conceivable::conceive(& self.0)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.1)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.2)
                        .expect("infallible datom ascent").1,
                        protos::Conceivable::conceive(& self.3)
                        .expect("infallible datom ascent").1
                    ],
                ),
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Retire(RetirePayload),
    Pin(PinPayload),
    Deploy(DeployPayload),
    Test(TestPayload),
    Unpin(UnpinPayload),
}
impl datom_codec::Datomic for Request {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "Retire" => {
                std::result::Result::Ok(Self::Retire(datom_codec::Carrying::body(v)?))
            }
            "Pin" => std::result::Result::Ok(Self::Pin(datom_codec::Carrying::body(v)?)),
            "Deploy" => {
                std::result::Result::Ok(Self::Deploy(datom_codec::Carrying::body(v)?))
            }
            "Test" => {
                std::result::Result::Ok(Self::Test(datom_codec::Carrying::body(v)?))
            }
            "Unpin" => {
                std::result::Result::Ok(Self::Unpin(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Request {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::Retire(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Retire").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Pin(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Pin").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Deploy(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Deploy").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Test(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Test").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Unpin(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Unpin").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    PinRejected(PinRejectedPayload),
    DeployRejected(DeployRejectedPayload),
    DeployAccepted(DeployAcceptedPayload),
    TestRejected(TestRejectedPayload),
    Unpinned(UnpinnedPayload),
    Tested(TestedPayload),
    UnpinRejected(UnpinRejectedPayload),
    DeployTerminal(DeployTerminalPayload),
    Pinned(PinnedPayload),
    RetireRejected(RetireRejectedPayload),
    Retired(RetiredPayload),
}
impl datom_codec::Datomic for Response {
    fn incorporate(
        site: datom_codec::Site<'_>,
    ) -> std::result::Result<Self, datom_codec::Fault> {
        let v = datom_codec::Sited::variant(site)?;
        match v.name {
            "PinRejected" => {
                std::result::Result::Ok(
                    Self::PinRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "DeployRejected" => {
                std::result::Result::Ok(
                    Self::DeployRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "DeployAccepted" => {
                std::result::Result::Ok(
                    Self::DeployAccepted(datom_codec::Carrying::body(v)?),
                )
            }
            "TestRejected" => {
                std::result::Result::Ok(
                    Self::TestRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "Unpinned" => {
                std::result::Result::Ok(Self::Unpinned(datom_codec::Carrying::body(v)?))
            }
            "Tested" => {
                std::result::Result::Ok(Self::Tested(datom_codec::Carrying::body(v)?))
            }
            "UnpinRejected" => {
                std::result::Result::Ok(
                    Self::UnpinRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "DeployTerminal" => {
                std::result::Result::Ok(
                    Self::DeployTerminal(datom_codec::Carrying::body(v)?),
                )
            }
            "Pinned" => {
                std::result::Result::Ok(Self::Pinned(datom_codec::Carrying::body(v)?))
            }
            "RetireRejected" => {
                std::result::Result::Ok(
                    Self::RetireRejected(datom_codec::Carrying::body(v)?),
                )
            }
            "Retired" => {
                std::result::Result::Ok(Self::Retired(datom_codec::Carrying::body(v)?))
            }
            _ => {
                std::result::Result::Err(
                    datom_codec::Headed::reject(
                        &v,
                        datom_codec::Problem::UnknownVariant(
                            protos::Word::try_from(v.name).expect("variant name"),
                        ),
                    ),
                )
            }
        }
    }
}
impl protos::Conceivable<datom_codec::Datom> for Response {
    type Fault = std::convert::Infallible;
    fn conceive(
        &self,
    ) -> std::result::Result<protos::Situated<datom_codec::Datom>, Self::Fault> {
        std::result::Result::Ok(
            protos::Situated(
                protos::Situation {
                    extent: protos::Extent(0, 0),
                    children: vec![],
                },
                match self {
                    Self::PinRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("PinRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::DeployRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("DeployRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::DeployAccepted(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("DeployAccepted")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::TestRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("TestRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Unpinned(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Unpinned")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Tested(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Tested").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::UnpinRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("UnpinRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::DeployTerminal(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("DeployTerminal")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Pinned(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Pinned").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::RetireRejected(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("RetireRejected")
                                .expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                    Self::Retired(p0) => {
                        datom_codec::Datom::Variant(
                            protos::Symbol::try_from("Retired").expect("static variant"),
                            std::boxed::Box::new(
                                protos::Conceivable::conceive(p0)
                                    .expect("infallible datom ascent")
                                    .1,
                            ),
                        )
                    }
                },
            ),
        )
    }
}
pub trait WireConversion: Sized {
    type Wire;
    fn into_wire(self) -> Self::Wire;
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault>;
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WireFault {
    Text,
}
pub type TestRejectedPayloadWire = RejectedTestWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UserEnvironmentDeploymentWire(
    pub signal_lojix::ClusterNameWire,
    pub signal_lojix::NodeNameWire,
    pub signal_lojix::UserNameWire,
    pub signal_lojix::ProposalSourceWire,
    pub signal_lojix::SecretsInputWire,
    pub signal_lojix::FlakeReferenceWire,
    pub signal_lojix::DeploymentTransportWire,
    pub signal_lojix::DeploymentInputModeWire,
    pub signal_lojix::DeploymentOutputSelectorWire,
    pub signal_lojix::ActivationBackendWire,
    pub signal_lojix::UserEnvironmentActionWire,
    pub signal_lojix::SourceRevisionPolicyWire,
    pub std::option::Option<signal_lojix::NixBuilderSpecWire>,
    pub std::vec::Vec<ExtraSubstituterWire>,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HostDeploymentWire(
    pub signal_lojix::ClusterNameWire,
    pub signal_lojix::NodeNameWire,
    pub signal_lojix::HostCompositionWire,
    pub signal_lojix::ProposalSourceWire,
    pub signal_lojix::SecretsInputWire,
    pub signal_lojix::FlakeReferenceWire,
    pub signal_lojix::DeploymentTransportWire,
    pub signal_lojix::DeploymentInputModeWire,
    pub signal_lojix::DeploymentOutputSelectorWire,
    pub signal_lojix::ActivationBackendWire,
    pub signal_lojix::HostDeployActionWire,
    pub signal_lojix::SourceRevisionPolicyWire,
    pub std::option::Option<signal_lojix::NixBuilderSpecWire>,
    pub std::vec::Vec<ExtraSubstituterWire>,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AppliedPinWire(
    pub signal_lojix::GenerationIdentifierWire,
    pub signal_lojix::PinLabelWire,
    pub signal_lojix::GenerationSlotWire,
    pub signal_lojix::GenerationSlotWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PinRejectionReasonWire {
    PinSlotExhausted,
    InternalError,
    NodeUnknown,
    PinLabelInUse,
    GenerationUnknown,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum NodeSelectionWire {
    All,
    Nodes(std::vec::Vec<signal_lojix::NodeNameWire>),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RetireRejectionReasonWire {
    NodeUnknown,
    GenerationUnknown,
    GenerationPinned,
    InternalError,
    GenerationActive,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedTestWire(
    pub TestRejectionReasonWire,
    pub signal_lojix::DatabaseMarkerWire,
);
pub type PinRejectedPayloadWire = RejectedPinWire;
pub type UnpinnedPayloadWire = AppliedUnpinWire;
pub type TestPayloadWire = TestRequestWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedDeployWire(pub signal_lojix::DeploymentRecordWire);
pub type PinnedPayloadWire = AppliedPinWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum UnpinRejectionReasonWire {
    GenerationNotPinned,
    PinLabelUnknown,
    InternalError,
    NodeUnknown,
}
pub type DeployTerminalPayloadWire = signal_lojix::DeploymentRecordWire;
pub type RetirePayloadWire = RetireRequestWire;
pub type PinPayloadWire = PinRequestWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PinRequestWire(
    pub signal_lojix::ClusterNameWire,
    pub signal_lojix::NodeNameWire,
    pub signal_lojix::GenerationIdentifierWire,
    pub signal_lojix::PinLabelWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedUnpinWire(
    pub UnpinRejectionReasonWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TestRequestWire {
    Run(TestRunWire),
    Check(QuickCheckWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedPinWire(
    pub PinRejectionReasonWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct UnpinRequestWire(
    pub signal_lojix::ClusterNameWire,
    pub signal_lojix::NodeNameWire,
    pub signal_lojix::PinLabelWire,
);
pub type RetiredPayloadWire = AppliedRetireWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ExtraSubstituterWire(pub std::string::String, pub std::string::String);
pub type DeployPayloadWire = DeployRequestWire;
pub type UnpinPayloadWire = UnpinRequestWire;
pub type TestedPayloadWire = AcceptedTestWire;
pub type DeployRejectedPayloadWire = RejectedDeployWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeployHandleWire(
    pub signal_lojix::DeploymentIdentifierWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RetireRequestWire(
    pub signal_lojix::ClusterNameWire,
    pub signal_lojix::NodeNameWire,
    pub signal_lojix::GenerationIdentifierWire,
);
pub type UnpinRejectedPayloadWire = RejectedUnpinWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DeployRequestWire {
    UserEnvironment(UserEnvironmentDeploymentWire),
    Host(HostDeploymentWire),
}
pub type QuickCheckWire = std::vec::Vec<signal_lojix::NodeNameWire>;
pub type RetireRejectedPayloadWire = RejectedRetireWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AcceptedTestWire(
    pub signal_lojix::TestRunIdentifierWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AppliedUnpinWire(
    pub signal_lojix::GenerationIdentifierWire,
    pub signal_lojix::PinLabelWire,
    pub signal_lojix::GenerationSlotWire,
    pub signal_lojix::GenerationSlotWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TestRejectionReasonWire {
    SubstrateUnavailable,
    NoTestDefaults,
    ClusterUnknown,
    HostDeclaresNoVmHost,
    LiveNotYetEnabled,
    NodeUnknown,
    VmHostNotDeclaredForNode,
    InternalError,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RejectedRetireWire(
    pub RetireRejectionReasonWire,
    pub signal_lojix::DatabaseMarkerWire,
);
pub type DeployAcceptedPayloadWire = DeployHandleWire;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct AppliedRetireWire(
    pub signal_lojix::GenerationIdentifierWire,
    pub signal_lojix::GenerationSlotWire,
    pub signal_lojix::DatabaseMarkerWire,
);
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TestRunWire(
    pub signal_lojix::ClusterNameWire,
    pub NodeSelectionWire,
    pub signal_lojix::HostSelectionWire,
    pub signal_lojix::TestExecutionProfileWire,
);
impl WireConversion for UserEnvironmentDeployment {
    type Wire = UserEnvironmentDeploymentWire;
    fn into_wire(self) -> Self::Wire {
        let UserEnvironmentDeployment(
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
            p11,
            p12,
            p13,
        ) = self;
        UserEnvironmentDeploymentWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
            <_ as signal_lojix::WireConversion>::into_wire(p3),
            <_ as signal_lojix::WireConversion>::into_wire(p4),
            <_ as signal_lojix::WireConversion>::into_wire(p5),
            <_ as signal_lojix::WireConversion>::into_wire(p6),
            <_ as signal_lojix::WireConversion>::into_wire(p7),
            <_ as signal_lojix::WireConversion>::into_wire(p8),
            <_ as signal_lojix::WireConversion>::into_wire(p9),
            <_ as signal_lojix::WireConversion>::into_wire(p10),
            <_ as signal_lojix::WireConversion>::into_wire(p11),
            p12.map(|value| <_ as signal_lojix::WireConversion>::into_wire(value)),
            p13
                .into_iter()
                .map(|value| <ExtraSubstituter as WireConversion>::into_wire(value))
                .collect(),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let UserEnvironmentDeploymentWire(
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
            p11,
            p12,
            p13,
        ) = wire;
        Ok(
            UserEnvironmentDeployment(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p3)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p4)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p5)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p6)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p7)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p8)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p9)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p10)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p11)
                    .map_err(|_| WireFault::Text)?,
                p12
                    .map(|value| {
                        <_ as signal_lojix::WireConversion>::try_from_wire(value)
                            .map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                p13
                    .into_iter()
                    .map(|value| <ExtraSubstituter as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
            ),
        )
    }
}
impl WireConversion for HostDeployment {
    type Wire = HostDeploymentWire;
    fn into_wire(self) -> Self::Wire {
        let HostDeployment(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13) = self;
        HostDeploymentWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
            <_ as signal_lojix::WireConversion>::into_wire(p3),
            <_ as signal_lojix::WireConversion>::into_wire(p4),
            <_ as signal_lojix::WireConversion>::into_wire(p5),
            <_ as signal_lojix::WireConversion>::into_wire(p6),
            <_ as signal_lojix::WireConversion>::into_wire(p7),
            <_ as signal_lojix::WireConversion>::into_wire(p8),
            <_ as signal_lojix::WireConversion>::into_wire(p9),
            <_ as signal_lojix::WireConversion>::into_wire(p10),
            <_ as signal_lojix::WireConversion>::into_wire(p11),
            p12.map(|value| <_ as signal_lojix::WireConversion>::into_wire(value)),
            p13
                .into_iter()
                .map(|value| <ExtraSubstituter as WireConversion>::into_wire(value))
                .collect(),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let HostDeploymentWire(
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
            p11,
            p12,
            p13,
        ) = wire;
        Ok(
            HostDeployment(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p3)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p4)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p5)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p6)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p7)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p8)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p9)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p10)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p11)
                    .map_err(|_| WireFault::Text)?,
                p12
                    .map(|value| {
                        <_ as signal_lojix::WireConversion>::try_from_wire(value)
                            .map_err(|_| WireFault::Text)
                    })
                    .transpose()?,
                p13
                    .into_iter()
                    .map(|value| <ExtraSubstituter as WireConversion>::try_from_wire(
                        value,
                    ))
                    .collect::<std::result::Result<std::vec::Vec<_>, WireFault>>()?,
            ),
        )
    }
}
impl WireConversion for AppliedPin {
    type Wire = AppliedPinWire;
    fn into_wire(self) -> Self::Wire {
        let AppliedPin(p0, p1, p2, p3, p4) = self;
        AppliedPinWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
            <_ as signal_lojix::WireConversion>::into_wire(p3),
            <_ as signal_lojix::WireConversion>::into_wire(p4),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let AppliedPinWire(p0, p1, p2, p3, p4) = wire;
        Ok(
            AppliedPin(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p3)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p4)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for PinRejectionReason {
    type Wire = PinRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            PinRejectionReason::PinSlotExhausted => {
                PinRejectionReasonWire::PinSlotExhausted
            }
            PinRejectionReason::InternalError => PinRejectionReasonWire::InternalError,
            PinRejectionReason::NodeUnknown => PinRejectionReasonWire::NodeUnknown,
            PinRejectionReason::PinLabelInUse => PinRejectionReasonWire::PinLabelInUse,
            PinRejectionReason::GenerationUnknown => {
                PinRejectionReasonWire::GenerationUnknown
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            PinRejectionReasonWire::PinSlotExhausted => {
                Ok(PinRejectionReason::PinSlotExhausted)
            }
            PinRejectionReasonWire::InternalError => {
                Ok(PinRejectionReason::InternalError)
            }
            PinRejectionReasonWire::NodeUnknown => Ok(PinRejectionReason::NodeUnknown),
            PinRejectionReasonWire::PinLabelInUse => {
                Ok(PinRejectionReason::PinLabelInUse)
            }
            PinRejectionReasonWire::GenerationUnknown => {
                Ok(PinRejectionReason::GenerationUnknown)
            }
        }
    }
}
impl WireConversion for NodeSelection {
    type Wire = NodeSelectionWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            NodeSelection::All => NodeSelectionWire::All,
            NodeSelection::Nodes(value) => {
                NodeSelectionWire::Nodes(
                    value
                        .into_iter()
                        .map(|value| <_ as signal_lojix::WireConversion>::into_wire(
                            value,
                        ))
                        .collect(),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            NodeSelectionWire::All => Ok(NodeSelection::All),
            NodeSelectionWire::Nodes(value) => {
                Ok(
                    NodeSelection::Nodes(
                        value
                            .into_iter()
                            .map(|value| {
                                <_ as signal_lojix::WireConversion>::try_from_wire(value)
                                    .map_err(|_| WireFault::Text)
                            })
                            .collect::<
                                std::result::Result<std::vec::Vec<_>, WireFault>,
                            >()?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for RetireRejectionReason {
    type Wire = RetireRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            RetireRejectionReason::NodeUnknown => RetireRejectionReasonWire::NodeUnknown,
            RetireRejectionReason::GenerationUnknown => {
                RetireRejectionReasonWire::GenerationUnknown
            }
            RetireRejectionReason::GenerationPinned => {
                RetireRejectionReasonWire::GenerationPinned
            }
            RetireRejectionReason::InternalError => {
                RetireRejectionReasonWire::InternalError
            }
            RetireRejectionReason::GenerationActive => {
                RetireRejectionReasonWire::GenerationActive
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RetireRejectionReasonWire::NodeUnknown => {
                Ok(RetireRejectionReason::NodeUnknown)
            }
            RetireRejectionReasonWire::GenerationUnknown => {
                Ok(RetireRejectionReason::GenerationUnknown)
            }
            RetireRejectionReasonWire::GenerationPinned => {
                Ok(RetireRejectionReason::GenerationPinned)
            }
            RetireRejectionReasonWire::InternalError => {
                Ok(RetireRejectionReason::InternalError)
            }
            RetireRejectionReasonWire::GenerationActive => {
                Ok(RetireRejectionReason::GenerationActive)
            }
        }
    }
}
impl WireConversion for RejectedTest {
    type Wire = RejectedTestWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedTest(p0, p1) = self;
        RejectedTestWire(
            <TestRejectionReason as WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedTestWire(p0, p1) = wire;
        Ok(
            RejectedTest(
                <TestRejectionReason as WireConversion>::try_from_wire(p0)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for RejectedDeploy {
    type Wire = RejectedDeployWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedDeploy(p0) = self;
        RejectedDeployWire(<_ as signal_lojix::WireConversion>::into_wire(p0))
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedDeployWire(p0) = wire;
        Ok(
            RejectedDeploy(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for UnpinRejectionReason {
    type Wire = UnpinRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            UnpinRejectionReason::GenerationNotPinned => {
                UnpinRejectionReasonWire::GenerationNotPinned
            }
            UnpinRejectionReason::PinLabelUnknown => {
                UnpinRejectionReasonWire::PinLabelUnknown
            }
            UnpinRejectionReason::InternalError => {
                UnpinRejectionReasonWire::InternalError
            }
            UnpinRejectionReason::NodeUnknown => UnpinRejectionReasonWire::NodeUnknown,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            UnpinRejectionReasonWire::GenerationNotPinned => {
                Ok(UnpinRejectionReason::GenerationNotPinned)
            }
            UnpinRejectionReasonWire::PinLabelUnknown => {
                Ok(UnpinRejectionReason::PinLabelUnknown)
            }
            UnpinRejectionReasonWire::InternalError => {
                Ok(UnpinRejectionReason::InternalError)
            }
            UnpinRejectionReasonWire::NodeUnknown => {
                Ok(UnpinRejectionReason::NodeUnknown)
            }
        }
    }
}
impl WireConversion for PinRequest {
    type Wire = PinRequestWire;
    fn into_wire(self) -> Self::Wire {
        let PinRequest(p0, p1, p2, p3) = self;
        PinRequestWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
            <_ as signal_lojix::WireConversion>::into_wire(p3),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let PinRequestWire(p0, p1, p2, p3) = wire;
        Ok(
            PinRequest(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p3)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for RejectedUnpin {
    type Wire = RejectedUnpinWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedUnpin(p0, p1) = self;
        RejectedUnpinWire(
            <UnpinRejectionReason as WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedUnpinWire(p0, p1) = wire;
        Ok(
            RejectedUnpin(
                <UnpinRejectionReason as WireConversion>::try_from_wire(p0)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for TestRequest {
    type Wire = TestRequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            TestRequest::Run(value) => {
                TestRequestWire::Run(<TestRun as WireConversion>::into_wire(value))
            }
            TestRequest::Check(value) => {
                TestRequestWire::Check(
                    value
                        .into_iter()
                        .map(|value| <_ as signal_lojix::WireConversion>::into_wire(
                            value,
                        ))
                        .collect(),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            TestRequestWire::Run(value) => {
                Ok(TestRequest::Run(<TestRun as WireConversion>::try_from_wire(value)?))
            }
            TestRequestWire::Check(value) => {
                Ok(
                    TestRequest::Check(
                        value
                            .into_iter()
                            .map(|value| {
                                <_ as signal_lojix::WireConversion>::try_from_wire(value)
                                    .map_err(|_| WireFault::Text)
                            })
                            .collect::<
                                std::result::Result<std::vec::Vec<_>, WireFault>,
                            >()?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for RejectedPin {
    type Wire = RejectedPinWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedPin(p0, p1) = self;
        RejectedPinWire(
            <PinRejectionReason as WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedPinWire(p0, p1) = wire;
        Ok(
            RejectedPin(
                <PinRejectionReason as WireConversion>::try_from_wire(p0)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for UnpinRequest {
    type Wire = UnpinRequestWire;
    fn into_wire(self) -> Self::Wire {
        let UnpinRequest(p0, p1, p2) = self;
        UnpinRequestWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let UnpinRequestWire(p0, p1, p2) = wire;
        Ok(
            UnpinRequest(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for ExtraSubstituter {
    type Wire = ExtraSubstituterWire;
    fn into_wire(self) -> Self::Wire {
        let ExtraSubstituter(p0, p1) = self;
        ExtraSubstituterWire(p0.to_string(), p1.to_string())
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let ExtraSubstituterWire(p0, p1) = wire;
        Ok(
            ExtraSubstituter(
                protos::Text::try_from(p0).map_err(|_| WireFault::Text)?,
                protos::Text::try_from(p1).map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for DeployHandle {
    type Wire = DeployHandleWire;
    fn into_wire(self) -> Self::Wire {
        let DeployHandle(p0, p1) = self;
        DeployHandleWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let DeployHandleWire(p0, p1) = wire;
        Ok(
            DeployHandle(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for RetireRequest {
    type Wire = RetireRequestWire;
    fn into_wire(self) -> Self::Wire {
        let RetireRequest(p0, p1, p2) = self;
        RetireRequestWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RetireRequestWire(p0, p1, p2) = wire;
        Ok(
            RetireRequest(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for DeployRequest {
    type Wire = DeployRequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            DeployRequest::UserEnvironment(value) => {
                DeployRequestWire::UserEnvironment(
                    <UserEnvironmentDeployment as WireConversion>::into_wire(value),
                )
            }
            DeployRequest::Host(value) => {
                DeployRequestWire::Host(
                    <HostDeployment as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            DeployRequestWire::UserEnvironment(value) => {
                Ok(
                    DeployRequest::UserEnvironment(
                        <UserEnvironmentDeployment as WireConversion>::try_from_wire(
                            value,
                        )?,
                    ),
                )
            }
            DeployRequestWire::Host(value) => {
                Ok(
                    DeployRequest::Host(
                        <HostDeployment as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for AcceptedTest {
    type Wire = AcceptedTestWire;
    fn into_wire(self) -> Self::Wire {
        let AcceptedTest(p0, p1) = self;
        AcceptedTestWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let AcceptedTestWire(p0, p1) = wire;
        Ok(
            AcceptedTest(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for AppliedUnpin {
    type Wire = AppliedUnpinWire;
    fn into_wire(self) -> Self::Wire {
        let AppliedUnpin(p0, p1, p2, p3, p4) = self;
        AppliedUnpinWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
            <_ as signal_lojix::WireConversion>::into_wire(p3),
            <_ as signal_lojix::WireConversion>::into_wire(p4),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let AppliedUnpinWire(p0, p1, p2, p3, p4) = wire;
        Ok(
            AppliedUnpin(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p3)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p4)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for TestRejectionReason {
    type Wire = TestRejectionReasonWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            TestRejectionReason::SubstrateUnavailable => {
                TestRejectionReasonWire::SubstrateUnavailable
            }
            TestRejectionReason::NoTestDefaults => {
                TestRejectionReasonWire::NoTestDefaults
            }
            TestRejectionReason::ClusterUnknown => {
                TestRejectionReasonWire::ClusterUnknown
            }
            TestRejectionReason::HostDeclaresNoVmHost => {
                TestRejectionReasonWire::HostDeclaresNoVmHost
            }
            TestRejectionReason::LiveNotYetEnabled => {
                TestRejectionReasonWire::LiveNotYetEnabled
            }
            TestRejectionReason::NodeUnknown => TestRejectionReasonWire::NodeUnknown,
            TestRejectionReason::VmHostNotDeclaredForNode => {
                TestRejectionReasonWire::VmHostNotDeclaredForNode
            }
            TestRejectionReason::InternalError => TestRejectionReasonWire::InternalError,
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            TestRejectionReasonWire::SubstrateUnavailable => {
                Ok(TestRejectionReason::SubstrateUnavailable)
            }
            TestRejectionReasonWire::NoTestDefaults => {
                Ok(TestRejectionReason::NoTestDefaults)
            }
            TestRejectionReasonWire::ClusterUnknown => {
                Ok(TestRejectionReason::ClusterUnknown)
            }
            TestRejectionReasonWire::HostDeclaresNoVmHost => {
                Ok(TestRejectionReason::HostDeclaresNoVmHost)
            }
            TestRejectionReasonWire::LiveNotYetEnabled => {
                Ok(TestRejectionReason::LiveNotYetEnabled)
            }
            TestRejectionReasonWire::NodeUnknown => Ok(TestRejectionReason::NodeUnknown),
            TestRejectionReasonWire::VmHostNotDeclaredForNode => {
                Ok(TestRejectionReason::VmHostNotDeclaredForNode)
            }
            TestRejectionReasonWire::InternalError => {
                Ok(TestRejectionReason::InternalError)
            }
        }
    }
}
impl WireConversion for RejectedRetire {
    type Wire = RejectedRetireWire;
    fn into_wire(self) -> Self::Wire {
        let RejectedRetire(p0, p1) = self;
        RejectedRetireWire(
            <RetireRejectionReason as WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let RejectedRetireWire(p0, p1) = wire;
        Ok(
            RejectedRetire(
                <RetireRejectionReason as WireConversion>::try_from_wire(p0)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for AppliedRetire {
    type Wire = AppliedRetireWire;
    fn into_wire(self) -> Self::Wire {
        let AppliedRetire(p0, p1, p2) = self;
        AppliedRetireWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <_ as signal_lojix::WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let AppliedRetireWire(p0, p1, p2) = wire;
        Ok(
            AppliedRetire(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p1)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for TestRun {
    type Wire = TestRunWire;
    fn into_wire(self) -> Self::Wire {
        let TestRun(p0, p1, p2, p3) = self;
        TestRunWire(
            <_ as signal_lojix::WireConversion>::into_wire(p0),
            <NodeSelection as WireConversion>::into_wire(p1),
            <_ as signal_lojix::WireConversion>::into_wire(p2),
            <_ as signal_lojix::WireConversion>::into_wire(p3),
        )
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        let TestRunWire(p0, p1, p2, p3) = wire;
        Ok(
            TestRun(
                <_ as signal_lojix::WireConversion>::try_from_wire(p0)
                    .map_err(|_| WireFault::Text)?,
                <NodeSelection as WireConversion>::try_from_wire(p1)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p2)
                    .map_err(|_| WireFault::Text)?,
                <_ as signal_lojix::WireConversion>::try_from_wire(p3)
                    .map_err(|_| WireFault::Text)?,
            ),
        )
    }
}
impl WireConversion for Request {
    type Wire = RequestWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Request::Retire(value) => {
                RequestWire::Retire(<RetireRequest as WireConversion>::into_wire(value))
            }
            Request::Pin(value) => {
                RequestWire::Pin(<PinRequest as WireConversion>::into_wire(value))
            }
            Request::Deploy(value) => {
                RequestWire::Deploy(<DeployRequest as WireConversion>::into_wire(value))
            }
            Request::Test(value) => {
                RequestWire::Test(<TestRequest as WireConversion>::into_wire(value))
            }
            Request::Unpin(value) => {
                RequestWire::Unpin(<UnpinRequest as WireConversion>::into_wire(value))
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            RequestWire::Retire(value) => {
                Ok(
                    Request::Retire(
                        <RetireRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Pin(value) => {
                Ok(Request::Pin(<PinRequest as WireConversion>::try_from_wire(value)?))
            }
            RequestWire::Deploy(value) => {
                Ok(
                    Request::Deploy(
                        <DeployRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            RequestWire::Test(value) => {
                Ok(Request::Test(<TestRequest as WireConversion>::try_from_wire(value)?))
            }
            RequestWire::Unpin(value) => {
                Ok(
                    Request::Unpin(
                        <UnpinRequest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
impl WireConversion for Response {
    type Wire = ResponseWire;
    fn into_wire(self) -> Self::Wire {
        match self {
            Response::PinRejected(value) => {
                ResponseWire::PinRejected(
                    <RejectedPin as WireConversion>::into_wire(value),
                )
            }
            Response::DeployRejected(value) => {
                ResponseWire::DeployRejected(
                    <RejectedDeploy as WireConversion>::into_wire(value),
                )
            }
            Response::DeployAccepted(value) => {
                ResponseWire::DeployAccepted(
                    <DeployHandle as WireConversion>::into_wire(value),
                )
            }
            Response::TestRejected(value) => {
                ResponseWire::TestRejected(
                    <RejectedTest as WireConversion>::into_wire(value),
                )
            }
            Response::Unpinned(value) => {
                ResponseWire::Unpinned(
                    <AppliedUnpin as WireConversion>::into_wire(value),
                )
            }
            Response::Tested(value) => {
                ResponseWire::Tested(<AcceptedTest as WireConversion>::into_wire(value))
            }
            Response::UnpinRejected(value) => {
                ResponseWire::UnpinRejected(
                    <RejectedUnpin as WireConversion>::into_wire(value),
                )
            }
            Response::DeployTerminal(value) => {
                ResponseWire::DeployTerminal(
                    <_ as signal_lojix::WireConversion>::into_wire(value),
                )
            }
            Response::Pinned(value) => {
                ResponseWire::Pinned(<AppliedPin as WireConversion>::into_wire(value))
            }
            Response::RetireRejected(value) => {
                ResponseWire::RetireRejected(
                    <RejectedRetire as WireConversion>::into_wire(value),
                )
            }
            Response::Retired(value) => {
                ResponseWire::Retired(
                    <AppliedRetire as WireConversion>::into_wire(value),
                )
            }
        }
    }
    fn try_from_wire(wire: Self::Wire) -> std::result::Result<Self, WireFault> {
        match wire {
            ResponseWire::PinRejected(value) => {
                Ok(
                    Response::PinRejected(
                        <RejectedPin as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::DeployRejected(value) => {
                Ok(
                    Response::DeployRejected(
                        <RejectedDeploy as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::DeployAccepted(value) => {
                Ok(
                    Response::DeployAccepted(
                        <DeployHandle as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::TestRejected(value) => {
                Ok(
                    Response::TestRejected(
                        <RejectedTest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Unpinned(value) => {
                Ok(
                    Response::Unpinned(
                        <AppliedUnpin as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Tested(value) => {
                Ok(
                    Response::Tested(
                        <AcceptedTest as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::UnpinRejected(value) => {
                Ok(
                    Response::UnpinRejected(
                        <RejectedUnpin as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::DeployTerminal(value) => {
                Ok(
                    Response::DeployTerminal(
                        <_ as signal_lojix::WireConversion>::try_from_wire(value)
                            .map_err(|_| WireFault::Text)?,
                    ),
                )
            }
            ResponseWire::Pinned(value) => {
                Ok(
                    Response::Pinned(
                        <AppliedPin as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::RetireRejected(value) => {
                Ok(
                    Response::RetireRejected(
                        <RejectedRetire as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
            ResponseWire::Retired(value) => {
                Ok(
                    Response::Retired(
                        <AppliedRetire as WireConversion>::try_from_wire(value)?,
                    ),
                )
            }
        }
    }
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum RequestWire {
    Retire(RetirePayloadWire),
    Pin(PinPayloadWire),
    Deploy(DeployPayloadWire),
    Test(TestPayloadWire),
    Unpin(UnpinPayloadWire),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ResponseWire {
    PinRejected(PinRejectedPayloadWire),
    DeployRejected(DeployRejectedPayloadWire),
    DeployAccepted(DeployAcceptedPayloadWire),
    TestRejected(TestRejectedPayloadWire),
    Unpinned(UnpinnedPayloadWire),
    Tested(TestedPayloadWire),
    UnpinRejected(UnpinRejectedPayloadWire),
    DeployTerminal(DeployTerminalPayloadWire),
    Pinned(PinnedPayloadWire),
    RetireRejected(RetireRejectedPayloadWire),
    Retired(RetiredPayloadWire),
}
