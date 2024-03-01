#[doc = "Register `PLLSTAT` reader"]
pub type R = crate::R<PllstatSpec>;
#[doc = "VCO Bypass Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcobyst {
    #[doc = "0: Free-running / Normal Mode is entered"]
    Value1 = 0,
    #[doc = "1: Prescaler Mode is entered"]
    Value2 = 1,
}
impl From<Vcobyst> for bool {
    #[inline(always)]
    fn from(variant: Vcobyst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOBYST` reader - VCO Bypass Status"]
pub type VcobystR = crate::BitReader<Vcobyst>;
impl VcobystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcobyst {
        match self.bits {
            false => Vcobyst::Value1,
            true => Vcobyst::Value2,
        }
    }
    #[doc = "Free-running / Normal Mode is entered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcobyst::Value1
    }
    #[doc = "Prescaler Mode is entered"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcobyst::Value2
    }
}
#[doc = "PLL Power-saving Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwdstat {
    #[doc = "0: PLL Power-saving Mode was not entered"]
    Value1 = 0,
    #[doc = "1: PLL Power-saving Mode was entered"]
    Value2 = 1,
}
impl From<Pwdstat> for bool {
    #[inline(always)]
    fn from(variant: Pwdstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDSTAT` reader - PLL Power-saving Mode Status"]
pub type PwdstatR = crate::BitReader<Pwdstat>;
impl PwdstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwdstat {
        match self.bits {
            false => Pwdstat::Value1,
            true => Pwdstat::Value2,
        }
    }
    #[doc = "PLL Power-saving Mode was not entered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pwdstat::Value1
    }
    #[doc = "PLL Power-saving Mode was entered"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pwdstat::Value2
    }
}
#[doc = "PLL LOCK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcolock {
    #[doc = "0: PLL not locked"]
    Value1 = 0,
    #[doc = "1: PLL locked"]
    Value2 = 1,
}
impl From<Vcolock> for bool {
    #[inline(always)]
    fn from(variant: Vcolock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOLOCK` reader - PLL LOCK Status"]
pub type VcolockR = crate::BitReader<Vcolock>;
impl VcolockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcolock {
        match self.bits {
            false => Vcolock::Value1,
            true => Vcolock::Value2,
        }
    }
    #[doc = "PLL not locked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcolock::Value1
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcolock::Value2
    }
}
#[doc = "K1 Divider Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K1rdy {
    #[doc = "0: K1-Divider does not operate with the new value"]
    Value1 = 0,
    #[doc = "1: K1-Divider operate with the new value"]
    Value2 = 1,
}
impl From<K1rdy> for bool {
    #[inline(always)]
    fn from(variant: K1rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K1RDY` reader - K1 Divider Ready Status"]
pub type K1rdyR = crate::BitReader<K1rdy>;
impl K1rdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> K1rdy {
        match self.bits {
            false => K1rdy::Value1,
            true => K1rdy::Value2,
        }
    }
    #[doc = "K1-Divider does not operate with the new value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == K1rdy::Value1
    }
    #[doc = "K1-Divider operate with the new value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == K1rdy::Value2
    }
}
#[doc = "K2 Divider Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum K2rdy {
    #[doc = "0: K2-Divider does not operate with the new value"]
    Value1 = 0,
    #[doc = "1: K2-Divider operate with the new value"]
    Value2 = 1,
}
impl From<K2rdy> for bool {
    #[inline(always)]
    fn from(variant: K2rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `K2RDY` reader - K2 Divider Ready Status"]
pub type K2rdyR = crate::BitReader<K2rdy>;
impl K2rdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> K2rdy {
        match self.bits {
            false => K2rdy::Value1,
            true => K2rdy::Value2,
        }
    }
    #[doc = "K2-Divider does not operate with the new value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == K2rdy::Value1
    }
    #[doc = "K2-Divider operate with the new value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == K2rdy::Value2
    }
}
#[doc = "Bypass Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum By {
    #[doc = "0: Bypass Mode is not entered"]
    Value1 = 0,
    #[doc = "1: Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    Value2 = 1,
}
impl From<By> for bool {
    #[inline(always)]
    fn from(variant: By) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BY` reader - Bypass Mode Status"]
pub type ByR = crate::BitReader<By>;
impl ByR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> By {
        match self.bits {
            false => By::Value1,
            true => By::Value2,
        }
    }
    #[doc = "Bypass Mode is not entered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == By::Value1
    }
    #[doc = "Bypass Mode is entered. Input fOSC is selected as output fPLL."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == By::Value2
    }
}
#[doc = "Oscillator for PLL Valid Low Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plllv {
    #[doc = "0: The OSC frequency is not usable. Frequency fREF is too low."]
    Value1 = 0,
    #[doc = "1: The OSC frequency is usable"]
    Value2 = 1,
}
impl From<Plllv> for bool {
    #[inline(always)]
    fn from(variant: Plllv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLLV` reader - Oscillator for PLL Valid Low Status Bit"]
pub type PlllvR = crate::BitReader<Plllv>;
impl PlllvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Plllv {
        match self.bits {
            false => Plllv::Value1,
            true => Plllv::Value2,
        }
    }
    #[doc = "The OSC frequency is not usable. Frequency fREF is too low."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Plllv::Value1
    }
    #[doc = "The OSC frequency is usable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Plllv::Value2
    }
}
#[doc = "Oscillator for PLL Valid High Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllhv {
    #[doc = "0: The OSC frequency is not usable. Frequency fOSC is too high."]
    Value1 = 0,
    #[doc = "1: The OSC frequency is usable"]
    Value2 = 1,
}
impl From<Pllhv> for bool {
    #[inline(always)]
    fn from(variant: Pllhv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLHV` reader - Oscillator for PLL Valid High Status Bit"]
pub type PllhvR = crate::BitReader<Pllhv>;
impl PllhvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllhv {
        match self.bits {
            false => Pllhv::Value1,
            true => Pllhv::Value2,
        }
    }
    #[doc = "The OSC frequency is not usable. Frequency fOSC is too high."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pllhv::Value1
    }
    #[doc = "The OSC frequency is usable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pllhv::Value2
    }
}
#[doc = "Oscillator for PLL Valid Spike Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllsp {
    #[doc = "0: The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    Value1 = 0,
    #[doc = "1: The OSC frequency is usable"]
    Value2 = 1,
}
impl From<Pllsp> for bool {
    #[inline(always)]
    fn from(variant: Pllsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSP` reader - Oscillator for PLL Valid Spike Status Bit"]
pub type PllspR = crate::BitReader<Pllsp>;
impl PllspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllsp {
        match self.bits {
            false => Pllsp::Value1,
            true => Pllsp::Value2,
        }
    }
    #[doc = "The OSC frequency is not usable. Spikes are detected that disturb a locked operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pllsp::Value1
    }
    #[doc = "The OSC frequency is usable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pllsp::Value2
    }
}
impl R {
    #[doc = "Bit 0 - VCO Bypass Status"]
    #[inline(always)]
    pub fn vcobyst(&self) -> VcobystR {
        VcobystR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Power-saving Mode Status"]
    #[inline(always)]
    pub fn pwdstat(&self) -> PwdstatR {
        PwdstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL LOCK Status"]
    #[inline(always)]
    pub fn vcolock(&self) -> VcolockR {
        VcolockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - K1 Divider Ready Status"]
    #[inline(always)]
    pub fn k1rdy(&self) -> K1rdyR {
        K1rdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - K2 Divider Ready Status"]
    #[inline(always)]
    pub fn k2rdy(&self) -> K2rdyR {
        K2rdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline(always)]
    pub fn by(&self) -> ByR {
        ByR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Oscillator for PLL Valid Low Status Bit"]
    #[inline(always)]
    pub fn plllv(&self) -> PlllvR {
        PlllvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Oscillator for PLL Valid High Status Bit"]
    #[inline(always)]
    pub fn pllhv(&self) -> PllhvR {
        PllhvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Oscillator for PLL Valid Spike Status Bit"]
    #[inline(always)]
    pub fn pllsp(&self) -> PllspR {
        PllspR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllstatSpec;
impl crate::RegisterSpec for PllstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstat::R`](R) reader structure"]
impl crate::Readable for PllstatSpec {}
#[doc = "`reset()` method sets PLLSTAT to value 0x02"]
impl crate::Resettable for PllstatSpec {
    const RESET_VALUE: u32 = 0x02;
}
