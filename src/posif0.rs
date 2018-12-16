#[doc = r" Register block"]
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
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - Module Identification register"]
    pub midr: MIDR,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Hall Sensor Patterns"]
    pub halp: HALP,
    #[doc = "0x34 - Hall Sensor Shadow Patterns"]
    pub halps: HALPS,
    _reserved2: [u8; 8usize],
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
    _reserved3: [u8; 12usize],
    #[doc = "0x60 - Quadrature Decoder Control"]
    pub qdc: QDC,
    _reserved4: [u8; 12usize],
    #[doc = "0x70 - POSIF Interrupt Flags"]
    pub pflg: PFLG,
    #[doc = "0x74 - POSIF Interrupt Enable"]
    pub pflge: PFLGE,
    #[doc = "0x78 - POSIF Interrupt Set"]
    pub spflg: SPFLG,
    #[doc = "0x7c - POSIF Interrupt Clear"]
    pub rpflg: RPFLG,
    _reserved5: [u8; 128usize],
    #[doc = "0x100 - POSIF Debug register"]
    pub pdbg: PDBG,
}
#[doc = "POSIF configuration"]
pub struct PCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF configuration"]
pub mod pconf;
#[doc = "POSIF Suspend Config"]
pub struct PSUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Suspend Config"]
pub mod psus;
#[doc = "POSIF Run Bit Set"]
pub struct PRUNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Run Bit Set"]
pub mod pruns;
#[doc = "POSIF Run Bit Clear"]
pub struct PRUNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Run Bit Clear"]
pub mod prunc;
#[doc = "POSIF Run Bit Status"]
pub struct PRUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Run Bit Status"]
pub mod prun;
#[doc = "Module Identification register"]
pub struct MIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "Hall Sensor Patterns"]
pub struct HALP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "Hall Sensor Shadow Patterns"]
pub struct HALPS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "Multi-Channel Pattern"]
pub struct MCM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "Multi-Channel Shadow Pattern"]
pub struct MCSM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "Multi-Channel Pattern Control set"]
pub struct MCMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "Multi-Channel Pattern Control clear"]
pub struct MCMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "Multi-Channel Pattern Control flag"]
pub struct MCMF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "Quadrature Decoder Control"]
pub struct QDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "POSIF Interrupt Flags"]
pub struct PFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Interrupt Flags"]
pub mod pflg;
#[doc = "POSIF Interrupt Enable"]
pub struct PFLGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Interrupt Enable"]
pub mod pflge;
#[doc = "POSIF Interrupt Set"]
pub struct SPFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Interrupt Set"]
pub mod spflg;
#[doc = "POSIF Interrupt Clear"]
pub struct RPFLG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Interrupt Clear"]
pub mod rpflg;
#[doc = "POSIF Debug register"]
pub struct PDBG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "POSIF Debug register"]
pub mod pdbg;
