#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Modulator Configuration Register"]
    pub modcfg: MODCFG,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Demodulator Input Configuration Register"]
    pub dicfg: DICFG,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Filter Configuration Register, Main CIC Filter"]
    pub fcfgc: FCFGC,
    #[doc = "0x18 - Filter Configuration Register, Auxiliary Filter"]
    pub fcfga: FCFGA,
    _reserved4: [u8; 4usize],
    #[doc = "0x20 - Integration Window Control Register"]
    pub iwctr: IWCTR,
    _reserved5: [u8; 4usize],
    #[doc = "0x28 - Boundary Select Register"]
    pub boundsel: BOUNDSEL,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - Result Register, Main Filter"]
    pub resm: RESM,
    _reserved7: [u8; 4usize],
    #[doc = "0x38 - Offset Register, Main Filter"]
    pub offm: OFFM,
    _reserved8: [u8; 4usize],
    #[doc = "0x40 - Result Register, Auxiliary Filter"]
    pub resa: RESA,
    _reserved9: [u8; 12usize],
    #[doc = "0x50 - Time-Stamp Register"]
    pub tstmp: TSTMP,
    _reserved10: [u8; 76usize],
    #[doc = "0xa0 - Carrier Generator Synchronization Register"]
    pub cgsync: CGSYNC,
    _reserved11: [u8; 4usize],
    #[doc = "0xa8 - Rectification Configuration Register"]
    pub rectcfg: RECTCFG,
}
#[doc = "Modulator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modcfg](modcfg) module"]
pub type MODCFG = crate::Reg<u32, _MODCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODCFG;
#[doc = "`read()` method returns [modcfg::R](modcfg::R) reader structure"]
impl crate::Readable for MODCFG {}
#[doc = "`write(|w| ..)` method takes [modcfg::W](modcfg::W) writer structure"]
impl crate::Writable for MODCFG {}
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "Demodulator Input Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dicfg](dicfg) module"]
pub type DICFG = crate::Reg<u32, _DICFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DICFG;
#[doc = "`read()` method returns [dicfg::R](dicfg::R) reader structure"]
impl crate::Readable for DICFG {}
#[doc = "`write(|w| ..)` method takes [dicfg::W](dicfg::W) writer structure"]
impl crate::Writable for DICFG {}
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "Filter Configuration Register, Main CIC Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcfgc](fcfgc) module"]
pub type FCFGC = crate::Reg<u32, _FCFGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFGC;
#[doc = "`read()` method returns [fcfgc::R](fcfgc::R) reader structure"]
impl crate::Readable for FCFGC {}
#[doc = "`write(|w| ..)` method takes [fcfgc::W](fcfgc::W) writer structure"]
impl crate::Writable for FCFGC {}
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "Filter Configuration Register, Auxiliary Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcfga](fcfga) module"]
pub type FCFGA = crate::Reg<u32, _FCFGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFGA;
#[doc = "`read()` method returns [fcfga::R](fcfga::R) reader structure"]
impl crate::Readable for FCFGA {}
#[doc = "`write(|w| ..)` method takes [fcfga::W](fcfga::W) writer structure"]
impl crate::Writable for FCFGA {}
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "Integration Window Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iwctr](iwctr) module"]
pub type IWCTR = crate::Reg<u32, _IWCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWCTR;
#[doc = "`read()` method returns [iwctr::R](iwctr::R) reader structure"]
impl crate::Readable for IWCTR {}
#[doc = "`write(|w| ..)` method takes [iwctr::W](iwctr::W) writer structure"]
impl crate::Writable for IWCTR {}
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "Boundary Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [boundsel](boundsel) module"]
pub type BOUNDSEL = crate::Reg<u32, _BOUNDSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOUNDSEL;
#[doc = "`read()` method returns [boundsel::R](boundsel::R) reader structure"]
impl crate::Readable for BOUNDSEL {}
#[doc = "`write(|w| ..)` method takes [boundsel::W](boundsel::W) writer structure"]
impl crate::Writable for BOUNDSEL {}
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "Result Register, Main Filter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resm](resm) module"]
pub type RESM = crate::Reg<u32, _RESM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESM;
#[doc = "`read()` method returns [resm::R](resm::R) reader structure"]
impl crate::Readable for RESM {}
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "Offset Register, Main Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [offm](offm) module"]
pub type OFFM = crate::Reg<u32, _OFFM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFFM;
#[doc = "`read()` method returns [offm::R](offm::R) reader structure"]
impl crate::Readable for OFFM {}
#[doc = "`write(|w| ..)` method takes [offm::W](offm::W) writer structure"]
impl crate::Writable for OFFM {}
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "Result Register, Auxiliary Filter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resa](resa) module"]
pub type RESA = crate::Reg<u32, _RESA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESA;
#[doc = "`read()` method returns [resa::R](resa::R) reader structure"]
impl crate::Readable for RESA {}
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "Time-Stamp Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tstmp](tstmp) module"]
pub type TSTMP = crate::Reg<u32, _TSTMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTMP;
#[doc = "`read()` method returns [tstmp::R](tstmp::R) reader structure"]
impl crate::Readable for TSTMP {}
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "Carrier Generator Synchronization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgsync](cgsync) module"]
pub type CGSYNC = crate::Reg<u32, _CGSYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGSYNC;
#[doc = "`read()` method returns [cgsync::R](cgsync::R) reader structure"]
impl crate::Readable for CGSYNC {}
#[doc = "`write(|w| ..)` method takes [cgsync::W](cgsync::W) writer structure"]
impl crate::Writable for CGSYNC {}
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "Rectification Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rectcfg](rectcfg) module"]
pub type RECTCFG = crate::Reg<u32, _RECTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECTCFG;
#[doc = "`read()` method returns [rectcfg::R](rectcfg::R) reader structure"]
impl crate::Readable for RECTCFG {}
#[doc = "`write(|w| ..)` method takes [rectcfg::W](rectcfg::W) writer structure"]
impl crate::Writable for RECTCFG {}
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
