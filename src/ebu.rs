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
    _reserved4: [u8; 0x08],
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
#[doc = "CLC (rw) register accessor: EBU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clc`]
module"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "EBU Clock Control Register"]
pub mod clc;
#[doc = "MODCON (rw) register accessor: EBU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`modcon`]
module"]
pub type MODCON = crate::Reg<modcon::MODCON_SPEC>;
#[doc = "EBU Configuration Register"]
pub mod modcon;
#[doc = "ID (r) register accessor: EBU Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "EBU Module Identification Register"]
pub mod id;
#[doc = "USERCON (rw) register accessor: EBU Test/Control Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usercon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usercon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`usercon`]
module"]
pub type USERCON = crate::Reg<usercon::USERCON_SPEC>;
#[doc = "EBU Test/Control Configuration Register"]
pub mod usercon;
#[doc = "ADDRSEL0 (rw) register accessor: EBU Address Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addrsel0`]
module"]
pub type ADDRSEL0 = crate::Reg<addrsel0::ADDRSEL0_SPEC>;
#[doc = "EBU Address Select Register 0"]
pub mod addrsel0;
#[doc = "ADDRSEL1 (rw) register accessor: EBU Address Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addrsel1`]
module"]
pub type ADDRSEL1 = crate::Reg<addrsel1::ADDRSEL1_SPEC>;
#[doc = "EBU Address Select Register 1"]
pub mod addrsel1;
#[doc = "ADDRSEL2 (rw) register accessor: EBU Address Select Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addrsel2`]
module"]
pub type ADDRSEL2 = crate::Reg<addrsel2::ADDRSEL2_SPEC>;
#[doc = "EBU Address Select Register 2"]
pub mod addrsel2;
#[doc = "ADDRSEL3 (rw) register accessor: EBU Address Select Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addrsel3`]
module"]
pub type ADDRSEL3 = crate::Reg<addrsel3::ADDRSEL3_SPEC>;
#[doc = "EBU Address Select Register 3"]
pub mod addrsel3;
#[doc = "BUSRCON0 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrcon0`]
module"]
pub type BUSRCON0 = crate::Reg<busrcon0::BUSRCON0_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon0;
#[doc = "BUSRAP0 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrap0`]
module"]
pub type BUSRAP0 = crate::Reg<busrap0::BUSRAP0_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap0;
#[doc = "BUSWCON0 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswcon0`]
module"]
pub type BUSWCON0 = crate::Reg<buswcon0::BUSWCON0_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon0;
#[doc = "BUSWAP0 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswap0`]
module"]
pub type BUSWAP0 = crate::Reg<buswap0::BUSWAP0_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap0;
#[doc = "BUSRCON1 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrcon1`]
module"]
pub type BUSRCON1 = crate::Reg<busrcon1::BUSRCON1_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon1;
#[doc = "BUSRAP1 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrap1`]
module"]
pub type BUSRAP1 = crate::Reg<busrap1::BUSRAP1_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap1;
#[doc = "BUSWCON1 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswcon1`]
module"]
pub type BUSWCON1 = crate::Reg<buswcon1::BUSWCON1_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon1;
#[doc = "BUSWAP1 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswap1`]
module"]
pub type BUSWAP1 = crate::Reg<buswap1::BUSWAP1_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap1;
#[doc = "BUSRCON2 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrcon2`]
module"]
pub type BUSRCON2 = crate::Reg<busrcon2::BUSRCON2_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon2;
#[doc = "BUSRAP2 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrap2`]
module"]
pub type BUSRAP2 = crate::Reg<busrap2::BUSRAP2_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap2;
#[doc = "BUSWCON2 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswcon2`]
module"]
pub type BUSWCON2 = crate::Reg<buswcon2::BUSWCON2_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon2;
#[doc = "BUSWAP2 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswap2`]
module"]
pub type BUSWAP2 = crate::Reg<buswap2::BUSWAP2_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap2;
#[doc = "BUSRCON3 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrcon3`]
module"]
pub type BUSRCON3 = crate::Reg<busrcon3::BUSRCON3_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon3;
#[doc = "BUSRAP3 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`busrap3`]
module"]
pub type BUSRAP3 = crate::Reg<busrap3::BUSRAP3_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap3;
#[doc = "BUSWCON3 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswcon3`]
module"]
pub type BUSWCON3 = crate::Reg<buswcon3::BUSWCON3_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon3;
#[doc = "BUSWAP3 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`buswap3`]
module"]
pub type BUSWAP3 = crate::Reg<buswap3::BUSWAP3_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap3;
#[doc = "SDRMCON (rw) register accessor: EBU SDRAM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdrmcon`]
module"]
pub type SDRMCON = crate::Reg<sdrmcon::SDRMCON_SPEC>;
#[doc = "EBU SDRAM Control Register"]
pub mod sdrmcon;
#[doc = "SDRMOD (rw) register accessor: EBU SDRAM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdrmod`]
module"]
pub type SDRMOD = crate::Reg<sdrmod::SDRMOD_SPEC>;
#[doc = "EBU SDRAM Mode Register"]
pub mod sdrmod;
#[doc = "SDRMREF (rw) register accessor: EBU SDRAM Refresh Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdrmref`]
module"]
pub type SDRMREF = crate::Reg<sdrmref::SDRMREF_SPEC>;
#[doc = "EBU SDRAM Refresh Control Register"]
pub mod sdrmref;
#[doc = "SDRSTAT (r) register accessor: EBU SDRAM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdrstat`]
module"]
pub type SDRSTAT = crate::Reg<sdrstat::SDRSTAT_SPEC>;
#[doc = "EBU SDRAM Status Register"]
pub mod sdrstat;
