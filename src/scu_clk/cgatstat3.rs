#[doc = "Register `CGATSTAT3` reader"]
pub type R = crate::R<Cgatstat3Spec>;
#[doc = "EBU Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebu {
    #[doc = "0: Gating de-asserted"]
    Value1 = 0,
    #[doc = "1: Gating asserted"]
    Value2 = 1,
}
impl From<Ebu> for bool {
    #[inline(always)]
    fn from(variant: Ebu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBU` reader - EBU Gating Status"]
pub type EbuR = crate::BitReader<Ebu>;
impl EbuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ebu {
        match self.bits {
            false => Ebu::Value1,
            true => Ebu::Value2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ebu::Value1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ebu::Value2
    }
}
impl R {
    #[doc = "Bit 2 - EBU Gating Status"]
    #[inline(always)]
    pub fn ebu(&self) -> EbuR {
        EbuR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Peripheral 3 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatstat3Spec;
impl crate::RegisterSpec for Cgatstat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat3::R`](R) reader structure"]
impl crate::Readable for Cgatstat3Spec {}
#[doc = "`reset()` method sets CGATSTAT3 to value 0"]
impl crate::Resettable for Cgatstat3Spec {
    const RESET_VALUE: u32 = 0;
}
