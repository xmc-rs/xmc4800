#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clc: CLC,
    _reserved1: [u8; 0x04],
    id: ID,
    _reserved2: [u8; 0x1c],
    ocs: OCS,
    _reserved3: [u8; 0x54],
    globcfg: GLOBCFG,
    _reserved4: [u8; 0x04],
    globrc: GLOBRC,
    _reserved5: [u8; 0x14],
    cgcfg: CGCFG,
    _reserved6: [u8; 0x3c],
    evflag: EVFLAG,
    evflagclr: EVFLAGCLR,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &CLC {
        &self.clc
    }
    #[doc = "0x08 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x28 - OCDS Control and Status Register"]
    #[inline(always)]
    pub const fn ocs(&self) -> &OCS {
        &self.ocs
    }
    #[doc = "0x80 - Global Configuration Register"]
    #[inline(always)]
    pub const fn globcfg(&self) -> &GLOBCFG {
        &self.globcfg
    }
    #[doc = "0x88 - Global Run Control Register"]
    #[inline(always)]
    pub const fn globrc(&self) -> &GLOBRC {
        &self.globrc
    }
    #[doc = "0xa0 - Carrier Generator Configuration Register"]
    #[inline(always)]
    pub const fn cgcfg(&self) -> &CGCFG {
        &self.cgcfg
    }
    #[doc = "0xe0 - Event Flag Register"]
    #[inline(always)]
    pub const fn evflag(&self) -> &EVFLAG {
        &self.evflag
    }
    #[doc = "0xe4 - Event Flag Clear Register"]
    #[inline(always)]
    pub const fn evflagclr(&self) -> &EVFLAGCLR {
        &self.evflagclr
    }
}
#[doc = "CLC (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCS (rw) register accessor: OCDS Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocs`]
module"]
pub type OCS = crate::Reg<ocs::OCS_SPEC>;
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "GLOBCFG (rw) register accessor: Global Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globcfg`]
module"]
pub type GLOBCFG = crate::Reg<globcfg::GLOBCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "GLOBRC (rw) register accessor: Global Run Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@globrc`]
module"]
pub type GLOBRC = crate::Reg<globrc::GLOBRC_SPEC>;
#[doc = "Global Run Control Register"]
pub mod globrc;
#[doc = "CGCFG (rw) register accessor: Carrier Generator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcfg`]
module"]
pub type CGCFG = crate::Reg<cgcfg::CGCFG_SPEC>;
#[doc = "Carrier Generator Configuration Register"]
pub mod cgcfg;
#[doc = "EVFLAG (rw) register accessor: Event Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflag`]
module"]
pub type EVFLAG = crate::Reg<evflag::EVFLAG_SPEC>;
#[doc = "Event Flag Register"]
pub mod evflag;
#[doc = "EVFLAGCLR (w) register accessor: Event Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflagclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflagclr`]
module"]
pub type EVFLAGCLR = crate::Reg<evflagclr::EVFLAGCLR_SPEC>;
#[doc = "Event Flag Clear Register"]
pub mod evflagclr;
