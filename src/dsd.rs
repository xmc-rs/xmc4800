#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved1: [u8; 28usize],
    #[doc = "0x28 - OCDS Control and Status Register"]
    pub ocs: OCS,
    _reserved2: [u8; 84usize],
    #[doc = "0x80 - Global Configuration Register"]
    pub globcfg: GLOBCFG,
    _reserved3: [u8; 4usize],
    #[doc = "0x88 - Global Run Control Register"]
    pub globrc: GLOBRC,
    _reserved4: [u8; 20usize],
    #[doc = "0xa0 - Carrier Generator Configuration Register"]
    pub cgcfg: CGCFG,
    _reserved5: [u8; 60usize],
    #[doc = "0xe0 - Event Flag Register"]
    pub evflag: EVFLAG,
    #[doc = "0xe4 - Event Flag Clear Register"]
    pub evflagclr: EVFLAGCLR,
}
#[doc = "Clock Control Register"]
pub struct CLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCDS Control and Status Register"]
pub struct OCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "Global Configuration Register"]
pub struct GLOBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "Global Run Control Register"]
pub struct GLOBRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Run Control Register"]
pub mod globrc;
#[doc = "Carrier Generator Configuration Register"]
pub struct CGCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Carrier Generator Configuration Register"]
pub mod cgcfg;
#[doc = "Event Flag Register"]
pub struct EVFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Register"]
pub mod evflag;
#[doc = "Event Flag Clear Register"]
pub struct EVFLAGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Clear Register"]
pub mod evflagclr;
