#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBU Clock Control Register"]
    pub clc: crate::Reg<clc::CLC_SPEC>,
    #[doc = "0x04 - EBU Configuration Register"]
    pub modcon: crate::Reg<modcon::MODCON_SPEC>,
    #[doc = "0x08 - EBU Module Identification Register"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x0c - EBU Test/Control Configuration Register"]
    pub usercon: crate::Reg<usercon::USERCON_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - EBU Address Select Register 0"]
    pub addrsel0: crate::Reg<addrsel0::ADDRSEL0_SPEC>,
    #[doc = "0x1c - EBU Address Select Register 1"]
    pub addrsel1: crate::Reg<addrsel1::ADDRSEL1_SPEC>,
    #[doc = "0x20 - EBU Address Select Register 2"]
    pub addrsel2: crate::Reg<addrsel2::ADDRSEL2_SPEC>,
    #[doc = "0x24 - EBU Address Select Register 3"]
    pub addrsel3: crate::Reg<addrsel3::ADDRSEL3_SPEC>,
    #[doc = "0x28 - EBU Bus Configuration Register"]
    pub busrcon0: crate::Reg<busrcon0::BUSRCON0_SPEC>,
    #[doc = "0x2c - EBU Bus Read Access Parameter Register"]
    pub busrap0: crate::Reg<busrap0::BUSRAP0_SPEC>,
    #[doc = "0x30 - EBU Bus Write Configuration Register"]
    pub buswcon0: crate::Reg<buswcon0::BUSWCON0_SPEC>,
    #[doc = "0x34 - EBU Bus Write Access Parameter Register"]
    pub buswap0: crate::Reg<buswap0::BUSWAP0_SPEC>,
    #[doc = "0x38 - EBU Bus Configuration Register"]
    pub busrcon1: crate::Reg<busrcon1::BUSRCON1_SPEC>,
    #[doc = "0x3c - EBU Bus Read Access Parameter Register"]
    pub busrap1: crate::Reg<busrap1::BUSRAP1_SPEC>,
    #[doc = "0x40 - EBU Bus Write Configuration Register"]
    pub buswcon1: crate::Reg<buswcon1::BUSWCON1_SPEC>,
    #[doc = "0x44 - EBU Bus Write Access Parameter Register"]
    pub buswap1: crate::Reg<buswap1::BUSWAP1_SPEC>,
    #[doc = "0x48 - EBU Bus Configuration Register"]
    pub busrcon2: crate::Reg<busrcon2::BUSRCON2_SPEC>,
    #[doc = "0x4c - EBU Bus Read Access Parameter Register"]
    pub busrap2: crate::Reg<busrap2::BUSRAP2_SPEC>,
    #[doc = "0x50 - EBU Bus Write Configuration Register"]
    pub buswcon2: crate::Reg<buswcon2::BUSWCON2_SPEC>,
    #[doc = "0x54 - EBU Bus Write Access Parameter Register"]
    pub buswap2: crate::Reg<buswap2::BUSWAP2_SPEC>,
    #[doc = "0x58 - EBU Bus Configuration Register"]
    pub busrcon3: crate::Reg<busrcon3::BUSRCON3_SPEC>,
    #[doc = "0x5c - EBU Bus Read Access Parameter Register"]
    pub busrap3: crate::Reg<busrap3::BUSRAP3_SPEC>,
    #[doc = "0x60 - EBU Bus Write Configuration Register"]
    pub buswcon3: crate::Reg<buswcon3::BUSWCON3_SPEC>,
    #[doc = "0x64 - EBU Bus Write Access Parameter Register"]
    pub buswap3: crate::Reg<buswap3::BUSWAP3_SPEC>,
    #[doc = "0x68 - EBU SDRAM Control Register"]
    pub sdrmcon: crate::Reg<sdrmcon::SDRMCON_SPEC>,
    #[doc = "0x6c - EBU SDRAM Mode Register"]
    pub sdrmod: crate::Reg<sdrmod::SDRMOD_SPEC>,
    #[doc = "0x70 - EBU SDRAM Refresh Control Register"]
    pub sdrmref: crate::Reg<sdrmref::SDRMREF_SPEC>,
    #[doc = "0x74 - EBU SDRAM Status Register"]
    pub sdrstat: crate::Reg<sdrstat::SDRSTAT_SPEC>,
}
#[doc = "CLC register accessor: an alias for `Reg<CLC_SPEC>`"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "EBU Clock Control Register"]
pub mod clc;
#[doc = "MODCON register accessor: an alias for `Reg<MODCON_SPEC>`"]
pub type MODCON = crate::Reg<modcon::MODCON_SPEC>;
#[doc = "EBU Configuration Register"]
pub mod modcon;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "EBU Module Identification Register"]
pub mod id;
#[doc = "USERCON register accessor: an alias for `Reg<USERCON_SPEC>`"]
pub type USERCON = crate::Reg<usercon::USERCON_SPEC>;
#[doc = "EBU Test/Control Configuration Register"]
pub mod usercon;
#[doc = "ADDRSEL0 register accessor: an alias for `Reg<ADDRSEL0_SPEC>`"]
pub type ADDRSEL0 = crate::Reg<addrsel0::ADDRSEL0_SPEC>;
#[doc = "EBU Address Select Register 0"]
pub mod addrsel0;
#[doc = "ADDRSEL1 register accessor: an alias for `Reg<ADDRSEL1_SPEC>`"]
pub type ADDRSEL1 = crate::Reg<addrsel1::ADDRSEL1_SPEC>;
#[doc = "EBU Address Select Register 1"]
pub mod addrsel1;
#[doc = "ADDRSEL2 register accessor: an alias for `Reg<ADDRSEL2_SPEC>`"]
pub type ADDRSEL2 = crate::Reg<addrsel2::ADDRSEL2_SPEC>;
#[doc = "EBU Address Select Register 2"]
pub mod addrsel2;
#[doc = "ADDRSEL3 register accessor: an alias for `Reg<ADDRSEL3_SPEC>`"]
pub type ADDRSEL3 = crate::Reg<addrsel3::ADDRSEL3_SPEC>;
#[doc = "EBU Address Select Register 3"]
pub mod addrsel3;
#[doc = "BUSRCON0 register accessor: an alias for `Reg<BUSRCON0_SPEC>`"]
pub type BUSRCON0 = crate::Reg<busrcon0::BUSRCON0_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon0;
#[doc = "BUSRAP0 register accessor: an alias for `Reg<BUSRAP0_SPEC>`"]
pub type BUSRAP0 = crate::Reg<busrap0::BUSRAP0_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap0;
#[doc = "BUSWCON0 register accessor: an alias for `Reg<BUSWCON0_SPEC>`"]
pub type BUSWCON0 = crate::Reg<buswcon0::BUSWCON0_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon0;
#[doc = "BUSWAP0 register accessor: an alias for `Reg<BUSWAP0_SPEC>`"]
pub type BUSWAP0 = crate::Reg<buswap0::BUSWAP0_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap0;
#[doc = "BUSRCON1 register accessor: an alias for `Reg<BUSRCON1_SPEC>`"]
pub type BUSRCON1 = crate::Reg<busrcon1::BUSRCON1_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon1;
#[doc = "BUSRAP1 register accessor: an alias for `Reg<BUSRAP1_SPEC>`"]
pub type BUSRAP1 = crate::Reg<busrap1::BUSRAP1_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap1;
#[doc = "BUSWCON1 register accessor: an alias for `Reg<BUSWCON1_SPEC>`"]
pub type BUSWCON1 = crate::Reg<buswcon1::BUSWCON1_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon1;
#[doc = "BUSWAP1 register accessor: an alias for `Reg<BUSWAP1_SPEC>`"]
pub type BUSWAP1 = crate::Reg<buswap1::BUSWAP1_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap1;
#[doc = "BUSRCON2 register accessor: an alias for `Reg<BUSRCON2_SPEC>`"]
pub type BUSRCON2 = crate::Reg<busrcon2::BUSRCON2_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon2;
#[doc = "BUSRAP2 register accessor: an alias for `Reg<BUSRAP2_SPEC>`"]
pub type BUSRAP2 = crate::Reg<busrap2::BUSRAP2_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap2;
#[doc = "BUSWCON2 register accessor: an alias for `Reg<BUSWCON2_SPEC>`"]
pub type BUSWCON2 = crate::Reg<buswcon2::BUSWCON2_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon2;
#[doc = "BUSWAP2 register accessor: an alias for `Reg<BUSWAP2_SPEC>`"]
pub type BUSWAP2 = crate::Reg<buswap2::BUSWAP2_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap2;
#[doc = "BUSRCON3 register accessor: an alias for `Reg<BUSRCON3_SPEC>`"]
pub type BUSRCON3 = crate::Reg<busrcon3::BUSRCON3_SPEC>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon3;
#[doc = "BUSRAP3 register accessor: an alias for `Reg<BUSRAP3_SPEC>`"]
pub type BUSRAP3 = crate::Reg<busrap3::BUSRAP3_SPEC>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap3;
#[doc = "BUSWCON3 register accessor: an alias for `Reg<BUSWCON3_SPEC>`"]
pub type BUSWCON3 = crate::Reg<buswcon3::BUSWCON3_SPEC>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon3;
#[doc = "BUSWAP3 register accessor: an alias for `Reg<BUSWAP3_SPEC>`"]
pub type BUSWAP3 = crate::Reg<buswap3::BUSWAP3_SPEC>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap3;
#[doc = "SDRMCON register accessor: an alias for `Reg<SDRMCON_SPEC>`"]
pub type SDRMCON = crate::Reg<sdrmcon::SDRMCON_SPEC>;
#[doc = "EBU SDRAM Control Register"]
pub mod sdrmcon;
#[doc = "SDRMOD register accessor: an alias for `Reg<SDRMOD_SPEC>`"]
pub type SDRMOD = crate::Reg<sdrmod::SDRMOD_SPEC>;
#[doc = "EBU SDRAM Mode Register"]
pub mod sdrmod;
#[doc = "SDRMREF register accessor: an alias for `Reg<SDRMREF_SPEC>`"]
pub type SDRMREF = crate::Reg<sdrmref::SDRMREF_SPEC>;
#[doc = "EBU SDRAM Refresh Control Register"]
pub mod sdrmref;
#[doc = "SDRSTAT register accessor: an alias for `Reg<SDRSTAT_SPEC>`"]
pub type SDRSTAT = crate::Reg<sdrstat::SDRSTAT_SPEC>;
#[doc = "EBU SDRAM Status Register"]
pub mod sdrstat;
