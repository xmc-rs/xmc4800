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
#[doc = "MODCFG (rw) register accessor: Modulator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modcfg`]
module"]
pub type MODCFG = crate::Reg<modcfg::MODCFG_SPEC>;
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "DICFG (rw) register accessor: Demodulator Input Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dicfg`]
module"]
pub type DICFG = crate::Reg<dicfg::DICFG_SPEC>;
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "FCFGC (rw) register accessor: Filter Configuration Register, Main CIC Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfgc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfgc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fcfgc`]
module"]
pub type FCFGC = crate::Reg<fcfgc::FCFGC_SPEC>;
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "FCFGA (rw) register accessor: Filter Configuration Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfga::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfga::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fcfga`]
module"]
pub type FCFGA = crate::Reg<fcfga::FCFGA_SPEC>;
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "IWCTR (rw) register accessor: Integration Window Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`iwctr`]
module"]
pub type IWCTR = crate::Reg<iwctr::IWCTR_SPEC>;
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "BOUNDSEL (rw) register accessor: Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`boundsel`]
module"]
pub type BOUNDSEL = crate::Reg<boundsel::BOUNDSEL_SPEC>;
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "RESM (r) register accessor: Result Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resm`]
module"]
pub type RESM = crate::Reg<resm::RESM_SPEC>;
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "OFFM (rw) register accessor: Offset Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`offm`]
module"]
pub type OFFM = crate::Reg<offm::OFFM_SPEC>;
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "RESA (r) register accessor: Result Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resa::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`resa`]
module"]
pub type RESA = crate::Reg<resa::RESA_SPEC>;
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "TSTMP (r) register accessor: Time-Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstmp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tstmp`]
module"]
pub type TSTMP = crate::Reg<tstmp::TSTMP_SPEC>;
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "CGSYNC (rw) register accessor: Carrier Generator Synchronization Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cgsync`]
module"]
pub type CGSYNC = crate::Reg<cgsync::CGSYNC_SPEC>;
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "RECTCFG (rw) register accessor: Rectification Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rectcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rectcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rectcfg`]
module"]
pub type RECTCFG = crate::Reg<rectcfg::RECTCFG_SPEC>;
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
