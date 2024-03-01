#[doc = "Register `SDRSTAT` reader"]
pub type R = crate::R<SdrstatSpec>;
#[doc = "SDRAM Refresh Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Referr {
    #[doc = "0: No refresh error."]
    Value1 = 0,
    #[doc = "1: Refresh error occurred."]
    Value2 = 1,
}
impl From<Referr> for bool {
    #[inline(always)]
    fn from(variant: Referr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFERR` reader - SDRAM Refresh Error"]
pub type ReferrR = crate::BitReader<Referr>;
impl ReferrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Referr {
        match self.bits {
            false => Referr::Value1,
            true => Referr::Value2,
        }
    }
    #[doc = "No refresh error."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Referr::Value1
    }
    #[doc = "Refresh error occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Referr::Value2
    }
}
#[doc = "SDRAM Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdrmbusy {
    #[doc = "0: Power-up initialization sequence is not running"]
    Value1 = 0,
    #[doc = "1: Power-up initialization sequence is running"]
    Value2 = 1,
}
impl From<Sdrmbusy> for bool {
    #[inline(always)]
    fn from(variant: Sdrmbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDRMBUSY` reader - SDRAM Busy"]
pub type SdrmbusyR = crate::BitReader<Sdrmbusy>;
impl SdrmbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdrmbusy {
        match self.bits {
            false => Sdrmbusy::Value1,
            true => Sdrmbusy::Value2,
        }
    }
    #[doc = "Power-up initialization sequence is not running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdrmbusy::Value1
    }
    #[doc = "Power-up initialization sequence is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sdrmbusy::Value2
    }
}
#[doc = "SDRAM read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sderr {
    #[doc = "0: Reads running successfully"]
    Value1 = 0,
    #[doc = "1: Read error condition has been detected"]
    Value2 = 1,
}
impl From<Sderr> for bool {
    #[inline(always)]
    fn from(variant: Sderr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDERR` reader - SDRAM read error"]
pub type SderrR = crate::BitReader<Sderr>;
impl SderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sderr {
        match self.bits {
            false => Sderr::Value1,
            true => Sderr::Value2,
        }
    }
    #[doc = "Reads running successfully"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sderr::Value1
    }
    #[doc = "Read error condition has been detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sderr::Value2
    }
}
impl R {
    #[doc = "Bit 0 - SDRAM Refresh Error"]
    #[inline(always)]
    pub fn referr(&self) -> ReferrR {
        ReferrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDRAM Busy"]
    #[inline(always)]
    pub fn sdrmbusy(&self) -> SdrmbusyR {
        SdrmbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDRAM read error"]
    #[inline(always)]
    pub fn sderr(&self) -> SderrR {
        SderrR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "EBU SDRAM Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrstatSpec;
impl crate::RegisterSpec for SdrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrstat::R`](R) reader structure"]
impl crate::Readable for SdrstatSpec {}
#[doc = "`reset()` method sets SDRSTAT to value 0x0001_0000"]
impl crate::Resettable for SdrstatSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
