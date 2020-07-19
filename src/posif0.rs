#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POSIF configuration"]
    pub pconf: PCONF,
    #[doc = "0x04 - POSIF Suspend Config"]
    pub psus: PSUS,
    #[doc = "0x08 - POSIF Run Bit Set"]
    pub pruns: PRUNS,
    #[doc = "0x0c - POSIF Run Bit Clear"]
    pub prunc: PRUNC,
    #[doc = "0x10 - POSIF Run Bit Status"]
    pub prun: PRUN,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - Module Identification register"]
    pub midr: MIDR,
    _reserved6: [u8; 12usize],
    #[doc = "0x30 - Hall Sensor Patterns"]
    pub halp: HALP,
    #[doc = "0x34 - Hall Sensor Shadow Patterns"]
    pub halps: HALPS,
    _reserved8: [u8; 8usize],
    #[doc = "0x40 - Multi-Channel Pattern"]
    pub mcm: MCM,
    #[doc = "0x44 - Multi-Channel Shadow Pattern"]
    pub mcsm: MCSM,
    #[doc = "0x48 - Multi-Channel Pattern Control set"]
    pub mcms: MCMS,
    #[doc = "0x4c - Multi-Channel Pattern Control clear"]
    pub mcmc: MCMC,
    #[doc = "0x50 - Multi-Channel Pattern Control flag"]
    pub mcmf: MCMF,
    _reserved13: [u8; 12usize],
    #[doc = "0x60 - Quadrature Decoder Control"]
    pub qdc: QDC,
    _reserved14: [u8; 12usize],
    #[doc = "0x70 - POSIF Interrupt Flags"]
    pub pflg: PFLG,
    #[doc = "0x74 - POSIF Interrupt Enable"]
    pub pflge: PFLGE,
    #[doc = "0x78 - POSIF Interrupt Set"]
    pub spflg: SPFLG,
    #[doc = "0x7c - POSIF Interrupt Clear"]
    pub rpflg: RPFLG,
    _reserved18: [u8; 128usize],
    #[doc = "0x100 - POSIF Debug register"]
    pub pdbg: PDBG,
}
#[doc = "POSIF configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconf](pconf) module"]
pub type PCONF = crate::Reg<u32, _PCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCONF;
#[doc = "`read()` method returns [pconf::R](pconf::R) reader structure"]
impl crate::Readable for PCONF {}
#[doc = "`write(|w| ..)` method takes [pconf::W](pconf::W) writer structure"]
impl crate::Writable for PCONF {}
#[doc = "POSIF configuration"]
pub mod pconf;
#[doc = "POSIF Suspend Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psus](psus) module"]
pub type PSUS = crate::Reg<u32, _PSUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSUS;
#[doc = "`read()` method returns [psus::R](psus::R) reader structure"]
impl crate::Readable for PSUS {}
#[doc = "`write(|w| ..)` method takes [psus::W](psus::W) writer structure"]
impl crate::Writable for PSUS {}
#[doc = "POSIF Suspend Config"]
pub mod psus;
#[doc = "POSIF Run Bit Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pruns](pruns) module"]
pub type PRUNS = crate::Reg<u32, _PRUNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUNS;
#[doc = "`write(|w| ..)` method takes [pruns::W](pruns::W) writer structure"]
impl crate::Writable for PRUNS {}
#[doc = "POSIF Run Bit Set"]
pub mod pruns;
#[doc = "POSIF Run Bit Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prunc](prunc) module"]
pub type PRUNC = crate::Reg<u32, _PRUNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUNC;
#[doc = "`write(|w| ..)` method takes [prunc::W](prunc::W) writer structure"]
impl crate::Writable for PRUNC {}
#[doc = "POSIF Run Bit Clear"]
pub mod prunc;
#[doc = "POSIF Run Bit Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prun](prun) module"]
pub type PRUN = crate::Reg<u32, _PRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRUN;
#[doc = "`read()` method returns [prun::R](prun::R) reader structure"]
impl crate::Readable for PRUN {}
#[doc = "POSIF Run Bit Status"]
pub mod prun;
#[doc = "Module Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midr](midr) module"]
pub type MIDR = crate::Reg<u32, _MIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIDR;
#[doc = "`read()` method returns [midr::R](midr::R) reader structure"]
impl crate::Readable for MIDR {}
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "Hall Sensor Patterns\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halp](halp) module"]
pub type HALP = crate::Reg<u32, _HALP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HALP;
#[doc = "`read()` method returns [halp::R](halp::R) reader structure"]
impl crate::Readable for HALP {}
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "Hall Sensor Shadow Patterns\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halps](halps) module"]
pub type HALPS = crate::Reg<u32, _HALPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HALPS;
#[doc = "`read()` method returns [halps::R](halps::R) reader structure"]
impl crate::Readable for HALPS {}
#[doc = "`write(|w| ..)` method takes [halps::W](halps::W) writer structure"]
impl crate::Writable for HALPS {}
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "Multi-Channel Pattern\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcm](mcm) module"]
pub type MCM = crate::Reg<u32, _MCM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCM;
#[doc = "`read()` method returns [mcm::R](mcm::R) reader structure"]
impl crate::Readable for MCM {}
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "Multi-Channel Shadow Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcsm](mcsm) module"]
pub type MCSM = crate::Reg<u32, _MCSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCSM;
#[doc = "`read()` method returns [mcsm::R](mcsm::R) reader structure"]
impl crate::Readable for MCSM {}
#[doc = "`write(|w| ..)` method takes [mcsm::W](mcsm::W) writer structure"]
impl crate::Writable for MCSM {}
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "Multi-Channel Pattern Control set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcms](mcms) module"]
pub type MCMS = crate::Reg<u32, _MCMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMS;
#[doc = "`write(|w| ..)` method takes [mcms::W](mcms::W) writer structure"]
impl crate::Writable for MCMS {}
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "Multi-Channel Pattern Control clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmc](mcmc) module"]
pub type MCMC = crate::Reg<u32, _MCMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMC;
#[doc = "`write(|w| ..)` method takes [mcmc::W](mcmc::W) writer structure"]
impl crate::Writable for MCMC {}
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "Multi-Channel Pattern Control flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmf](mcmf) module"]
pub type MCMF = crate::Reg<u32, _MCMF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMF;
#[doc = "`read()` method returns [mcmf::R](mcmf::R) reader structure"]
impl crate::Readable for MCMF {}
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "Quadrature Decoder Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdc](qdc) module"]
pub type QDC = crate::Reg<u32, _QDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDC;
#[doc = "`read()` method returns [qdc::R](qdc::R) reader structure"]
impl crate::Readable for QDC {}
#[doc = "`write(|w| ..)` method takes [qdc::W](qdc::W) writer structure"]
impl crate::Writable for QDC {}
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "POSIF Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pflg](pflg) module"]
pub type PFLG = crate::Reg<u32, _PFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFLG;
#[doc = "`read()` method returns [pflg::R](pflg::R) reader structure"]
impl crate::Readable for PFLG {}
#[doc = "POSIF Interrupt Flags"]
pub mod pflg;
#[doc = "POSIF Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pflge](pflge) module"]
pub type PFLGE = crate::Reg<u32, _PFLGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFLGE;
#[doc = "`read()` method returns [pflge::R](pflge::R) reader structure"]
impl crate::Readable for PFLGE {}
#[doc = "`write(|w| ..)` method takes [pflge::W](pflge::W) writer structure"]
impl crate::Writable for PFLGE {}
#[doc = "POSIF Interrupt Enable"]
pub mod pflge;
#[doc = "POSIF Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spflg](spflg) module"]
pub type SPFLG = crate::Reg<u32, _SPFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPFLG;
#[doc = "`write(|w| ..)` method takes [spflg::W](spflg::W) writer structure"]
impl crate::Writable for SPFLG {}
#[doc = "POSIF Interrupt Set"]
pub mod spflg;
#[doc = "POSIF Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpflg](rpflg) module"]
pub type RPFLG = crate::Reg<u32, _RPFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPFLG;
#[doc = "`write(|w| ..)` method takes [rpflg::W](rpflg::W) writer structure"]
impl crate::Writable for RPFLG {}
#[doc = "POSIF Interrupt Clear"]
pub mod rpflg;
#[doc = "POSIF Debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdbg](pdbg) module"]
pub type PDBG = crate::Reg<u32, _PDBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDBG;
#[doc = "`read()` method returns [pdbg::R](pdbg::R) reader structure"]
impl crate::Readable for PDBG {}
#[doc = "POSIF Debug register"]
pub mod pdbg;
