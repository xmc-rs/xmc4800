#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POSIF configuration"]
    pub pconf: PCONF,
    #[doc = "0x04 - POSIF Suspend Config"]
    pub psus: PSUS,
    #[doc = "0x08 - POSIF Run Bit Set"]
    pub pruns: PRUNS,
    #[doc = "0x0c - POSIF Run Bit Clear"]
    pub prunc: PRUNC,
    #[doc = "0x10 - POSIF Run Bit Status"]
    pub prun: PRUN,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Module Identification register"]
    pub midr: MIDR,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - Hall Sensor Patterns"]
    pub halp: HALP,
    #[doc = "0x34 - Hall Sensor Shadow Patterns"]
    pub halps: HALPS,
    _reserved8: [u8; 0x08],
    #[doc = "0x40 - Multi-Channel Pattern"]
    pub mcm: MCM,
    #[doc = "0x44 - Multi-Channel Shadow Pattern"]
    pub mcsm: MCSM,
    #[doc = "0x48 - Multi-Channel Pattern Control set"]
    pub mcms: MCMS,
    #[doc = "0x4c - Multi-Channel Pattern Control clear"]
    pub mcmc: MCMC,
    #[doc = "0x50 - Multi-Channel Pattern Control flag"]
    pub mcmf: MCMF,
    _reserved13: [u8; 0x0c],
    #[doc = "0x60 - Quadrature Decoder Control"]
    pub qdc: QDC,
    _reserved14: [u8; 0x0c],
    #[doc = "0x70 - POSIF Interrupt Flags"]
    pub pflg: PFLG,
    #[doc = "0x74 - POSIF Interrupt Enable"]
    pub pflge: PFLGE,
    #[doc = "0x78 - POSIF Interrupt Set"]
    pub spflg: SPFLG,
    #[doc = "0x7c - POSIF Interrupt Clear"]
    pub rpflg: RPFLG,
    _reserved18: [u8; 0x80],
    #[doc = "0x100 - POSIF Debug register"]
    pub pdbg: PDBG,
}
#[doc = "PCONF (rw) register accessor: POSIF configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconf`]
module"]
pub type PCONF = crate::Reg<pconf::PCONF_SPEC>;
#[doc = "POSIF configuration"]
pub mod pconf;
#[doc = "PSUS (rw) register accessor: POSIF Suspend Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psus`]
module"]
pub type PSUS = crate::Reg<psus::PSUS_SPEC>;
#[doc = "POSIF Suspend Config"]
pub mod psus;
#[doc = "PRUNS (w) register accessor: POSIF Run Bit Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pruns::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pruns`]
module"]
pub type PRUNS = crate::Reg<pruns::PRUNS_SPEC>;
#[doc = "POSIF Run Bit Set"]
pub mod pruns;
#[doc = "PRUNC (w) register accessor: POSIF Run Bit Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prunc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prunc`]
module"]
pub type PRUNC = crate::Reg<prunc::PRUNC_SPEC>;
#[doc = "POSIF Run Bit Clear"]
pub mod prunc;
#[doc = "PRUN (r) register accessor: POSIF Run Bit Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prun::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prun`]
module"]
pub type PRUN = crate::Reg<prun::PRUN_SPEC>;
#[doc = "POSIF Run Bit Status"]
pub mod prun;
#[doc = "MIDR (r) register accessor: Module Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "HALP (r) register accessor: Hall Sensor Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halp`]
module"]
pub type HALP = crate::Reg<halp::HALP_SPEC>;
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "HALPS (rw) register accessor: Hall Sensor Shadow Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`halps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halps`]
module"]
pub type HALPS = crate::Reg<halps::HALPS_SPEC>;
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "MCM (r) register accessor: Multi-Channel Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcm`]
module"]
pub type MCM = crate::Reg<mcm::MCM_SPEC>;
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "MCSM (rw) register accessor: Multi-Channel Shadow Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcsm`]
module"]
pub type MCSM = crate::Reg<mcsm::MCSM_SPEC>;
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "MCMS (w) register accessor: Multi-Channel Pattern Control set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcms::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcms`]
module"]
pub type MCMS = crate::Reg<mcms::MCMS_SPEC>;
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "MCMC (w) register accessor: Multi-Channel Pattern Control clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmc`]
module"]
pub type MCMC = crate::Reg<mcmc::MCMC_SPEC>;
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "MCMF (r) register accessor: Multi-Channel Pattern Control flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmf`]
module"]
pub type MCMF = crate::Reg<mcmf::MCMF_SPEC>;
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "QDC (rw) register accessor: Quadrature Decoder Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdc`]
module"]
pub type QDC = crate::Reg<qdc::QDC_SPEC>;
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "PFLG (r) register accessor: POSIF Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pflg`]
module"]
pub type PFLG = crate::Reg<pflg::PFLG_SPEC>;
#[doc = "POSIF Interrupt Flags"]
pub mod pflg;
#[doc = "PFLGE (rw) register accessor: POSIF Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pflge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pflge`]
module"]
pub type PFLGE = crate::Reg<pflge::PFLGE_SPEC>;
#[doc = "POSIF Interrupt Enable"]
pub mod pflge;
#[doc = "SPFLG (w) register accessor: POSIF Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spflg`]
module"]
pub type SPFLG = crate::Reg<spflg::SPFLG_SPEC>;
#[doc = "POSIF Interrupt Set"]
pub mod spflg;
#[doc = "RPFLG (w) register accessor: POSIF Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpflg`]
module"]
pub type RPFLG = crate::Reg<rpflg::RPFLG_SPEC>;
#[doc = "POSIF Interrupt Clear"]
pub mod rpflg;
#[doc = "PDBG (r) register accessor: POSIF Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdbg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbg`]
module"]
pub type PDBG = crate::Reg<pdbg::PDBG_SPEC>;
#[doc = "POSIF Debug register"]
pub mod pdbg;
