#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    modcfg: MODCFG,
    _reserved1: [u8; 0x04],
    dicfg: DICFG,
    _reserved2: [u8; 0x08],
    fcfgc: FCFGC,
    fcfga: FCFGA,
    _reserved4: [u8; 0x04],
    iwctr: IWCTR,
    _reserved5: [u8; 0x04],
    boundsel: BOUNDSEL,
    _reserved6: [u8; 0x04],
    resm: RESM,
    _reserved7: [u8; 0x04],
    offm: OFFM,
    _reserved8: [u8; 0x04],
    resa: RESA,
    _reserved9: [u8; 0x0c],
    tstmp: TSTMP,
    _reserved10: [u8; 0x4c],
    cgsync: CGSYNC,
    _reserved11: [u8; 0x04],
    rectcfg: RECTCFG,
}
impl RegisterBlock {
    #[doc = "0x00 - Modulator Configuration Register"]
    #[inline(always)]
    pub const fn modcfg(&self) -> &MODCFG {
        &self.modcfg
    }
    #[doc = "0x08 - Demodulator Input Configuration Register"]
    #[inline(always)]
    pub const fn dicfg(&self) -> &DICFG {
        &self.dicfg
    }
    #[doc = "0x14 - Filter Configuration Register, Main CIC Filter"]
    #[inline(always)]
    pub const fn fcfgc(&self) -> &FCFGC {
        &self.fcfgc
    }
    #[doc = "0x18 - Filter Configuration Register, Auxiliary Filter"]
    #[inline(always)]
    pub const fn fcfga(&self) -> &FCFGA {
        &self.fcfga
    }
    #[doc = "0x20 - Integration Window Control Register"]
    #[inline(always)]
    pub const fn iwctr(&self) -> &IWCTR {
        &self.iwctr
    }
    #[doc = "0x28 - Boundary Select Register"]
    #[inline(always)]
    pub const fn boundsel(&self) -> &BOUNDSEL {
        &self.boundsel
    }
    #[doc = "0x30 - Result Register, Main Filter"]
    #[inline(always)]
    pub const fn resm(&self) -> &RESM {
        &self.resm
    }
    #[doc = "0x38 - Offset Register, Main Filter"]
    #[inline(always)]
    pub const fn offm(&self) -> &OFFM {
        &self.offm
    }
    #[doc = "0x40 - Result Register, Auxiliary Filter"]
    #[inline(always)]
    pub const fn resa(&self) -> &RESA {
        &self.resa
    }
    #[doc = "0x50 - Time-Stamp Register"]
    #[inline(always)]
    pub const fn tstmp(&self) -> &TSTMP {
        &self.tstmp
    }
    #[doc = "0xa0 - Carrier Generator Synchronization Register"]
    #[inline(always)]
    pub const fn cgsync(&self) -> &CGSYNC {
        &self.cgsync
    }
    #[doc = "0xa8 - Rectification Configuration Register"]
    #[inline(always)]
    pub const fn rectcfg(&self) -> &RECTCFG {
        &self.rectcfg
    }
}
#[doc = "MODCFG (rw) register accessor: Modulator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modcfg`]
module"]
pub type MODCFG = crate::Reg<modcfg::MODCFG_SPEC>;
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "DICFG (rw) register accessor: Demodulator Input Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dicfg`]
module"]
pub type DICFG = crate::Reg<dicfg::DICFG_SPEC>;
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "FCFGC (rw) register accessor: Filter Configuration Register, Main CIC Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfgc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfgc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfgc`]
module"]
pub type FCFGC = crate::Reg<fcfgc::FCFGC_SPEC>;
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "FCFGA (rw) register accessor: Filter Configuration Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfga::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfga::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfga`]
module"]
pub type FCFGA = crate::Reg<fcfga::FCFGA_SPEC>;
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "IWCTR (rw) register accessor: Integration Window Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwctr`]
module"]
pub type IWCTR = crate::Reg<iwctr::IWCTR_SPEC>;
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "BOUNDSEL (rw) register accessor: Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boundsel`]
module"]
pub type BOUNDSEL = crate::Reg<boundsel::BOUNDSEL_SPEC>;
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "RESM (r) register accessor: Result Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resm`]
module"]
pub type RESM = crate::Reg<resm::RESM_SPEC>;
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "OFFM (rw) register accessor: Offset Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offm`]
module"]
pub type OFFM = crate::Reg<offm::OFFM_SPEC>;
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "RESA (r) register accessor: Result Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resa::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resa`]
module"]
pub type RESA = crate::Reg<resa::RESA_SPEC>;
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "TSTMP (r) register accessor: Time-Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstmp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstmp`]
module"]
pub type TSTMP = crate::Reg<tstmp::TSTMP_SPEC>;
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "CGSYNC (rw) register accessor: Carrier Generator Synchronization Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgsync`]
module"]
pub type CGSYNC = crate::Reg<cgsync::CGSYNC_SPEC>;
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "RECTCFG (rw) register accessor: Rectification Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rectcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rectcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rectcfg`]
module"]
pub type RECTCFG = crate::Reg<rectcfg::RECTCFG_SPEC>;
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
