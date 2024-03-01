#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pconf: Pconf,
    psus: Psus,
    pruns: Pruns,
    prunc: Prunc,
    prun: Prun,
    _reserved5: [u8; 0x0c],
    midr: Midr,
    _reserved6: [u8; 0x0c],
    halp: Halp,
    halps: Halps,
    _reserved8: [u8; 0x08],
    mcm: Mcm,
    mcsm: Mcsm,
    mcms: Mcms,
    mcmc: Mcmc,
    mcmf: Mcmf,
    _reserved13: [u8; 0x0c],
    qdc: Qdc,
    _reserved14: [u8; 0x0c],
    pflg: Pflg,
    pflge: Pflge,
    spflg: Spflg,
    rpflg: Rpflg,
    _reserved18: [u8; 0x80],
    pdbg: Pdbg,
}
impl RegisterBlock {
    #[doc = "0x00 - POSIF configuration"]
    #[inline(always)]
    pub const fn pconf(&self) -> &Pconf {
        &self.pconf
    }
    #[doc = "0x04 - POSIF Suspend Config"]
    #[inline(always)]
    pub const fn psus(&self) -> &Psus {
        &self.psus
    }
    #[doc = "0x08 - POSIF Run Bit Set"]
    #[inline(always)]
    pub const fn pruns(&self) -> &Pruns {
        &self.pruns
    }
    #[doc = "0x0c - POSIF Run Bit Clear"]
    #[inline(always)]
    pub const fn prunc(&self) -> &Prunc {
        &self.prunc
    }
    #[doc = "0x10 - POSIF Run Bit Status"]
    #[inline(always)]
    pub const fn prun(&self) -> &Prun {
        &self.prun
    }
    #[doc = "0x20 - Module Identification register"]
    #[inline(always)]
    pub const fn midr(&self) -> &Midr {
        &self.midr
    }
    #[doc = "0x30 - Hall Sensor Patterns"]
    #[inline(always)]
    pub const fn halp(&self) -> &Halp {
        &self.halp
    }
    #[doc = "0x34 - Hall Sensor Shadow Patterns"]
    #[inline(always)]
    pub const fn halps(&self) -> &Halps {
        &self.halps
    }
    #[doc = "0x40 - Multi-Channel Pattern"]
    #[inline(always)]
    pub const fn mcm(&self) -> &Mcm {
        &self.mcm
    }
    #[doc = "0x44 - Multi-Channel Shadow Pattern"]
    #[inline(always)]
    pub const fn mcsm(&self) -> &Mcsm {
        &self.mcsm
    }
    #[doc = "0x48 - Multi-Channel Pattern Control set"]
    #[inline(always)]
    pub const fn mcms(&self) -> &Mcms {
        &self.mcms
    }
    #[doc = "0x4c - Multi-Channel Pattern Control clear"]
    #[inline(always)]
    pub const fn mcmc(&self) -> &Mcmc {
        &self.mcmc
    }
    #[doc = "0x50 - Multi-Channel Pattern Control flag"]
    #[inline(always)]
    pub const fn mcmf(&self) -> &Mcmf {
        &self.mcmf
    }
    #[doc = "0x60 - Quadrature Decoder Control"]
    #[inline(always)]
    pub const fn qdc(&self) -> &Qdc {
        &self.qdc
    }
    #[doc = "0x70 - POSIF Interrupt Flags"]
    #[inline(always)]
    pub const fn pflg(&self) -> &Pflg {
        &self.pflg
    }
    #[doc = "0x74 - POSIF Interrupt Enable"]
    #[inline(always)]
    pub const fn pflge(&self) -> &Pflge {
        &self.pflge
    }
    #[doc = "0x78 - POSIF Interrupt Set"]
    #[inline(always)]
    pub const fn spflg(&self) -> &Spflg {
        &self.spflg
    }
    #[doc = "0x7c - POSIF Interrupt Clear"]
    #[inline(always)]
    pub const fn rpflg(&self) -> &Rpflg {
        &self.rpflg
    }
    #[doc = "0x100 - POSIF Debug register"]
    #[inline(always)]
    pub const fn pdbg(&self) -> &Pdbg {
        &self.pdbg
    }
}
#[doc = "PCONF (rw) register accessor: POSIF configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconf`]
module"]
#[doc(alias = "PCONF")]
pub type Pconf = crate::Reg<pconf::PconfSpec>;
#[doc = "POSIF configuration"]
pub mod pconf;
#[doc = "PSUS (rw) register accessor: POSIF Suspend Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psus`]
module"]
#[doc(alias = "PSUS")]
pub type Psus = crate::Reg<psus::PsusSpec>;
#[doc = "POSIF Suspend Config"]
pub mod psus;
#[doc = "PRUNS (w) register accessor: POSIF Run Bit Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pruns::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pruns`]
module"]
#[doc(alias = "PRUNS")]
pub type Pruns = crate::Reg<pruns::PrunsSpec>;
#[doc = "POSIF Run Bit Set"]
pub mod pruns;
#[doc = "PRUNC (w) register accessor: POSIF Run Bit Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prunc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prunc`]
module"]
#[doc(alias = "PRUNC")]
pub type Prunc = crate::Reg<prunc::PruncSpec>;
#[doc = "POSIF Run Bit Clear"]
pub mod prunc;
#[doc = "PRUN (r) register accessor: POSIF Run Bit Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prun::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prun`]
module"]
#[doc(alias = "PRUN")]
pub type Prun = crate::Reg<prun::PrunSpec>;
#[doc = "POSIF Run Bit Status"]
pub mod prun;
#[doc = "MIDR (r) register accessor: Module Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
#[doc(alias = "MIDR")]
pub type Midr = crate::Reg<midr::MidrSpec>;
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "HALP (r) register accessor: Hall Sensor Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halp`]
module"]
#[doc(alias = "HALP")]
pub type Halp = crate::Reg<halp::HalpSpec>;
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "HALPS (rw) register accessor: Hall Sensor Shadow Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`halps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halps`]
module"]
#[doc(alias = "HALPS")]
pub type Halps = crate::Reg<halps::HalpsSpec>;
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "MCM (r) register accessor: Multi-Channel Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcm`]
module"]
#[doc(alias = "MCM")]
pub type Mcm = crate::Reg<mcm::McmSpec>;
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "MCSM (rw) register accessor: Multi-Channel Shadow Pattern\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcsm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcsm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcsm`]
module"]
#[doc(alias = "MCSM")]
pub type Mcsm = crate::Reg<mcsm::McsmSpec>;
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "MCMS (w) register accessor: Multi-Channel Pattern Control set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcms::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcms`]
module"]
#[doc(alias = "MCMS")]
pub type Mcms = crate::Reg<mcms::McmsSpec>;
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "MCMC (w) register accessor: Multi-Channel Pattern Control clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmc`]
module"]
#[doc(alias = "MCMC")]
pub type Mcmc = crate::Reg<mcmc::McmcSpec>;
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "MCMF (r) register accessor: Multi-Channel Pattern Control flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmf`]
module"]
#[doc(alias = "MCMF")]
pub type Mcmf = crate::Reg<mcmf::McmfSpec>;
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "QDC (rw) register accessor: Quadrature Decoder Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdc`]
module"]
#[doc(alias = "QDC")]
pub type Qdc = crate::Reg<qdc::QdcSpec>;
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "PFLG (r) register accessor: POSIF Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pflg`]
module"]
#[doc(alias = "PFLG")]
pub type Pflg = crate::Reg<pflg::PflgSpec>;
#[doc = "POSIF Interrupt Flags"]
pub mod pflg;
#[doc = "PFLGE (rw) register accessor: POSIF Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pflge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pflge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pflge`]
module"]
#[doc(alias = "PFLGE")]
pub type Pflge = crate::Reg<pflge::PflgeSpec>;
#[doc = "POSIF Interrupt Enable"]
pub mod pflge;
#[doc = "SPFLG (w) register accessor: POSIF Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spflg`]
module"]
#[doc(alias = "SPFLG")]
pub type Spflg = crate::Reg<spflg::SpflgSpec>;
#[doc = "POSIF Interrupt Set"]
pub mod spflg;
#[doc = "RPFLG (w) register accessor: POSIF Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rpflg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpflg`]
module"]
#[doc(alias = "RPFLG")]
pub type Rpflg = crate::Reg<rpflg::RpflgSpec>;
#[doc = "POSIF Interrupt Clear"]
pub mod rpflg;
#[doc = "PDBG (r) register accessor: POSIF Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdbg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdbg`]
module"]
#[doc(alias = "PDBG")]
pub type Pdbg = crate::Reg<pdbg::PdbgSpec>;
#[doc = "POSIF Debug register"]
pub mod pdbg;
