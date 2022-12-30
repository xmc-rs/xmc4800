#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Modulator Configuration Register"]
    pub modcfg: MODCFG,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Demodulator Input Configuration Register"]
    pub dicfg: DICFG,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Filter Configuration Register, Main CIC Filter"]
    pub fcfgc: FCFGC,
    #[doc = "0x18 - Filter Configuration Register, Auxiliary Filter"]
    pub fcfga: FCFGA,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Integration Window Control Register"]
    pub iwctr: IWCTR,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - Boundary Select Register"]
    pub boundsel: BOUNDSEL,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - Result Register, Main Filter"]
    pub resm: RESM,
    _reserved7: [u8; 0x04],
    #[doc = "0x38 - Offset Register, Main Filter"]
    pub offm: OFFM,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - Result Register, Auxiliary Filter"]
    pub resa: RESA,
    _reserved9: [u8; 0x0c],
    #[doc = "0x50 - Time-Stamp Register"]
    pub tstmp: TSTMP,
    _reserved10: [u8; 0x4c],
    #[doc = "0xa0 - Carrier Generator Synchronization Register"]
    pub cgsync: CGSYNC,
    _reserved11: [u8; 0x04],
    #[doc = "0xa8 - Rectification Configuration Register"]
    pub rectcfg: RECTCFG,
}
#[doc = "MODCFG (rw) register accessor: an alias for `Reg<MODCFG_SPEC>`"]
pub type MODCFG = crate::Reg<modcfg::MODCFG_SPEC>;
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "DICFG (rw) register accessor: an alias for `Reg<DICFG_SPEC>`"]
pub type DICFG = crate::Reg<dicfg::DICFG_SPEC>;
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "FCFGC (rw) register accessor: an alias for `Reg<FCFGC_SPEC>`"]
pub type FCFGC = crate::Reg<fcfgc::FCFGC_SPEC>;
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "FCFGA (rw) register accessor: an alias for `Reg<FCFGA_SPEC>`"]
pub type FCFGA = crate::Reg<fcfga::FCFGA_SPEC>;
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "IWCTR (rw) register accessor: an alias for `Reg<IWCTR_SPEC>`"]
pub type IWCTR = crate::Reg<iwctr::IWCTR_SPEC>;
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "BOUNDSEL (rw) register accessor: an alias for `Reg<BOUNDSEL_SPEC>`"]
pub type BOUNDSEL = crate::Reg<boundsel::BOUNDSEL_SPEC>;
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "RESM (r) register accessor: an alias for `Reg<RESM_SPEC>`"]
pub type RESM = crate::Reg<resm::RESM_SPEC>;
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "OFFM (rw) register accessor: an alias for `Reg<OFFM_SPEC>`"]
pub type OFFM = crate::Reg<offm::OFFM_SPEC>;
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "RESA (r) register accessor: an alias for `Reg<RESA_SPEC>`"]
pub type RESA = crate::Reg<resa::RESA_SPEC>;
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "TSTMP (r) register accessor: an alias for `Reg<TSTMP_SPEC>`"]
pub type TSTMP = crate::Reg<tstmp::TSTMP_SPEC>;
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "CGSYNC (rw) register accessor: an alias for `Reg<CGSYNC_SPEC>`"]
pub type CGSYNC = crate::Reg<cgsync::CGSYNC_SPEC>;
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "RECTCFG (rw) register accessor: an alias for `Reg<RECTCFG_SPEC>`"]
pub type RECTCFG = crate::Reg<rectcfg::RECTCFG_SPEC>;
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
