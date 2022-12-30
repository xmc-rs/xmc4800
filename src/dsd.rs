#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved2: [u8; 0x1c],
    #[doc = "0x28 - OCDS Control and Status Register"]
    pub ocs: OCS,
    _reserved3: [u8; 0x54],
    #[doc = "0x80 - Global Configuration Register"]
    pub globcfg: GLOBCFG,
    _reserved4: [u8; 0x04],
    #[doc = "0x88 - Global Run Control Register"]
    pub globrc: GLOBRC,
    _reserved5: [u8; 0x14],
    #[doc = "0xa0 - Carrier Generator Configuration Register"]
    pub cgcfg: CGCFG,
    _reserved6: [u8; 0x3c],
    #[doc = "0xe0 - Event Flag Register"]
    pub evflag: EVFLAG,
    #[doc = "0xe4 - Event Flag Clear Register"]
    pub evflagclr: EVFLAGCLR,
}
#[doc = "CLC (rw) register accessor: an alias for `Reg<CLC_SPEC>`"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCS (rw) register accessor: an alias for `Reg<OCS_SPEC>`"]
pub type OCS = crate::Reg<ocs::OCS_SPEC>;
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "GLOBCFG (rw) register accessor: an alias for `Reg<GLOBCFG_SPEC>`"]
pub type GLOBCFG = crate::Reg<globcfg::GLOBCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "GLOBRC (rw) register accessor: an alias for `Reg<GLOBRC_SPEC>`"]
pub type GLOBRC = crate::Reg<globrc::GLOBRC_SPEC>;
#[doc = "Global Run Control Register"]
pub mod globrc;
#[doc = "CGCFG (rw) register accessor: an alias for `Reg<CGCFG_SPEC>`"]
pub type CGCFG = crate::Reg<cgcfg::CGCFG_SPEC>;
#[doc = "Carrier Generator Configuration Register"]
pub mod cgcfg;
#[doc = "EVFLAG (rw) register accessor: an alias for `Reg<EVFLAG_SPEC>`"]
pub type EVFLAG = crate::Reg<evflag::EVFLAG_SPEC>;
#[doc = "Event Flag Register"]
pub mod evflag;
#[doc = "EVFLAGCLR (w) register accessor: an alias for `Reg<EVFLAGCLR_SPEC>`"]
pub type EVFLAGCLR = crate::Reg<evflagclr::EVFLAGCLR_SPEC>;
#[doc = "Event Flag Clear Register"]
pub mod evflagclr;
