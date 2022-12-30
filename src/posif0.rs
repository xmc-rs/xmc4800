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
#[doc = "PCONF (rw) register accessor: an alias for `Reg<PCONF_SPEC>`"]
pub type PCONF = crate::Reg<pconf::PCONF_SPEC>;
#[doc = "POSIF configuration"]
pub mod pconf;
#[doc = "PSUS (rw) register accessor: an alias for `Reg<PSUS_SPEC>`"]
pub type PSUS = crate::Reg<psus::PSUS_SPEC>;
#[doc = "POSIF Suspend Config"]
pub mod psus;
#[doc = "PRUNS (w) register accessor: an alias for `Reg<PRUNS_SPEC>`"]
pub type PRUNS = crate::Reg<pruns::PRUNS_SPEC>;
#[doc = "POSIF Run Bit Set"]
pub mod pruns;
#[doc = "PRUNC (w) register accessor: an alias for `Reg<PRUNC_SPEC>`"]
pub type PRUNC = crate::Reg<prunc::PRUNC_SPEC>;
#[doc = "POSIF Run Bit Clear"]
pub mod prunc;
#[doc = "PRUN (r) register accessor: an alias for `Reg<PRUN_SPEC>`"]
pub type PRUN = crate::Reg<prun::PRUN_SPEC>;
#[doc = "POSIF Run Bit Status"]
pub mod prun;
#[doc = "MIDR (r) register accessor: an alias for `Reg<MIDR_SPEC>`"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "HALP (r) register accessor: an alias for `Reg<HALP_SPEC>`"]
pub type HALP = crate::Reg<halp::HALP_SPEC>;
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "HALPS (rw) register accessor: an alias for `Reg<HALPS_SPEC>`"]
pub type HALPS = crate::Reg<halps::HALPS_SPEC>;
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "MCM (r) register accessor: an alias for `Reg<MCM_SPEC>`"]
pub type MCM = crate::Reg<mcm::MCM_SPEC>;
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "MCSM (rw) register accessor: an alias for `Reg<MCSM_SPEC>`"]
pub type MCSM = crate::Reg<mcsm::MCSM_SPEC>;
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "MCMS (w) register accessor: an alias for `Reg<MCMS_SPEC>`"]
pub type MCMS = crate::Reg<mcms::MCMS_SPEC>;
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "MCMC (w) register accessor: an alias for `Reg<MCMC_SPEC>`"]
pub type MCMC = crate::Reg<mcmc::MCMC_SPEC>;
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "MCMF (r) register accessor: an alias for `Reg<MCMF_SPEC>`"]
pub type MCMF = crate::Reg<mcmf::MCMF_SPEC>;
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "QDC (rw) register accessor: an alias for `Reg<QDC_SPEC>`"]
pub type QDC = crate::Reg<qdc::QDC_SPEC>;
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "PFLG (r) register accessor: an alias for `Reg<PFLG_SPEC>`"]
pub type PFLG = crate::Reg<pflg::PFLG_SPEC>;
#[doc = "POSIF Interrupt Flags"]
pub mod pflg;
#[doc = "PFLGE (rw) register accessor: an alias for `Reg<PFLGE_SPEC>`"]
pub type PFLGE = crate::Reg<pflge::PFLGE_SPEC>;
#[doc = "POSIF Interrupt Enable"]
pub mod pflge;
#[doc = "SPFLG (w) register accessor: an alias for `Reg<SPFLG_SPEC>`"]
pub type SPFLG = crate::Reg<spflg::SPFLG_SPEC>;
#[doc = "POSIF Interrupt Set"]
pub mod spflg;
#[doc = "RPFLG (w) register accessor: an alias for `Reg<RPFLG_SPEC>`"]
pub type RPFLG = crate::Reg<rpflg::RPFLG_SPEC>;
#[doc = "POSIF Interrupt Clear"]
pub mod rpflg;
#[doc = "PDBG (r) register accessor: an alias for `Reg<PDBG_SPEC>`"]
pub type PDBG = crate::Reg<pdbg::PDBG_SPEC>;
#[doc = "POSIF Debug register"]
pub mod pdbg;
