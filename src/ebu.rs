#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBU Clock Control Register"]
    pub clc: CLC,
    #[doc = "0x04 - EBU Configuration Register"]
    pub modcon: MODCON,
    #[doc = "0x08 - EBU Module Identification Register"]
    pub id: ID,
    #[doc = "0x0c - EBU Test/Control Configuration Register"]
    pub usercon: USERCON,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - EBU Address Select Register 0"]
    pub addrsel0: ADDRSEL0,
    #[doc = "0x1c - EBU Address Select Register 1"]
    pub addrsel1: ADDRSEL1,
    #[doc = "0x20 - EBU Address Select Register 2"]
    pub addrsel2: ADDRSEL2,
    #[doc = "0x24 - EBU Address Select Register 3"]
    pub addrsel3: ADDRSEL3,
    #[doc = "0x28 - EBU Bus Configuration Register"]
    pub busrcon0: BUSRCON0,
    #[doc = "0x2c - EBU Bus Read Access Parameter Register"]
    pub busrap0: BUSRAP0,
    #[doc = "0x30 - EBU Bus Write Configuration Register"]
    pub buswcon0: BUSWCON0,
    #[doc = "0x34 - EBU Bus Write Access Parameter Register"]
    pub buswap0: BUSWAP0,
    #[doc = "0x38 - EBU Bus Configuration Register"]
    pub busrcon1: BUSRCON1,
    #[doc = "0x3c - EBU Bus Read Access Parameter Register"]
    pub busrap1: BUSRAP1,
    #[doc = "0x40 - EBU Bus Write Configuration Register"]
    pub buswcon1: BUSWCON1,
    #[doc = "0x44 - EBU Bus Write Access Parameter Register"]
    pub buswap1: BUSWAP1,
    #[doc = "0x48 - EBU Bus Configuration Register"]
    pub busrcon2: BUSRCON2,
    #[doc = "0x4c - EBU Bus Read Access Parameter Register"]
    pub busrap2: BUSRAP2,
    #[doc = "0x50 - EBU Bus Write Configuration Register"]
    pub buswcon2: BUSWCON2,
    #[doc = "0x54 - EBU Bus Write Access Parameter Register"]
    pub buswap2: BUSWAP2,
    #[doc = "0x58 - EBU Bus Configuration Register"]
    pub busrcon3: BUSRCON3,
    #[doc = "0x5c - EBU Bus Read Access Parameter Register"]
    pub busrap3: BUSRAP3,
    #[doc = "0x60 - EBU Bus Write Configuration Register"]
    pub buswcon3: BUSWCON3,
    #[doc = "0x64 - EBU Bus Write Access Parameter Register"]
    pub buswap3: BUSWAP3,
    #[doc = "0x68 - EBU SDRAM Control Register"]
    pub sdrmcon: SDRMCON,
    #[doc = "0x6c - EBU SDRAM Mode Register"]
    pub sdrmod: SDRMOD,
    #[doc = "0x70 - EBU SDRAM Refresh Control Register"]
    pub sdrmref: SDRMREF,
    #[doc = "0x74 - EBU SDRAM Status Register"]
    pub sdrstat: SDRSTAT,
}
#[doc = "EBU Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clc](clc) module"]
pub type CLC = crate::Reg<u32, _CLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLC;
#[doc = "`read()` method returns [clc::R](clc::R) reader structure"]
impl crate::Readable for CLC {}
#[doc = "`write(|w| ..)` method takes [clc::W](clc::W) writer structure"]
impl crate::Writable for CLC {}
#[doc = "EBU Clock Control Register"]
pub mod clc;
#[doc = "EBU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modcon](modcon) module"]
pub type MODCON = crate::Reg<u32, _MODCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODCON;
#[doc = "`read()` method returns [modcon::R](modcon::R) reader structure"]
impl crate::Readable for MODCON {}
#[doc = "`write(|w| ..)` method takes [modcon::W](modcon::W) writer structure"]
impl crate::Writable for MODCON {}
#[doc = "EBU Configuration Register"]
pub mod modcon;
#[doc = "EBU Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "EBU Module Identification Register"]
pub mod id;
#[doc = "EBU Test/Control Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usercon](usercon) module"]
pub type USERCON = crate::Reg<u32, _USERCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERCON;
#[doc = "`read()` method returns [usercon::R](usercon::R) reader structure"]
impl crate::Readable for USERCON {}
#[doc = "`write(|w| ..)` method takes [usercon::W](usercon::W) writer structure"]
impl crate::Writable for USERCON {}
#[doc = "EBU Test/Control Configuration Register"]
pub mod usercon;
#[doc = "EBU Address Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrsel0](addrsel0) module"]
pub type ADDRSEL0 = crate::Reg<u32, _ADDRSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRSEL0;
#[doc = "`read()` method returns [addrsel0::R](addrsel0::R) reader structure"]
impl crate::Readable for ADDRSEL0 {}
#[doc = "`write(|w| ..)` method takes [addrsel0::W](addrsel0::W) writer structure"]
impl crate::Writable for ADDRSEL0 {}
#[doc = "EBU Address Select Register 0"]
pub mod addrsel0;
#[doc = "EBU Address Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrsel1](addrsel1) module"]
pub type ADDRSEL1 = crate::Reg<u32, _ADDRSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRSEL1;
#[doc = "`read()` method returns [addrsel1::R](addrsel1::R) reader structure"]
impl crate::Readable for ADDRSEL1 {}
#[doc = "`write(|w| ..)` method takes [addrsel1::W](addrsel1::W) writer structure"]
impl crate::Writable for ADDRSEL1 {}
#[doc = "EBU Address Select Register 1"]
pub mod addrsel1;
#[doc = "EBU Address Select Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrsel2](addrsel2) module"]
pub type ADDRSEL2 = crate::Reg<u32, _ADDRSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRSEL2;
#[doc = "`read()` method returns [addrsel2::R](addrsel2::R) reader structure"]
impl crate::Readable for ADDRSEL2 {}
#[doc = "`write(|w| ..)` method takes [addrsel2::W](addrsel2::W) writer structure"]
impl crate::Writable for ADDRSEL2 {}
#[doc = "EBU Address Select Register 2"]
pub mod addrsel2;
#[doc = "EBU Address Select Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrsel3](addrsel3) module"]
pub type ADDRSEL3 = crate::Reg<u32, _ADDRSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRSEL3;
#[doc = "`read()` method returns [addrsel3::R](addrsel3::R) reader structure"]
impl crate::Readable for ADDRSEL3 {}
#[doc = "`write(|w| ..)` method takes [addrsel3::W](addrsel3::W) writer structure"]
impl crate::Writable for ADDRSEL3 {}
#[doc = "EBU Address Select Register 3"]
pub mod addrsel3;
#[doc = "EBU Bus Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrcon0](busrcon0) module"]
pub type BUSRCON0 = crate::Reg<u32, _BUSRCON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRCON0;
#[doc = "`read()` method returns [busrcon0::R](busrcon0::R) reader structure"]
impl crate::Readable for BUSRCON0 {}
#[doc = "`write(|w| ..)` method takes [busrcon0::W](busrcon0::W) writer structure"]
impl crate::Writable for BUSRCON0 {}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon0;
#[doc = "EBU Bus Read Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrap0](busrap0) module"]
pub type BUSRAP0 = crate::Reg<u32, _BUSRAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRAP0;
#[doc = "`read()` method returns [busrap0::R](busrap0::R) reader structure"]
impl crate::Readable for BUSRAP0 {}
#[doc = "`write(|w| ..)` method takes [busrap0::W](busrap0::W) writer structure"]
impl crate::Writable for BUSRAP0 {}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap0;
#[doc = "EBU Bus Write Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswcon0](buswcon0) module"]
pub type BUSWCON0 = crate::Reg<u32, _BUSWCON0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWCON0;
#[doc = "`read()` method returns [buswcon0::R](buswcon0::R) reader structure"]
impl crate::Readable for BUSWCON0 {}
#[doc = "`write(|w| ..)` method takes [buswcon0::W](buswcon0::W) writer structure"]
impl crate::Writable for BUSWCON0 {}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon0;
#[doc = "EBU Bus Write Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswap0](buswap0) module"]
pub type BUSWAP0 = crate::Reg<u32, _BUSWAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWAP0;
#[doc = "`read()` method returns [buswap0::R](buswap0::R) reader structure"]
impl crate::Readable for BUSWAP0 {}
#[doc = "`write(|w| ..)` method takes [buswap0::W](buswap0::W) writer structure"]
impl crate::Writable for BUSWAP0 {}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap0;
#[doc = "EBU Bus Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrcon1](busrcon1) module"]
pub type BUSRCON1 = crate::Reg<u32, _BUSRCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRCON1;
#[doc = "`read()` method returns [busrcon1::R](busrcon1::R) reader structure"]
impl crate::Readable for BUSRCON1 {}
#[doc = "`write(|w| ..)` method takes [busrcon1::W](busrcon1::W) writer structure"]
impl crate::Writable for BUSRCON1 {}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon1;
#[doc = "EBU Bus Read Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrap1](busrap1) module"]
pub type BUSRAP1 = crate::Reg<u32, _BUSRAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRAP1;
#[doc = "`read()` method returns [busrap1::R](busrap1::R) reader structure"]
impl crate::Readable for BUSRAP1 {}
#[doc = "`write(|w| ..)` method takes [busrap1::W](busrap1::W) writer structure"]
impl crate::Writable for BUSRAP1 {}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap1;
#[doc = "EBU Bus Write Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswcon1](buswcon1) module"]
pub type BUSWCON1 = crate::Reg<u32, _BUSWCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWCON1;
#[doc = "`read()` method returns [buswcon1::R](buswcon1::R) reader structure"]
impl crate::Readable for BUSWCON1 {}
#[doc = "`write(|w| ..)` method takes [buswcon1::W](buswcon1::W) writer structure"]
impl crate::Writable for BUSWCON1 {}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon1;
#[doc = "EBU Bus Write Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswap1](buswap1) module"]
pub type BUSWAP1 = crate::Reg<u32, _BUSWAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWAP1;
#[doc = "`read()` method returns [buswap1::R](buswap1::R) reader structure"]
impl crate::Readable for BUSWAP1 {}
#[doc = "`write(|w| ..)` method takes [buswap1::W](buswap1::W) writer structure"]
impl crate::Writable for BUSWAP1 {}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap1;
#[doc = "EBU Bus Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrcon2](busrcon2) module"]
pub type BUSRCON2 = crate::Reg<u32, _BUSRCON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRCON2;
#[doc = "`read()` method returns [busrcon2::R](busrcon2::R) reader structure"]
impl crate::Readable for BUSRCON2 {}
#[doc = "`write(|w| ..)` method takes [busrcon2::W](busrcon2::W) writer structure"]
impl crate::Writable for BUSRCON2 {}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon2;
#[doc = "EBU Bus Read Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrap2](busrap2) module"]
pub type BUSRAP2 = crate::Reg<u32, _BUSRAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRAP2;
#[doc = "`read()` method returns [busrap2::R](busrap2::R) reader structure"]
impl crate::Readable for BUSRAP2 {}
#[doc = "`write(|w| ..)` method takes [busrap2::W](busrap2::W) writer structure"]
impl crate::Writable for BUSRAP2 {}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap2;
#[doc = "EBU Bus Write Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswcon2](buswcon2) module"]
pub type BUSWCON2 = crate::Reg<u32, _BUSWCON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWCON2;
#[doc = "`read()` method returns [buswcon2::R](buswcon2::R) reader structure"]
impl crate::Readable for BUSWCON2 {}
#[doc = "`write(|w| ..)` method takes [buswcon2::W](buswcon2::W) writer structure"]
impl crate::Writable for BUSWCON2 {}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon2;
#[doc = "EBU Bus Write Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswap2](buswap2) module"]
pub type BUSWAP2 = crate::Reg<u32, _BUSWAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWAP2;
#[doc = "`read()` method returns [buswap2::R](buswap2::R) reader structure"]
impl crate::Readable for BUSWAP2 {}
#[doc = "`write(|w| ..)` method takes [buswap2::W](buswap2::W) writer structure"]
impl crate::Writable for BUSWAP2 {}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap2;
#[doc = "EBU Bus Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrcon3](busrcon3) module"]
pub type BUSRCON3 = crate::Reg<u32, _BUSRCON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRCON3;
#[doc = "`read()` method returns [busrcon3::R](busrcon3::R) reader structure"]
impl crate::Readable for BUSRCON3 {}
#[doc = "`write(|w| ..)` method takes [busrcon3::W](busrcon3::W) writer structure"]
impl crate::Writable for BUSRCON3 {}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon3;
#[doc = "EBU Bus Read Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrap3](busrap3) module"]
pub type BUSRAP3 = crate::Reg<u32, _BUSRAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSRAP3;
#[doc = "`read()` method returns [busrap3::R](busrap3::R) reader structure"]
impl crate::Readable for BUSRAP3 {}
#[doc = "`write(|w| ..)` method takes [busrap3::W](busrap3::W) writer structure"]
impl crate::Writable for BUSRAP3 {}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap3;
#[doc = "EBU Bus Write Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswcon3](buswcon3) module"]
pub type BUSWCON3 = crate::Reg<u32, _BUSWCON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWCON3;
#[doc = "`read()` method returns [buswcon3::R](buswcon3::R) reader structure"]
impl crate::Readable for BUSWCON3 {}
#[doc = "`write(|w| ..)` method takes [buswcon3::W](buswcon3::W) writer structure"]
impl crate::Writable for BUSWCON3 {}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon3;
#[doc = "EBU Bus Write Access Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buswap3](buswap3) module"]
pub type BUSWAP3 = crate::Reg<u32, _BUSWAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUSWAP3;
#[doc = "`read()` method returns [buswap3::R](buswap3::R) reader structure"]
impl crate::Readable for BUSWAP3 {}
#[doc = "`write(|w| ..)` method takes [buswap3::W](buswap3::W) writer structure"]
impl crate::Writable for BUSWAP3 {}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap3;
#[doc = "EBU SDRAM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrmcon](sdrmcon) module"]
pub type SDRMCON = crate::Reg<u32, _SDRMCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRMCON;
#[doc = "`read()` method returns [sdrmcon::R](sdrmcon::R) reader structure"]
impl crate::Readable for SDRMCON {}
#[doc = "`write(|w| ..)` method takes [sdrmcon::W](sdrmcon::W) writer structure"]
impl crate::Writable for SDRMCON {}
#[doc = "EBU SDRAM Control Register"]
pub mod sdrmcon;
#[doc = "EBU SDRAM Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrmod](sdrmod) module"]
pub type SDRMOD = crate::Reg<u32, _SDRMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRMOD;
#[doc = "`read()` method returns [sdrmod::R](sdrmod::R) reader structure"]
impl crate::Readable for SDRMOD {}
#[doc = "`write(|w| ..)` method takes [sdrmod::W](sdrmod::W) writer structure"]
impl crate::Writable for SDRMOD {}
#[doc = "EBU SDRAM Mode Register"]
pub mod sdrmod;
#[doc = "EBU SDRAM Refresh Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrmref](sdrmref) module"]
pub type SDRMREF = crate::Reg<u32, _SDRMREF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRMREF;
#[doc = "`read()` method returns [sdrmref::R](sdrmref::R) reader structure"]
impl crate::Readable for SDRMREF {}
#[doc = "`write(|w| ..)` method takes [sdrmref::W](sdrmref::W) writer structure"]
impl crate::Writable for SDRMREF {}
#[doc = "EBU SDRAM Refresh Control Register"]
pub mod sdrmref;
#[doc = "EBU SDRAM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrstat](sdrstat) module"]
pub type SDRSTAT = crate::Reg<u32, _SDRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRSTAT;
#[doc = "`read()` method returns [sdrstat::R](sdrstat::R) reader structure"]
impl crate::Readable for SDRSTAT {}
#[doc = "EBU SDRAM Status Register"]
pub mod sdrstat;
