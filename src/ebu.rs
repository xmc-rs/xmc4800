#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clc: Clc,
    modcon: Modcon,
    id: Id,
    usercon: Usercon,
    _reserved4: [u8; 0x08],
    addrsel0: Addrsel0,
    addrsel1: Addrsel1,
    addrsel2: Addrsel2,
    addrsel3: Addrsel3,
    busrcon0: Busrcon0,
    busrap0: Busrap0,
    buswcon0: Buswcon0,
    buswap0: Buswap0,
    busrcon1: Busrcon1,
    busrap1: Busrap1,
    buswcon1: Buswcon1,
    buswap1: Buswap1,
    busrcon2: Busrcon2,
    busrap2: Busrap2,
    buswcon2: Buswcon2,
    buswap2: Buswap2,
    busrcon3: Busrcon3,
    busrap3: Busrap3,
    buswcon3: Buswcon3,
    buswap3: Buswap3,
    sdrmcon: Sdrmcon,
    sdrmod: Sdrmod,
    sdrmref: Sdrmref,
    sdrstat: Sdrstat,
}
impl RegisterBlock {
    #[doc = "0x00 - EBU Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &Clc {
        &self.clc
    }
    #[doc = "0x04 - EBU Configuration Register"]
    #[inline(always)]
    pub const fn modcon(&self) -> &Modcon {
        &self.modcon
    }
    #[doc = "0x08 - EBU Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x0c - EBU Test/Control Configuration Register"]
    #[inline(always)]
    pub const fn usercon(&self) -> &Usercon {
        &self.usercon
    }
    #[doc = "0x18 - EBU Address Select Register 0"]
    #[inline(always)]
    pub const fn addrsel0(&self) -> &Addrsel0 {
        &self.addrsel0
    }
    #[doc = "0x1c - EBU Address Select Register 1"]
    #[inline(always)]
    pub const fn addrsel1(&self) -> &Addrsel1 {
        &self.addrsel1
    }
    #[doc = "0x20 - EBU Address Select Register 2"]
    #[inline(always)]
    pub const fn addrsel2(&self) -> &Addrsel2 {
        &self.addrsel2
    }
    #[doc = "0x24 - EBU Address Select Register 3"]
    #[inline(always)]
    pub const fn addrsel3(&self) -> &Addrsel3 {
        &self.addrsel3
    }
    #[doc = "0x28 - EBU Bus Configuration Register"]
    #[inline(always)]
    pub const fn busrcon0(&self) -> &Busrcon0 {
        &self.busrcon0
    }
    #[doc = "0x2c - EBU Bus Read Access Parameter Register"]
    #[inline(always)]
    pub const fn busrap0(&self) -> &Busrap0 {
        &self.busrap0
    }
    #[doc = "0x30 - EBU Bus Write Configuration Register"]
    #[inline(always)]
    pub const fn buswcon0(&self) -> &Buswcon0 {
        &self.buswcon0
    }
    #[doc = "0x34 - EBU Bus Write Access Parameter Register"]
    #[inline(always)]
    pub const fn buswap0(&self) -> &Buswap0 {
        &self.buswap0
    }
    #[doc = "0x38 - EBU Bus Configuration Register"]
    #[inline(always)]
    pub const fn busrcon1(&self) -> &Busrcon1 {
        &self.busrcon1
    }
    #[doc = "0x3c - EBU Bus Read Access Parameter Register"]
    #[inline(always)]
    pub const fn busrap1(&self) -> &Busrap1 {
        &self.busrap1
    }
    #[doc = "0x40 - EBU Bus Write Configuration Register"]
    #[inline(always)]
    pub const fn buswcon1(&self) -> &Buswcon1 {
        &self.buswcon1
    }
    #[doc = "0x44 - EBU Bus Write Access Parameter Register"]
    #[inline(always)]
    pub const fn buswap1(&self) -> &Buswap1 {
        &self.buswap1
    }
    #[doc = "0x48 - EBU Bus Configuration Register"]
    #[inline(always)]
    pub const fn busrcon2(&self) -> &Busrcon2 {
        &self.busrcon2
    }
    #[doc = "0x4c - EBU Bus Read Access Parameter Register"]
    #[inline(always)]
    pub const fn busrap2(&self) -> &Busrap2 {
        &self.busrap2
    }
    #[doc = "0x50 - EBU Bus Write Configuration Register"]
    #[inline(always)]
    pub const fn buswcon2(&self) -> &Buswcon2 {
        &self.buswcon2
    }
    #[doc = "0x54 - EBU Bus Write Access Parameter Register"]
    #[inline(always)]
    pub const fn buswap2(&self) -> &Buswap2 {
        &self.buswap2
    }
    #[doc = "0x58 - EBU Bus Configuration Register"]
    #[inline(always)]
    pub const fn busrcon3(&self) -> &Busrcon3 {
        &self.busrcon3
    }
    #[doc = "0x5c - EBU Bus Read Access Parameter Register"]
    #[inline(always)]
    pub const fn busrap3(&self) -> &Busrap3 {
        &self.busrap3
    }
    #[doc = "0x60 - EBU Bus Write Configuration Register"]
    #[inline(always)]
    pub const fn buswcon3(&self) -> &Buswcon3 {
        &self.buswcon3
    }
    #[doc = "0x64 - EBU Bus Write Access Parameter Register"]
    #[inline(always)]
    pub const fn buswap3(&self) -> &Buswap3 {
        &self.buswap3
    }
    #[doc = "0x68 - EBU SDRAM Control Register"]
    #[inline(always)]
    pub const fn sdrmcon(&self) -> &Sdrmcon {
        &self.sdrmcon
    }
    #[doc = "0x6c - EBU SDRAM Mode Register"]
    #[inline(always)]
    pub const fn sdrmod(&self) -> &Sdrmod {
        &self.sdrmod
    }
    #[doc = "0x70 - EBU SDRAM Refresh Control Register"]
    #[inline(always)]
    pub const fn sdrmref(&self) -> &Sdrmref {
        &self.sdrmref
    }
    #[doc = "0x74 - EBU SDRAM Status Register"]
    #[inline(always)]
    pub const fn sdrstat(&self) -> &Sdrstat {
        &self.sdrstat
    }
}
#[doc = "CLC (rw) register accessor: EBU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
#[doc(alias = "CLC")]
pub type Clc = crate::Reg<clc::ClcSpec>;
#[doc = "EBU Clock Control Register"]
pub mod clc;
#[doc = "MODCON (rw) register accessor: EBU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modcon`]
module"]
#[doc(alias = "MODCON")]
pub type Modcon = crate::Reg<modcon::ModconSpec>;
#[doc = "EBU Configuration Register"]
pub mod modcon;
#[doc = "ID (r) register accessor: EBU Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "EBU Module Identification Register"]
pub mod id;
#[doc = "USERCON (rw) register accessor: EBU Test/Control Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usercon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usercon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usercon`]
module"]
#[doc(alias = "USERCON")]
pub type Usercon = crate::Reg<usercon::UserconSpec>;
#[doc = "EBU Test/Control Configuration Register"]
pub mod usercon;
#[doc = "ADDRSEL0 (rw) register accessor: EBU Address Select Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrsel0`]
module"]
#[doc(alias = "ADDRSEL0")]
pub type Addrsel0 = crate::Reg<addrsel0::Addrsel0Spec>;
#[doc = "EBU Address Select Register 0"]
pub mod addrsel0;
#[doc = "ADDRSEL1 (rw) register accessor: EBU Address Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrsel1`]
module"]
#[doc(alias = "ADDRSEL1")]
pub type Addrsel1 = crate::Reg<addrsel1::Addrsel1Spec>;
#[doc = "EBU Address Select Register 1"]
pub mod addrsel1;
#[doc = "ADDRSEL2 (rw) register accessor: EBU Address Select Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrsel2`]
module"]
#[doc(alias = "ADDRSEL2")]
pub type Addrsel2 = crate::Reg<addrsel2::Addrsel2Spec>;
#[doc = "EBU Address Select Register 2"]
pub mod addrsel2;
#[doc = "ADDRSEL3 (rw) register accessor: EBU Address Select Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrsel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrsel3`]
module"]
#[doc(alias = "ADDRSEL3")]
pub type Addrsel3 = crate::Reg<addrsel3::Addrsel3Spec>;
#[doc = "EBU Address Select Register 3"]
pub mod addrsel3;
#[doc = "BUSRCON0 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrcon0`]
module"]
#[doc(alias = "BUSRCON0")]
pub type Busrcon0 = crate::Reg<busrcon0::Busrcon0Spec>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon0;
#[doc = "BUSRAP0 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrap0`]
module"]
#[doc(alias = "BUSRAP0")]
pub type Busrap0 = crate::Reg<busrap0::Busrap0Spec>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap0;
#[doc = "BUSWCON0 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswcon0`]
module"]
#[doc(alias = "BUSWCON0")]
pub type Buswcon0 = crate::Reg<buswcon0::Buswcon0Spec>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon0;
#[doc = "BUSWAP0 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswap0`]
module"]
#[doc(alias = "BUSWAP0")]
pub type Buswap0 = crate::Reg<buswap0::Buswap0Spec>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap0;
#[doc = "BUSRCON1 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrcon1`]
module"]
#[doc(alias = "BUSRCON1")]
pub type Busrcon1 = crate::Reg<busrcon1::Busrcon1Spec>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon1;
#[doc = "BUSRAP1 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrap1`]
module"]
#[doc(alias = "BUSRAP1")]
pub type Busrap1 = crate::Reg<busrap1::Busrap1Spec>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap1;
#[doc = "BUSWCON1 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswcon1`]
module"]
#[doc(alias = "BUSWCON1")]
pub type Buswcon1 = crate::Reg<buswcon1::Buswcon1Spec>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon1;
#[doc = "BUSWAP1 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswap1`]
module"]
#[doc(alias = "BUSWAP1")]
pub type Buswap1 = crate::Reg<buswap1::Buswap1Spec>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap1;
#[doc = "BUSRCON2 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrcon2`]
module"]
#[doc(alias = "BUSRCON2")]
pub type Busrcon2 = crate::Reg<busrcon2::Busrcon2Spec>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon2;
#[doc = "BUSRAP2 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrap2`]
module"]
#[doc(alias = "BUSRAP2")]
pub type Busrap2 = crate::Reg<busrap2::Busrap2Spec>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap2;
#[doc = "BUSWCON2 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswcon2`]
module"]
#[doc(alias = "BUSWCON2")]
pub type Buswcon2 = crate::Reg<buswcon2::Buswcon2Spec>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon2;
#[doc = "BUSWAP2 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswap2`]
module"]
#[doc(alias = "BUSWAP2")]
pub type Buswap2 = crate::Reg<buswap2::Buswap2Spec>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap2;
#[doc = "BUSRCON3 (rw) register accessor: EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrcon3`]
module"]
#[doc(alias = "BUSRCON3")]
pub type Busrcon3 = crate::Reg<busrcon3::Busrcon3Spec>;
#[doc = "EBU Bus Configuration Register"]
pub mod busrcon3;
#[doc = "BUSRAP3 (rw) register accessor: EBU Bus Read Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busrap3`]
module"]
#[doc(alias = "BUSRAP3")]
pub type Busrap3 = crate::Reg<busrap3::Busrap3Spec>;
#[doc = "EBU Bus Read Access Parameter Register"]
pub mod busrap3;
#[doc = "BUSWCON3 (rw) register accessor: EBU Bus Write Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswcon3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswcon3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswcon3`]
module"]
#[doc(alias = "BUSWCON3")]
pub type Buswcon3 = crate::Reg<buswcon3::Buswcon3Spec>;
#[doc = "EBU Bus Write Configuration Register"]
pub mod buswcon3;
#[doc = "BUSWAP3 (rw) register accessor: EBU Bus Write Access Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buswap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buswap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buswap3`]
module"]
#[doc(alias = "BUSWAP3")]
pub type Buswap3 = crate::Reg<buswap3::Buswap3Spec>;
#[doc = "EBU Bus Write Access Parameter Register"]
pub mod buswap3;
#[doc = "SDRMCON (rw) register accessor: EBU SDRAM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdrmcon`]
module"]
#[doc(alias = "SDRMCON")]
pub type Sdrmcon = crate::Reg<sdrmcon::SdrmconSpec>;
#[doc = "EBU SDRAM Control Register"]
pub mod sdrmcon;
#[doc = "SDRMOD (rw) register accessor: EBU SDRAM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmod::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmod::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdrmod`]
module"]
#[doc(alias = "SDRMOD")]
pub type Sdrmod = crate::Reg<sdrmod::SdrmodSpec>;
#[doc = "EBU SDRAM Mode Register"]
pub mod sdrmod;
#[doc = "SDRMREF (rw) register accessor: EBU SDRAM Refresh Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmref::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmref::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdrmref`]
module"]
#[doc(alias = "SDRMREF")]
pub type Sdrmref = crate::Reg<sdrmref::SdrmrefSpec>;
#[doc = "EBU SDRAM Refresh Control Register"]
pub mod sdrmref;
#[doc = "SDRSTAT (r) register accessor: EBU SDRAM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdrstat`]
module"]
#[doc(alias = "SDRSTAT")]
pub type Sdrstat = crate::Reg<sdrstat::SdrstatSpec>;
#[doc = "EBU SDRAM Status Register"]
pub mod sdrstat;
