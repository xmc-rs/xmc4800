#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pconf: PCONF,
    psus: PSUS,
    pruns: PRUNS,
    prunc: PRUNC,
    prun: PRUN,
    _reserved5: [u8; 0x0c],
    midr: MIDR,
    _reserved6: [u8; 0x0c],
    halp: HALP,
    halps: HALPS,
    _reserved8: [u8; 0x08],
    mcm: MCM,
    mcsm: MCSM,
    mcms: MCMS,
    mcmc: MCMC,
    mcmf: MCMF,
    _reserved13: [u8; 0x0c],
    qdc: QDC,
    _reserved14: [u8; 0x0c],
    pflg: PFLG,
    pflge: PFLGE,
    spflg: SPFLG,
    rpflg: RPFLG,
    _reserved18: [u8; 0x80],
    pdbg: PDBG,
}
impl RegisterBlock {
    #[doc = "0x00 - POSIF configuration"]
    #[inline(always)]
    pub const fn pconf(&self) -> &PCONF {
        &self.pconf
    }
    #[doc = "0x04 - POSIF Suspend Config"]
    #[inline(always)]
    pub const fn psus(&self) -> &PSUS {
        &self.psus
    }
    #[doc = "0x08 - POSIF Run Bit Set"]
    #[inline(always)]
    pub const fn pruns(&self) -> &PRUNS {
        &self.pruns
    }
    #[doc = "0x0c - POSIF Run Bit Clear"]
    #[inline(always)]
    pub const fn prunc(&self) -> &PRUNC {
        &self.prunc
    }
    #[doc = "0x10 - POSIF Run Bit Status"]
    #[inline(always)]
    pub const fn prun(&self) -> &PRUN {
        &self.prun
    }
    #[doc = "0x20 - Module Identification register"]
    #[inline(always)]
    pub const fn midr(&self) -> &MIDR {
        &self.midr
    }
    #[doc = "0x30 - Hall Sensor Patterns"]
    #[inline(always)]
    pub const fn halp(&self) -> &HALP {
        &self.halp
    }
    #[doc = "0x34 - Hall Sensor Shadow Patterns"]
    #[inline(always)]
    pub const fn halps(&self) -> &HALPS {
        &self.halps
    }
    #[doc = "0x40 - Multi-Channel Pattern"]
    #[inline(always)]
    pub const fn mcm(&self) -> &MCM {
        &self.mcm
    }
    #[doc = "0x44 - Multi-Channel Shadow Pattern"]
    #[inline(always)]
    pub const fn mcsm(&self) -> &MCSM {
        &self.mcsm
    }
    #[doc = "0x48 - Multi-Channel Pattern Control set"]
    #[inline(always)]
    pub const fn mcms(&self) -> &MCMS {
        &self.mcms
    }
    #[doc = "0x4c - Multi-Channel Pattern Control clear"]
    #[inline(always)]
    pub const fn mcmc(&self) -> &MCMC {
        &self.mcmc
    }
    #[doc = "0x50 - Multi-Channel Pattern Control flag"]
    #[inline(always)]
    pub const fn mcmf(&self) -> &MCMF {
        &self.mcmf
    }
    #[doc = "0x60 - Quadrature Decoder Control"]
    #[inline(always)]
    pub const fn qdc(&self) -> &QDC {
        &self.qdc
    }
    #[doc = "0x70 - POSIF Interrupt Flags"]
    #[inline(always)]
    pub const fn pflg(&self) -> &PFLG {
        &self.pflg
    }
    #[doc = "0x74 - POSIF Interrupt Enable"]
    #[inline(always)]
    pub const fn pflge(&self) -> &PFLGE {
        &self.pflge
    }
    #[doc = "0x78 - POSIF Interrupt Set"]
    #[inline(always)]
    pub const fn spflg(&self) -> &SPFLG {
        &self.spflg
    }
    #[doc = "0x7c - POSIF Interrupt Clear"]
    #[inline(always)]
    pub const fn rpflg(&self) -> &RPFLG {
        &self.rpflg
    }
    #[doc = "0x100 - POSIF Debug register"]
    #[inline(always)]
    pub const fn pdbg(&self) -> &PDBG {
        &self.pdbg
    }
}
#[doc = "PCONF (rw) register accessor: POSIF configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconf`]
module"]
pub type PCONF = crate::Reg<pconf::PCONF_SPEC>;
#[doc = "POSIF configuration"]
pub mod pconf;
#[doc = "PSUS (rw) register accessor: POSIF Suspend Config\n\nYou can [`read`](crate::Reg::read) this register and get [`psus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psus`]
module"]
pub type PSUS = crate::Reg<psus::PSUS_SPEC>;
#[doc = "POSIF Suspend Config"]
pub mod psus;
#[doc = "PRUNS (w) register accessor: POSIF Run Bit Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pruns::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pruns`]
module"]
pub type PRUNS = crate::Reg<pruns::PRUNS_SPEC>;
#[doc = "POSIF Run Bit Set"]
pub mod pruns;
#[doc = "PRUNC (w) register accessor: POSIF Run Bit Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prunc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prunc`]
module"]
pub type PRUNC = crate::Reg<prunc::PRUNC_SPEC>;
#[doc = "POSIF Run Bit Clear"]
pub mod prunc;
#[doc = "PRUN (r) register accessor: POSIF Run Bit Status\n\nYou can [`read`](crate::Reg::read) this register and get [`prun::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prun`]
module"]
pub type PRUN = crate::Reg<prun::PRUN_SPEC>;
#[doc = "POSIF Run Bit Status"]
pub mod prun;
#[doc = "MIDR (r) register accessor: Module Identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`midr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "HALP (r) register accessor: Hall Sensor Patterns\n\nYou can [`read`](crate::Reg::read) this register and get [`halp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halp`]
module"]
pub type HALP = crate::Reg<halp::HALP_SPEC>;
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "HALPS (rw) register accessor: Hall Sensor Shadow Patterns\n\nYou can [`read`](crate::Reg::read) this register and get [`halps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`halps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halps`]
module"]
pub type HALPS = crate::Reg<halps::HALPS_SPEC>;
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "MCM (r) register accessor: Multi-Channel Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`mcm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcm`]
module"]
pub type MCM = crate::Reg<mcm::MCM_SPEC>;
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "MCSM (rw) register accessor: Multi-Channel Shadow Pattern\n\nYou can [`read`](crate::Reg::read) this register and get [`mcsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcsm`]
module"]
pub type MCSM = crate::Reg<mcsm::MCSM_SPEC>;
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "MCMS (w) register accessor: Multi-Channel Pattern Control set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcms::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcms`]
module"]
pub type MCMS = crate::Reg<mcms::MCMS_SPEC>;
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "MCMC (w) register accessor: Multi-Channel Pattern Control clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmc`]
module"]
pub type MCMC = crate::Reg<mcmc::MCMC_SPEC>;
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "MCMF (r) register accessor: Multi-Channel Pattern Control flag\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmf`]
module"]
pub type MCMF = crate::Reg<mcmf::MCMF_SPEC>;
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "QDC (rw) register accessor: Quadrature Decoder Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdc`]
module"]
pub type QDC = crate::Reg<qdc::QDC_SPEC>;
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "PFLG (r) register accessor: POSIF Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`pflg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pflg`]
module"]
pub type PFLG = crate::Reg<pflg::PFLG_SPEC>;
#[doc = "POSIF Interrupt Flags"]
pub mod pflg;
#[doc = "PFLGE (rw) register accessor: POSIF Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pflge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pflge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pflge`]
module"]
pub type PFLGE = crate::Reg<pflge::PFLGE_SPEC>;
#[doc = "POSIF Interrupt Enable"]
pub mod pflge;
#[doc = "SPFLG (w) register accessor: POSIF Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spflg`]
module"]
pub type SPFLG = crate::Reg<spflg::SPFLG_SPEC>;
#[doc = "POSIF Interrupt Set"]
pub mod spflg;
#[doc = "RPFLG (w) register accessor: POSIF Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpflg`]
module"]
pub type RPFLG = crate::Reg<rpflg::RPFLG_SPEC>;
#[doc = "POSIF Interrupt Clear"]
pub mod rpflg;
#[doc = "PDBG (r) register accessor: POSIF Debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbg`]
module"]
pub type PDBG = crate::Reg<pdbg::PDBG_SPEC>;
#[doc = "POSIF Debug register"]
pub mod pdbg;
