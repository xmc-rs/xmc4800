#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    modcfg: Modcfg,
    _reserved1: [u8; 0x04],
    dicfg: Dicfg,
    _reserved2: [u8; 0x08],
    fcfgc: Fcfgc,
    fcfga: Fcfga,
    _reserved4: [u8; 0x04],
    iwctr: Iwctr,
    _reserved5: [u8; 0x04],
    boundsel: Boundsel,
    _reserved6: [u8; 0x04],
    resm: Resm,
    _reserved7: [u8; 0x04],
    offm: Offm,
    _reserved8: [u8; 0x04],
    resa: Resa,
    _reserved9: [u8; 0x0c],
    tstmp: Tstmp,
    _reserved10: [u8; 0x4c],
    cgsync: Cgsync,
    _reserved11: [u8; 0x04],
    rectcfg: Rectcfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Modulator Configuration Register"]
    #[inline(always)]
    pub const fn modcfg(&self) -> &Modcfg {
        &self.modcfg
    }
    #[doc = "0x08 - Demodulator Input Configuration Register"]
    #[inline(always)]
    pub const fn dicfg(&self) -> &Dicfg {
        &self.dicfg
    }
    #[doc = "0x14 - Filter Configuration Register, Main CIC Filter"]
    #[inline(always)]
    pub const fn fcfgc(&self) -> &Fcfgc {
        &self.fcfgc
    }
    #[doc = "0x18 - Filter Configuration Register, Auxiliary Filter"]
    #[inline(always)]
    pub const fn fcfga(&self) -> &Fcfga {
        &self.fcfga
    }
    #[doc = "0x20 - Integration Window Control Register"]
    #[inline(always)]
    pub const fn iwctr(&self) -> &Iwctr {
        &self.iwctr
    }
    #[doc = "0x28 - Boundary Select Register"]
    #[inline(always)]
    pub const fn boundsel(&self) -> &Boundsel {
        &self.boundsel
    }
    #[doc = "0x30 - Result Register, Main Filter"]
    #[inline(always)]
    pub const fn resm(&self) -> &Resm {
        &self.resm
    }
    #[doc = "0x38 - Offset Register, Main Filter"]
    #[inline(always)]
    pub const fn offm(&self) -> &Offm {
        &self.offm
    }
    #[doc = "0x40 - Result Register, Auxiliary Filter"]
    #[inline(always)]
    pub const fn resa(&self) -> &Resa {
        &self.resa
    }
    #[doc = "0x50 - Time-Stamp Register"]
    #[inline(always)]
    pub const fn tstmp(&self) -> &Tstmp {
        &self.tstmp
    }
    #[doc = "0xa0 - Carrier Generator Synchronization Register"]
    #[inline(always)]
    pub const fn cgsync(&self) -> &Cgsync {
        &self.cgsync
    }
    #[doc = "0xa8 - Rectification Configuration Register"]
    #[inline(always)]
    pub const fn rectcfg(&self) -> &Rectcfg {
        &self.rectcfg
    }
}
#[doc = "MODCFG (rw) register accessor: Modulator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modcfg`]
module"]
#[doc(alias = "MODCFG")]
pub type Modcfg = crate::Reg<modcfg::ModcfgSpec>;
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "DICFG (rw) register accessor: Demodulator Input Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dicfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dicfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dicfg`]
module"]
#[doc(alias = "DICFG")]
pub type Dicfg = crate::Reg<dicfg::DicfgSpec>;
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "FCFGC (rw) register accessor: Filter Configuration Register, Main CIC Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfgc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfgc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfgc`]
module"]
#[doc(alias = "FCFGC")]
pub type Fcfgc = crate::Reg<fcfgc::FcfgcSpec>;
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "FCFGA (rw) register accessor: Filter Configuration Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfga::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfga::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcfga`]
module"]
#[doc(alias = "FCFGA")]
pub type Fcfga = crate::Reg<fcfga::FcfgaSpec>;
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "IWCTR (rw) register accessor: Integration Window Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwctr`]
module"]
#[doc(alias = "IWCTR")]
pub type Iwctr = crate::Reg<iwctr::IwctrSpec>;
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "BOUNDSEL (rw) register accessor: Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boundsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boundsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boundsel`]
module"]
#[doc(alias = "BOUNDSEL")]
pub type Boundsel = crate::Reg<boundsel::BoundselSpec>;
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "RESM (r) register accessor: Result Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resm`]
module"]
#[doc(alias = "RESM")]
pub type Resm = crate::Reg<resm::ResmSpec>;
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "OFFM (rw) register accessor: Offset Register, Main Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`offm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`offm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@offm`]
module"]
#[doc(alias = "OFFM")]
pub type Offm = crate::Reg<offm::OffmSpec>;
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "RESA (r) register accessor: Result Register, Auxiliary Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resa::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resa`]
module"]
#[doc(alias = "RESA")]
pub type Resa = crate::Reg<resa::ResaSpec>;
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "TSTMP (r) register accessor: Time-Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstmp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tstmp`]
module"]
#[doc(alias = "TSTMP")]
pub type Tstmp = crate::Reg<tstmp::TstmpSpec>;
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "CGSYNC (rw) register accessor: Carrier Generator Synchronization Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgsync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgsync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgsync`]
module"]
#[doc(alias = "CGSYNC")]
pub type Cgsync = crate::Reg<cgsync::CgsyncSpec>;
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "RECTCFG (rw) register accessor: Rectification Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rectcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rectcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rectcfg`]
module"]
#[doc(alias = "RECTCFG")]
pub type Rectcfg = crate::Reg<rectcfg::RectcfgSpec>;
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
