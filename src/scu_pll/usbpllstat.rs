#[doc = "Register `USBPLLSTAT` reader"]
pub type R = crate::R<UsbpllstatSpec>;
#[doc = "VCO Bypass Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcobyst {
    #[doc = "0: Normal Mode is entered"]
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
    #[doc = "Normal Mode is entered"]
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
#[doc = "PLL VCO Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcolock {
    #[doc = "0: The frequency difference of fREF and fDIV is greater than allowed. The VCO part of the PLL can not lock on a target frequency."]
    Value1 = 0,
    #[doc = "1: The frequency difference of fREF and fDIV is small enough to enable a stable VCO operation"]
    Value2 = 1,
}
impl From<Vcolock> for bool {
    #[inline(always)]
    fn from(variant: Vcolock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOLOCK` reader - PLL VCO Lock Status"]
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
    #[doc = "The frequency difference of fREF and fDIV is greater than allowed. The VCO part of the PLL can not lock on a target frequency."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcolock::Value1
    }
    #[doc = "The frequency difference of fREF and fDIV is small enough to enable a stable VCO operation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcolock::Value2
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
#[doc = "PLL LOCK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcolocked {
    #[doc = "0: PLL not locked"]
    Value1 = 0,
    #[doc = "1: PLL locked"]
    Value2 = 1,
}
impl From<Vcolocked> for bool {
    #[inline(always)]
    fn from(variant: Vcolocked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOLOCKED` reader - PLL LOCK Status"]
pub type VcolockedR = crate::BitReader<Vcolocked>;
impl VcolockedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcolocked {
        match self.bits {
            false => Vcolocked::Value1,
            true => Vcolocked::Value2,
        }
    }
    #[doc = "PLL not locked"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcolocked::Value1
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcolocked::Value2
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
    #[doc = "Bit 2 - PLL VCO Lock Status"]
    #[inline(always)]
    pub fn vcolock(&self) -> VcolockR {
        VcolockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Bypass Mode Status"]
    #[inline(always)]
    pub fn by(&self) -> ByR {
        ByR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL LOCK Status"]
    #[inline(always)]
    pub fn vcolocked(&self) -> VcolockedR {
        VcolockedR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB PLL Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbpllstatSpec;
impl crate::RegisterSpec for UsbpllstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbpllstat::R`](R) reader structure"]
impl crate::Readable for UsbpllstatSpec {}
#[doc = "`reset()` method sets USBPLLSTAT to value 0x02"]
impl crate::Resettable for UsbpllstatSpec {
    const RESET_VALUE: u32 = 0x02;
}
