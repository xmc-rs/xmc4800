#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Host Channel Characteristics Register"]
    pub hcchar: HCCHAR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Host Channel Interrupt Register"]
    pub hcint: HCINT,
    #[doc = "0x0c - Host Channel Interrupt Mask Register"]
    pub hcintmsk: HCINTMSK,
    _reserved_3_hctsiz: [u8; 0x04],
    _reserved_4_hcdma: [u8; 0x04],
    _reserved5: [u8; 0x04],
    #[doc = "0x1c - Host Channel DMA Buffer Address Register"]
    pub hcdmab: HCDMAB,
}
impl RegisterBlock {
    #[doc = "0x10 - Host Channel Transfer Size Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub const fn hctsiz_scatgather(&self) -> &HCTSIZ_SCATGATHER {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x10 - Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub const fn hctsiz_buffermode(&self) -> &HCTSIZ_BUFFERMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(16usize).cast() }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub const fn hcdma_scatgather(&self) -> &HCDMA_SCATGATHER {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub const fn hcdma_buffermode(&self) -> &HCDMA_BUFFERMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
}
#[doc = "HCCHAR (rw) register accessor: an alias for `Reg<HCCHAR_SPEC>`"]
pub type HCCHAR = crate::Reg<hcchar::HCCHAR_SPEC>;
#[doc = "Host Channel Characteristics Register"]
pub mod hcchar;
#[doc = "HCINT (rw) register accessor: an alias for `Reg<HCINT_SPEC>`"]
pub type HCINT = crate::Reg<hcint::HCINT_SPEC>;
#[doc = "Host Channel Interrupt Register"]
pub mod hcint;
#[doc = "HCINTMSK (rw) register accessor: an alias for `Reg<HCINTMSK_SPEC>`"]
pub type HCINTMSK = crate::Reg<hcintmsk::HCINTMSK_SPEC>;
#[doc = "Host Channel Interrupt Mask Register"]
pub mod hcintmsk;
#[doc = "HCTSIZ_BUFFERMODE (rw) register accessor: an alias for `Reg<HCTSIZ_BUFFERMODE_SPEC>`"]
pub type HCTSIZ_BUFFERMODE = crate::Reg<hctsiz_buffermode::HCTSIZ_BUFFERMODE_SPEC>;
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
pub mod hctsiz_buffermode;
#[doc = "HCTSIZ_SCATGATHER (rw) register accessor: an alias for `Reg<HCTSIZ_SCATGATHER_SPEC>`"]
pub type HCTSIZ_SCATGATHER = crate::Reg<hctsiz_scatgather::HCTSIZ_SCATGATHER_SPEC>;
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]"]
pub mod hctsiz_scatgather;
#[doc = "HCDMA_BUFFERMODE (rw) register accessor: an alias for `Reg<HCDMA_BUFFERMODE_SPEC>`"]
pub type HCDMA_BUFFERMODE = crate::Reg<hcdma_buffermode::HCDMA_BUFFERMODE_SPEC>;
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]"]
pub mod hcdma_buffermode;
#[doc = "HCDMA_SCATGATHER (rw) register accessor: an alias for `Reg<HCDMA_SCATGATHER_SPEC>`"]
pub type HCDMA_SCATGATHER = crate::Reg<hcdma_scatgather::HCDMA_SCATGATHER_SPEC>;
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]"]
pub mod hcdma_scatgather;
#[doc = "HCDMAB (r) register accessor: an alias for `Reg<HCDMAB_SPEC>`"]
pub type HCDMAB = crate::Reg<hcdmab::HCDMAB_SPEC>;
#[doc = "Host Channel DMA Buffer Address Register"]
pub mod hcdmab;
