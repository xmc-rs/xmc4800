#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved2: [u8; 28usize],
    #[doc = "0x28 - OCDS Control and Status Register"]
    pub ocs: OCS,
    _reserved3: [u8; 84usize],
    #[doc = "0x80 - Global Configuration Register"]
    pub globcfg: GLOBCFG,
    _reserved4: [u8; 4usize],
    #[doc = "0x88 - Global Run Control Register"]
    pub globrc: GLOBRC,
    _reserved5: [u8; 20usize],
    #[doc = "0xa0 - Carrier Generator Configuration Register"]
    pub cgcfg: CGCFG,
    _reserved6: [u8; 60usize],
    #[doc = "0xe0 - Event Flag Register"]
    pub evflag: EVFLAG,
    #[doc = "0xe4 - Event Flag Clear Register"]
    pub evflagclr: EVFLAGCLR,
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](clc) module"]
pub type CLC = crate::Reg<u32, _CLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLC;
#[doc = "`read()` method returns [clc::R](clc::R) reader structure"]
impl crate::Readable for CLC {}
#[doc = "`write(|w| ..)` method takes [clc::W](clc::W) writer structure"]
impl crate::Writable for CLC {}
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCDS Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocs](ocs) module"]
pub type OCS = crate::Reg<u32, _OCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCS;
#[doc = "`read()` method returns [ocs::R](ocs::R) reader structure"]
impl crate::Readable for OCS {}
#[doc = "`write(|w| ..)` method takes [ocs::W](ocs::W) writer structure"]
impl crate::Writable for OCS {}
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globcfg](globcfg) module"]
pub type GLOBCFG = crate::Reg<u32, _GLOBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBCFG;
#[doc = "`read()` method returns [globcfg::R](globcfg::R) reader structure"]
impl crate::Readable for GLOBCFG {}
#[doc = "`write(|w| ..)` method takes [globcfg::W](globcfg::W) writer structure"]
impl crate::Writable for GLOBCFG {}
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "Global Run Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globrc](globrc) module"]
pub type GLOBRC = crate::Reg<u32, _GLOBRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLOBRC;
#[doc = "`read()` method returns [globrc::R](globrc::R) reader structure"]
impl crate::Readable for GLOBRC {}
#[doc = "`write(|w| ..)` method takes [globrc::W](globrc::W) writer structure"]
impl crate::Writable for GLOBRC {}
#[doc = "Global Run Control Register"]
pub mod globrc;
#[doc = "Carrier Generator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgcfg](cgcfg) module"]
pub type CGCFG = crate::Reg<u32, _CGCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGCFG;
#[doc = "`read()` method returns [cgcfg::R](cgcfg::R) reader structure"]
impl crate::Readable for CGCFG {}
#[doc = "`write(|w| ..)` method takes [cgcfg::W](cgcfg::W) writer structure"]
impl crate::Writable for CGCFG {}
#[doc = "Carrier Generator Configuration Register"]
pub mod cgcfg;
#[doc = "Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflag](evflag) module"]
pub type EVFLAG = crate::Reg<u32, _EVFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFLAG;
#[doc = "`read()` method returns [evflag::R](evflag::R) reader structure"]
impl crate::Readable for EVFLAG {}
#[doc = "`write(|w| ..)` method takes [evflag::W](evflag::W) writer structure"]
impl crate::Writable for EVFLAG {}
#[doc = "Event Flag Register"]
pub mod evflag;
#[doc = "Event Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflagclr](evflagclr) module"]
pub type EVFLAGCLR = crate::Reg<u32, _EVFLAGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFLAGCLR;
#[doc = "`write(|w| ..)` method takes [evflagclr::W](evflagclr::W) writer structure"]
impl crate::Writable for EVFLAGCLR {}
#[doc = "Event Flag Clear Register"]
pub mod evflagclr;
