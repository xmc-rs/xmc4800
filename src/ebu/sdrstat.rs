#[doc = "Register `SDRSTAT` reader"]
pub type R = crate::R<SDRSTAT_SPEC>;
#[doc = "SDRAM Refresh Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFERR_A {
    #[doc = "0: No refresh error."]
    VALUE1 = 0,
    #[doc = "1: Refresh error occurred."]
    VALUE2 = 1,
}
impl From<REFERR_A> for bool {
    #[inline(always)]
    fn from(variant: REFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFERR` reader - SDRAM Refresh Error"]
pub type REFERR_R = crate::BitReader<REFERR_A>;
impl REFERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFERR_A {
        match self.bits {
            false => REFERR_A::VALUE1,
            true => REFERR_A::VALUE2,
        }
    }
    #[doc = "No refresh error."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFERR_A::VALUE1
    }
    #[doc = "Refresh error occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFERR_A::VALUE2
    }
}
#[doc = "SDRAM Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDRMBUSY_A {
    #[doc = "0: Power-up initialization sequence is not running"]
    VALUE1 = 0,
    #[doc = "1: Power-up initialization sequence is running"]
    VALUE2 = 1,
}
impl From<SDRMBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SDRMBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDRMBUSY` reader - SDRAM Busy"]
pub type SDRMBUSY_R = crate::BitReader<SDRMBUSY_A>;
impl SDRMBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDRMBUSY_A {
        match self.bits {
            false => SDRMBUSY_A::VALUE1,
            true => SDRMBUSY_A::VALUE2,
        }
    }
    #[doc = "Power-up initialization sequence is not running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDRMBUSY_A::VALUE1
    }
    #[doc = "Power-up initialization sequence is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDRMBUSY_A::VALUE2
    }
}
#[doc = "SDRAM read error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDERR_A {
    #[doc = "0: Reads running successfully"]
    VALUE1 = 0,
    #[doc = "1: Read error condition has been detected"]
    VALUE2 = 1,
}
impl From<SDERR_A> for bool {
    #[inline(always)]
    fn from(variant: SDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDERR` reader - SDRAM read error"]
pub type SDERR_R = crate::BitReader<SDERR_A>;
impl SDERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDERR_A {
        match self.bits {
            false => SDERR_A::VALUE1,
            true => SDERR_A::VALUE2,
        }
    }
    #[doc = "Reads running successfully"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDERR_A::VALUE1
    }
    #[doc = "Read error condition has been detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDERR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - SDRAM Refresh Error"]
    #[inline(always)]
    pub fn referr(&self) -> REFERR_R {
        REFERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDRAM Busy"]
    #[inline(always)]
    pub fn sdrmbusy(&self) -> SDRMBUSY_R {
        SDRMBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SDRAM read error"]
    #[inline(always)]
    pub fn sderr(&self) -> SDERR_R {
        SDERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "EBU SDRAM Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDRSTAT_SPEC;
impl crate::RegisterSpec for SDRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrstat::R`](R) reader structure"]
impl crate::Readable for SDRSTAT_SPEC {}
#[doc = "`reset()` method sets SDRSTAT to value 0x0001_0000"]
impl crate::Resettable for SDRSTAT_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
