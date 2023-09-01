#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_diepctl: [u8; 0x04],
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Device Endpoint Interrupt Register"]
    pub diepint: DIEPINT,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Device Endpoint Transfer Size Register"]
    pub dieptsiz: DIEPTSIZ,
    #[doc = "0x14 - Device Endpoint DMA Address Register"]
    pub diepdma: DIEPDMA,
    #[doc = "0x18 - Device IN Endpoint Transmit FIFO Status Register"]
    pub dtxfsts: DTXFSTS,
    #[doc = "0x1c - Device Endpoint DMA Buffer Address Register"]
    pub diepdmab: DIEPDMAB,
    _reserved6: [u8; 0x01e0],
    _reserved_6_doepctl: [u8; 0x04],
    _reserved7: [u8; 0x04],
    #[doc = "0x208 - Device Endpoint Interrupt Register"]
    pub doepint: DOEPINT,
    _reserved8: [u8; 0x04],
    _reserved_8_doeptsiz: [u8; 0x04],
    #[doc = "0x214 - Device Endpoint DMA Address Register"]
    pub doepdma: DOEPDMA,
    _reserved10: [u8; 0x04],
    #[doc = "0x21c - Device Endpoint DMA Buffer Address Register"]
    pub doepdmab: DOEPDMAB,
}
impl RegisterBlock {
    #[doc = "0x00 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub const fn diepctl_intbulk(&self) -> &DIEPCTL_INTBULK {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub const fn diepctl_isocont(&self) -> &DIEPCTL_ISOCONT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[INTBULK\\]"]
    #[inline(always)]
    pub const fn doepctl_intbulk(&self) -> &DOEPCTL_INTBULK {
        unsafe { &*(self as *const Self).cast::<u8>().add(512usize).cast() }
    }
    #[doc = "0x200 - Device Endpoint Control Register \\[ISOCONT\\]"]
    #[inline(always)]
    pub const fn doepctl_isocont(&self) -> &DOEPCTL_ISOCONT {
        unsafe { &*(self as *const Self).cast::<u8>().add(512usize).cast() }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[CONT\\]"]
    #[inline(always)]
    pub const fn doeptsiz_control(&self) -> &DOEPTSIZ_CONTROL {
        unsafe { &*(self as *const Self).cast::<u8>().add(528usize).cast() }
    }
    #[doc = "0x210 - Device Endpoint Transfer Size Register \\[ISO\\]"]
    #[inline(always)]
    pub const fn doeptsiz_iso(&self) -> &DOEPTSIZ_ISO {
        unsafe { &*(self as *const Self).cast::<u8>().add(528usize).cast() }
    }
}
#[doc = "DIEPCTL_ISOCONT (rw) register accessor: Device Endpoint Control Register \\[ISOCONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl_isocont::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl_isocont::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl_isocont`]
module"]
pub type DIEPCTL_ISOCONT = crate::Reg<diepctl_isocont::DIEPCTL_ISOCONT_SPEC>;
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod diepctl_isocont;
#[doc = "DIEPCTL_INTBULK (rw) register accessor: Device Endpoint Control Register \\[INTBULK\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl_intbulk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl_intbulk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepctl_intbulk`]
module"]
pub type DIEPCTL_INTBULK = crate::Reg<diepctl_intbulk::DIEPCTL_INTBULK_SPEC>;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod diepctl_intbulk;
#[doc = "DIEPINT (rw) register accessor: Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepint`]
module"]
pub type DIEPINT = crate::Reg<diepint::DIEPINT_SPEC>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod diepint;
#[doc = "DIEPTSIZ (rw) register accessor: Device Endpoint Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dieptsiz`]
module"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = "Device Endpoint Transfer Size Register"]
pub mod dieptsiz;
#[doc = "DIEPDMA (rw) register accessor: Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdma`]
module"]
pub type DIEPDMA = crate::Reg<diepdma::DIEPDMA_SPEC>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod diepdma;
#[doc = "DTXFSTS (r) register accessor: Device IN Endpoint Transmit FIFO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dtxfsts`]
module"]
pub type DTXFSTS = crate::Reg<dtxfsts::DTXFSTS_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO Status Register"]
pub mod dtxfsts;
#[doc = "DIEPDMAB (r) register accessor: Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`diepdmab`]
module"]
pub type DIEPDMAB = crate::Reg<diepdmab::DIEPDMAB_SPEC>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod diepdmab;
#[doc = "DOEPCTL_ISOCONT (rw) register accessor: Device Endpoint Control Register \\[ISOCONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl_isocont::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl_isocont::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl_isocont`]
module"]
pub type DOEPCTL_ISOCONT = crate::Reg<doepctl_isocont::DOEPCTL_ISOCONT_SPEC>;
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]"]
pub mod doepctl_isocont;
#[doc = "DOEPCTL_INTBULK (rw) register accessor: Device Endpoint Control Register \\[INTBULK\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl_intbulk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl_intbulk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepctl_intbulk`]
module"]
pub type DOEPCTL_INTBULK = crate::Reg<doepctl_intbulk::DOEPCTL_INTBULK_SPEC>;
#[doc = "Device Endpoint Control Register \\[INTBULK\\]"]
pub mod doepctl_intbulk;
#[doc = "DOEPINT (rw) register accessor: Device Endpoint Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepint`]
module"]
pub type DOEPINT = crate::Reg<doepint::DOEPINT_SPEC>;
#[doc = "Device Endpoint Interrupt Register"]
pub mod doepint;
#[doc = "DOEPTSIZ_ISO (rw) register accessor: Device Endpoint Transfer Size Register \\[ISO\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_iso::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_iso::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz_iso`]
module"]
pub type DOEPTSIZ_ISO = crate::Reg<doeptsiz_iso::DOEPTSIZ_ISO_SPEC>;
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]"]
pub mod doeptsiz_iso;
#[doc = "DOEPTSIZ_CONTROL (rw) register accessor: Device Endpoint Transfer Size Register \\[CONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doeptsiz_control`]
module"]
pub type DOEPTSIZ_CONTROL = crate::Reg<doeptsiz_control::DOEPTSIZ_CONTROL_SPEC>;
#[doc = "Device Endpoint Transfer Size Register \\[CONT\\]"]
pub mod doeptsiz_control;
#[doc = "DOEPDMA (rw) register accessor: Device Endpoint DMA Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdma`]
module"]
pub type DOEPDMA = crate::Reg<doepdma::DOEPDMA_SPEC>;
#[doc = "Device Endpoint DMA Address Register"]
pub mod doepdma;
#[doc = "DOEPDMAB (r) register accessor: Device Endpoint DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`doepdmab`]
module"]
pub type DOEPDMAB = crate::Reg<doepdmab::DOEPDMAB_SPEC>;
#[doc = "Device Endpoint DMA Buffer Address Register"]
pub mod doepdmab;
