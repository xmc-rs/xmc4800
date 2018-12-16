#[doc = r" Register block"]
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
    _reserved0: [u8; 8usize],
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
#[doc = "EBU Clock Control Register"]
pub struct CLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Clock Control Register"]
pub mod clc;
#[doc = "EBU Configuration Register"]
pub struct MODCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Configuration Register"]
pub mod modcon;
#[doc = "EBU Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Module Identification Register"]
pub mod id;
#[doc = "EBU Test/Control Configuration Register"]
pub struct USERCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Test/Control Configuration Register"]
pub mod usercon;
#[doc = "EBU Address Select Register 0"]
pub struct ADDRSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Address Select Register 0"]
pub mod addrsel0;
#[doc = "EBU Address Select Register 1"]
pub struct ADDRSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Address Select Register 1"]
pub mod addrsel1;
#[doc = "EBU Address Select Register 2"]
pub struct ADDRSEL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Address Select Register 2"]
pub mod addrsel2;
#[doc = "EBU Address Select Register 3"]
pub struct ADDRSEL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Address Select Register 3"]
pub mod addrsel3;
#[doc = "EBU Bus Configuration Register"]
pub struct BUSRCON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon0;
#[doc = "EBU Bus Read Access Parameter Register"]
pub struct BUSRAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap0;
#[doc = "EBU Bus Write Configuration Register"]
pub struct BUSWCON0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon0;
#[doc = "EBU Bus Write Access Parameter Register"]
pub struct BUSWAP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap0;
#[doc = "EBU Bus Configuration Register"]
pub struct BUSRCON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon1;
#[doc = "EBU Bus Read Access Parameter Register"]
pub struct BUSRAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap1;
#[doc = "EBU Bus Write Configuration Register"]
pub struct BUSWCON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon1;
#[doc = "EBU Bus Write Access Parameter Register"]
pub struct BUSWAP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap1;
#[doc = "EBU Bus Configuration Register"]
pub struct BUSRCON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon2;
#[doc = "EBU Bus Read Access Parameter Register"]
pub struct BUSRAP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap2;
#[doc = "EBU Bus Write Configuration Register"]
pub struct BUSWCON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon2;
#[doc = "EBU Bus Write Access Parameter Register"]
pub struct BUSWAP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap2;
#[doc = "EBU Bus Configuration Register"]
pub struct BUSRCON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon3;
#[doc = "EBU Bus Read Access Parameter Register"]
pub struct BUSRAP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap3;
#[doc = "EBU Bus Write Configuration Register"]
pub struct BUSWCON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon3;
#[doc = "EBU Bus Write Access Parameter Register"]
pub struct BUSWAP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap3;
#[doc = "EBU SDRAM Control Register"]
pub struct SDRMCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU SDRAM Control Register"]
pub mod sdrmcon;
#[doc = "EBU SDRAM Mode Register"]
pub struct SDRMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU SDRAM Mode Register"]
pub mod sdrmod;
#[doc = "EBU SDRAM Refresh Control Register"]
pub struct SDRMREF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU SDRAM Refresh Control Register"]
pub mod sdrmref;
#[doc = "EBU SDRAM Status Register"]
pub struct SDRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EBU SDRAM Status Register"]
pub mod sdrstat;
