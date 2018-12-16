#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Modulator Configuration Register"]
    pub modcfg: MODCFG,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Demodulator Input Configuration Register"]
    pub dicfg: DICFG,
    _reserved1: [u8; 8usize],
    #[doc = "0x14 - Filter Configuration Register, Main CIC Filter"]
    pub fcfgc: FCFGC,
    #[doc = "0x18 - Filter Configuration Register, Auxiliary Filter"]
    pub fcfga: FCFGA,
    _reserved2: [u8; 4usize],
    #[doc = "0x20 - Integration Window Control Register"]
    pub iwctr: IWCTR,
    _reserved3: [u8; 4usize],
    #[doc = "0x28 - Boundary Select Register"]
    pub boundsel: BOUNDSEL,
    _reserved4: [u8; 4usize],
    #[doc = "0x30 - Result Register, Main Filter"]
    pub resm: RESM,
    _reserved5: [u8; 4usize],
    #[doc = "0x38 - Offset Register, Main Filter"]
    pub offm: OFFM,
    _reserved6: [u8; 4usize],
    #[doc = "0x40 - Result Register, Auxiliary Filter"]
    pub resa: RESA,
    _reserved7: [u8; 12usize],
    #[doc = "0x50 - Time-Stamp Register"]
    pub tstmp: TSTMP,
    _reserved8: [u8; 76usize],
    #[doc = "0xa0 - Carrier Generator Synchronization Register"]
    pub cgsync: CGSYNC,
    _reserved9: [u8; 4usize],
    #[doc = "0xa8 - Rectification Configuration Register"]
    pub rectcfg: RECTCFG,
}
#[doc = "Modulator Configuration Register"]
pub struct MODCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modulator Configuration Register"]
pub mod modcfg;
#[doc = "Demodulator Input Configuration Register"]
pub struct DICFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Demodulator Input Configuration Register"]
pub mod dicfg;
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub struct FCFGC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter Configuration Register, Main CIC Filter"]
pub mod fcfgc;
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub struct FCFGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter Configuration Register, Auxiliary Filter"]
pub mod fcfga;
#[doc = "Integration Window Control Register"]
pub struct IWCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integration Window Control Register"]
pub mod iwctr;
#[doc = "Boundary Select Register"]
pub struct BOUNDSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Boundary Select Register"]
pub mod boundsel;
#[doc = "Result Register, Main Filter"]
pub struct RESM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Register, Main Filter"]
pub mod resm;
#[doc = "Offset Register, Main Filter"]
pub struct OFFM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Offset Register, Main Filter"]
pub mod offm;
#[doc = "Result Register, Auxiliary Filter"]
pub struct RESA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Register, Auxiliary Filter"]
pub mod resa;
#[doc = "Time-Stamp Register"]
pub struct TSTMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time-Stamp Register"]
pub mod tstmp;
#[doc = "Carrier Generator Synchronization Register"]
pub struct CGSYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Carrier Generator Synchronization Register"]
pub mod cgsync;
#[doc = "Rectification Configuration Register"]
pub struct RECTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rectification Configuration Register"]
pub mod rectcfg;
