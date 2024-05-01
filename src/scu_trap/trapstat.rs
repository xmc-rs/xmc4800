#[doc = "Register `TRAPSTAT` reader"]
pub type R = crate::R<TrapstatSpec>;
#[doc = "OSC_HP Oscillator Watchdog Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soscwdgt {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Soscwdgt> for bool {
    #[inline(always)]
    fn from(variant: Soscwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` reader - OSC_HP Oscillator Watchdog Trap Status"]
pub type SoscwdgtR = crate::BitReader<Soscwdgt>;
impl SoscwdgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soscwdgt {
        match self.bits {
            false => Soscwdgt::Value1,
            true => Soscwdgt::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Soscwdgt::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Soscwdgt::Value2
    }
}
#[doc = "System VCO Lock Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcolckt {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Svcolckt> for bool {
    #[inline(always)]
    fn from(variant: Svcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Status"]
pub type SvcolcktR = crate::BitReader<Svcolckt>;
impl SvcolcktR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcolckt {
        match self.bits {
            false => Svcolckt::Value1,
            true => Svcolckt::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Svcolckt::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Svcolckt::Value2
    }
}
#[doc = "USB VCO Lock Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uvcolckt {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Uvcolckt> for bool {
    #[inline(always)]
    fn from(variant: Uvcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Status"]
pub type UvcolcktR = crate::BitReader<Uvcolckt>;
impl UvcolcktR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uvcolckt {
        match self.bits {
            false => Uvcolckt::Value1,
            true => Uvcolckt::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Uvcolckt::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Uvcolckt::Value2
    }
}
#[doc = "Parity Error Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pet {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Pet> for bool {
    #[inline(always)]
    fn from(variant: Pet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Status"]
pub type PetR = crate::BitReader<Pet>;
impl PetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pet {
        match self.bits {
            false => Pet::Value1,
            true => Pet::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pet::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pet::Value2
    }
}
#[doc = "Brown Out Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brwnt {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Brwnt> for bool {
    #[inline(always)]
    fn from(variant: Brwnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Status"]
pub type BrwntR = crate::BitReader<Brwnt>;
impl BrwntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brwnt {
        match self.bits {
            false => Brwnt::Value1,
            true => Brwnt::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Brwnt::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Brwnt::Value2
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdgt {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Ulpwdgt> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` reader - OSC_ULP Oscillator Watchdog Trap Status"]
pub type UlpwdgtR = crate::BitReader<Ulpwdgt>;
impl UlpwdgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulpwdgt {
        match self.bits {
            false => Ulpwdgt::Value1,
            true => Ulpwdgt::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ulpwdgt::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ulpwdgt::Value2
    }
}
#[doc = "Peripheral Bridge 0 Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr0t {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Bwerr0t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr0t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Status"]
pub type Bwerr0tR = crate::BitReader<Bwerr0t>;
impl Bwerr0tR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwerr0t {
        match self.bits {
            false => Bwerr0t::Value1,
            true => Bwerr0t::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bwerr0t::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bwerr0t::Value2
    }
}
#[doc = "Peripheral Bridge 1 Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr1t {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Bwerr1t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr1t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Status"]
pub type Bwerr1tR = crate::BitReader<Bwerr1t>;
impl Bwerr1tR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwerr1t {
        match self.bits {
            false => Bwerr1t::Value1,
            true => Bwerr1t::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bwerr1t::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bwerr1t::Value2
    }
}
#[doc = "EtherCat Reset 0 Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecat0rst {
    #[doc = "0: No pending trap request"]
    Value1 = 0,
    #[doc = "1: Pending trap request"]
    Value2 = 1,
}
impl From<Ecat0rst> for bool {
    #[inline(always)]
    fn from(variant: Ecat0rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECAT0RST` reader - EtherCat Reset 0 Trap Status"]
pub type Ecat0rstR = crate::BitReader<Ecat0rst>;
impl Ecat0rstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecat0rst {
        match self.bits {
            false => Ecat0rst::Value1,
            true => Ecat0rst::Value2,
        }
    }
    #[doc = "No pending trap request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecat0rst::Value1
    }
    #[doc = "Pending trap request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecat0rst::Value2
    }
}
impl R {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SoscwdgtR {
        SoscwdgtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SvcolcktR {
        SvcolcktR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UvcolcktR {
        UvcolcktR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Status"]
    #[inline(always)]
    pub fn pet(&self) -> PetR {
        PetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BrwntR {
        BrwntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> UlpwdgtR {
        UlpwdgtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> Bwerr0tR {
        Bwerr0tR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> Bwerr1tR {
        Bwerr1tR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - EtherCat Reset 0 Trap Status"]
    #[inline(always)]
    pub fn ecat0rst(&self) -> Ecat0rstR {
        Ecat0rstR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Trap Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trapstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrapstatSpec;
impl crate::RegisterSpec for TrapstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trapstat::R`](R) reader structure"]
impl crate::Readable for TrapstatSpec {}
#[doc = "`reset()` method sets TRAPSTAT to value 0"]
impl crate::Resettable for TrapstatSpec {
    const RESET_VALUE: u32 = 0;
}
