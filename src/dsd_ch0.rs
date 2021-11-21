#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Modulator Configuration Register"]
    pub modcfg: crate::Reg<modcfg::MODCFG_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Demodulator Input Configuration Register"]
    pub dicfg: crate::Reg<dicfg::DICFG_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Filter Configuration Register, Main CIC Filter"]
    pub fcfgc: crate::Reg<fcfgc::FCFGC_SPEC>,
    #[doc = "0x18 - Filter Configuration Register, Auxiliary Filter"]
    pub fcfga: crate::Reg<fcfga::FCFGA_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Integration Window Control Register"]
    pub iwctr: crate::Reg<iwctr::IWCTR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - Boundary Select Register"]
    pub boundsel: crate::Reg<boundsel::BOUNDSEL_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - Result Register, Main Filter"]
    pub resm: crate::Reg<resm::RESM_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x38 - Offset Register, Main Filter"]
    pub offm: crate::Reg<offm::OFFM_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - Result Register, Auxiliary Filter"]
    pub resa: crate::Reg<resa::RESA_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x50 - Time-Stamp Register"]
    pub tstmp: crate::Reg<tstmp::TSTMP_SPEC>,
    _reserved10: [u8; 0x4c],
    #[doc = "0xa0 - Carrier Generator Synchronization Register"]
    pub cgsync: crate::Reg<cgsync::CGSYNC_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0xa8 - Rectification Configuration Register"]
    pub rectcfg: crate::Reg<rectcfg::RECTCFG_SPEC>,
}
#[doc = "MODCFG register accessor: an alias for `Reg<MODCFG_SPEC>`"]
pub type MODCFG = crate::Reg<modcfg::MODCFG_SPEC>;
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "DICFG register accessor: an alias for `Reg<DICFG_SPEC>`"]
pub type DICFG = crate::Reg<dicfg::DICFG_SPEC>;
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "FCFGC register accessor: an alias for `Reg<FCFGC_SPEC>`"]
pub type FCFGC = crate::Reg<fcfgc::FCFGC_SPEC>;
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "FCFGA register accessor: an alias for `Reg<FCFGA_SPEC>`"]
pub type FCFGA = crate::Reg<fcfga::FCFGA_SPEC>;
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "IWCTR register accessor: an alias for `Reg<IWCTR_SPEC>`"]
pub type IWCTR = crate::Reg<iwctr::IWCTR_SPEC>;
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "BOUNDSEL register accessor: an alias for `Reg<BOUNDSEL_SPEC>`"]
pub type BOUNDSEL = crate::Reg<boundsel::BOUNDSEL_SPEC>;
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "RESM register accessor: an alias for `Reg<RESM_SPEC>`"]
pub type RESM = crate::Reg<resm::RESM_SPEC>;
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "OFFM register accessor: an alias for `Reg<OFFM_SPEC>`"]
pub type OFFM = crate::Reg<offm::OFFM_SPEC>;
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "RESA register accessor: an alias for `Reg<RESA_SPEC>`"]
pub type RESA = crate::Reg<resa::RESA_SPEC>;
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "TSTMP register accessor: an alias for `Reg<TSTMP_SPEC>`"]
pub type TSTMP = crate::Reg<tstmp::TSTMP_SPEC>;
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "CGSYNC register accessor: an alias for `Reg<CGSYNC_SPEC>`"]
pub type CGSYNC = crate::Reg<cgsync::CGSYNC_SPEC>;
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "RECTCFG register accessor: an alias for `Reg<RECTCFG_SPEC>`"]
pub type RECTCFG = crate::Reg<rectcfg::RECTCFG_SPEC>;
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
