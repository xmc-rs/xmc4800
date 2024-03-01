#[doc = "Register `HDSTAT` reader"]
pub type R = crate::R<HdstatSpec>;
#[doc = "Wake-up Pin Event Positive Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epev {
    #[doc = "0: Wake-up on positive edge pin event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on positive edge pin event active"]
    Value2 = 1,
}
impl From<Epev> for bool {
    #[inline(always)]
    fn from(variant: Epev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` reader - Wake-up Pin Event Positive Edge"]
pub type EpevR = crate::BitReader<Epev>;
impl EpevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epev {
        match self.bits {
            false => Epev::Value1,
            true => Epev::Value2,
        }
    }
    #[doc = "Wake-up on positive edge pin event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Epev::Value1
    }
    #[doc = "Wake-up on positive edge pin event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Epev::Value2
    }
}
#[doc = "Wake-up Pin Event Negative Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enev {
    #[doc = "0: Wake-up on negative edge pin event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on negative edge pin event active"]
    Value2 = 1,
}
impl From<Enev> for bool {
    #[inline(always)]
    fn from(variant: Enev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` reader - Wake-up Pin Event Negative Edge"]
pub type EnevR = crate::BitReader<Enev>;
impl EnevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enev {
        match self.bits {
            false => Enev::Value1,
            true => Enev::Value2,
        }
    }
    #[doc = "Wake-up on negative edge pin event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Enev::Value1
    }
    #[doc = "Wake-up on negative edge pin event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Enev::Value2
    }
}
#[doc = "RTC Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcev {
    #[doc = "0: Wake-up on RTC event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on RTC event active"]
    Value2 = 1,
}
impl From<Rtcev> for bool {
    #[inline(always)]
    fn from(variant: Rtcev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` reader - RTC Event"]
pub type RtcevR = crate::BitReader<Rtcev>;
impl RtcevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcev {
        match self.bits {
            false => Rtcev::Value1,
            true => Rtcev::Value2,
        }
    }
    #[doc = "Wake-up on RTC event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rtcev::Value1
    }
    #[doc = "Wake-up on RTC event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rtcev::Value2
    }
}
#[doc = "ULP WDG Alarm Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdg {
    #[doc = "0: Watchdog alarm did not occur"]
    Value1 = 0,
    #[doc = "1: Watchdog alarm occurred"]
    Value2 = 1,
}
impl From<Ulpwdg> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` reader - ULP WDG Alarm Status"]
pub type UlpwdgR = crate::BitReader<Ulpwdg>;
impl UlpwdgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulpwdg {
        match self.bits {
            false => Ulpwdg::Value1,
            true => Ulpwdg::Value2,
        }
    }
    #[doc = "Watchdog alarm did not occur"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ulpwdg::Value1
    }
    #[doc = "Watchdog alarm occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ulpwdg::Value2
    }
}
#[doc = "Hibernate Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibnout {
    #[doc = "0: Hibernate not driven active to pads"]
    Value1 = 0,
    #[doc = "1: Hibernate driven active to pads"]
    Value2 = 1,
}
impl From<Hibnout> for bool {
    #[inline(always)]
    fn from(variant: Hibnout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNOUT` reader - Hibernate Control Status"]
pub type HibnoutR = crate::BitReader<Hibnout>;
impl HibnoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibnout {
        match self.bits {
            false => Hibnout::Value1,
            true => Hibnout::Value2,
        }
    }
    #[doc = "Hibernate not driven active to pads"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibnout::Value1
    }
    #[doc = "Hibernate driven active to pads"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibnout::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline(always)]
    pub fn epev(&self) -> EpevR {
        EpevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline(always)]
    pub fn enev(&self) -> EnevR {
        EnevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline(always)]
    pub fn rtcev(&self) -> RtcevR {
        RtcevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline(always)]
    pub fn ulpwdg(&self) -> UlpwdgR {
        UlpwdgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline(always)]
    pub fn hibnout(&self) -> HibnoutR {
        HibnoutR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdstatSpec;
impl crate::RegisterSpec for HdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdstat::R`](R) reader structure"]
impl crate::Readable for HdstatSpec {}
#[doc = "`reset()` method sets HDSTAT to value 0"]
impl crate::Resettable for HdstatSpec {
    const RESET_VALUE: u32 = 0;
}
