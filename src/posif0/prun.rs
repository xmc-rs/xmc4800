#[doc = "Register `PRUN` reader"]
pub type R = crate::R<PrunSpec>;
#[doc = "Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rb {
    #[doc = "0: IDLE"]
    Value1 = 0,
    #[doc = "1: Running"]
    Value2 = 1,
}
impl From<Rb> for bool {
    #[inline(always)]
    fn from(variant: Rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RB` reader - Run Bit"]
pub type RbR = crate::BitReader<Rb>;
impl RbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rb {
        match self.bits {
            false => Rb::Value1,
            true => Rb::Value2,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rb::Value1
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rb::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Run Bit"]
    #[inline(always)]
    pub fn rb(&self) -> RbR {
        RbR::new((self.bits & 1) != 0)
    }
}
#[doc = "POSIF Run Bit Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prun::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrunSpec;
impl crate::RegisterSpec for PrunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prun::R`](R) reader structure"]
impl crate::Readable for PrunSpec {}
#[doc = "`reset()` method sets PRUN to value 0"]
impl crate::Resettable for PrunSpec {
    const RESET_VALUE: u32 = 0;
}
