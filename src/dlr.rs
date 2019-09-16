#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Overrun Status"]
    pub ovrstat: OVRSTAT,
    #[doc = "0x04 - Overrun Clear"]
    pub ovrclr: OVRCLR,
    #[doc = "0x08 - Service Request Selection 0"]
    pub srsel0: SRSEL0,
    #[doc = "0x0c - Service Request Selection 1"]
    pub srsel1: SRSEL1,
    #[doc = "0x10 - Line Enable"]
    pub lnen: LNEN,
}
#[doc = "Overrun Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ovrstat](ovrstat) module"]
pub type OVRSTAT = crate::Reg<u32, _OVRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVRSTAT;
#[doc = "`read()` method returns [ovrstat::R](ovrstat::R) reader structure"]
impl crate::Readable for OVRSTAT {}
#[doc = "Overrun Status"]
pub mod ovrstat;
#[doc = "Overrun Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ovrclr](ovrclr) module"]
pub type OVRCLR = crate::Reg<u32, _OVRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVRCLR;
#[doc = "`write(|w| ..)` method takes [ovrclr::W](ovrclr::W) writer structure"]
impl crate::Writable for OVRCLR {}
#[doc = "Overrun Clear"]
pub mod ovrclr;
#[doc = "Service Request Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srsel0](srsel0) module"]
pub type SRSEL0 = crate::Reg<u32, _SRSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSEL0;
#[doc = "`read()` method returns [srsel0::R](srsel0::R) reader structure"]
impl crate::Readable for SRSEL0 {}
#[doc = "`write(|w| ..)` method takes [srsel0::W](srsel0::W) writer structure"]
impl crate::Writable for SRSEL0 {}
#[doc = "Service Request Selection 0"]
pub mod srsel0;
#[doc = "Service Request Selection 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srsel1](srsel1) module"]
pub type SRSEL1 = crate::Reg<u32, _SRSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSEL1;
#[doc = "`read()` method returns [srsel1::R](srsel1::R) reader structure"]
impl crate::Readable for SRSEL1 {}
#[doc = "`write(|w| ..)` method takes [srsel1::W](srsel1::W) writer structure"]
impl crate::Writable for SRSEL1 {}
#[doc = "Service Request Selection 1"]
pub mod srsel1;
#[doc = "Line Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lnen](lnen) module"]
pub type LNEN = crate::Reg<u32, _LNEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LNEN;
#[doc = "`read()` method returns [lnen::R](lnen::R) reader structure"]
impl crate::Readable for LNEN {}
#[doc = "`write(|w| ..)` method takes [lnen::W](lnen::W) writer structure"]
impl crate::Writable for LNEN {}
#[doc = "Line Enable"]
pub mod lnen;
