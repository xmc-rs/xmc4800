#[doc = "Register `PRSTAT3` reader"]
pub type R = crate::R<Prstat3Spec>;
#[doc = "EBU Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eburs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Eburs> for bool {
    #[inline(always)]
    fn from(variant: Eburs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBURS` reader - EBU Reset Status"]
pub type EbursR = crate::BitReader<Eburs>;
impl EbursR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eburs {
        match self.bits {
            false => Eburs::Value1,
            true => Eburs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eburs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eburs::Value2
    }
}
impl R {
    #[doc = "Bit 2 - EBU Reset Status"]
    #[inline(always)]
    pub fn eburs(&self) -> EbursR {
        EbursR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 3 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstat3Spec;
impl crate::RegisterSpec for Prstat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat3::R`](R) reader structure"]
impl crate::Readable for Prstat3Spec {}
#[doc = "`reset()` method sets PRSTAT3 to value 0x04"]
impl crate::Resettable for Prstat3Spec {
    const RESET_VALUE: u32 = 0x04;
}
