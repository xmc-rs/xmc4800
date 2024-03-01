#[doc = "Register `MCMF` reader"]
pub type R = crate::R<McmfSpec>;
#[doc = "Multi-Channel Pattern update status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mss {
    #[doc = "0: Update of the Multi-Channel pattern is set"]
    Value1 = 0,
    #[doc = "1: Update of the Multi-Channel pattern is not set"]
    Value2 = 1,
}
impl From<Mss> for bool {
    #[inline(always)]
    fn from(variant: Mss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSS` reader - Multi-Channel Pattern update status"]
pub type MssR = crate::BitReader<Mss>;
impl MssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mss {
        match self.bits {
            false => Mss::Value1,
            true => Mss::Value2,
        }
    }
    #[doc = "Update of the Multi-Channel pattern is set"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mss::Value1
    }
    #[doc = "Update of the Multi-Channel pattern is not set"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mss::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Multi-Channel Pattern update status"]
    #[inline(always)]
    pub fn mss(&self) -> MssR {
        MssR::new((self.bits & 1) != 0)
    }
}
#[doc = "Multi-Channel Pattern Control flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McmfSpec;
impl crate::RegisterSpec for McmfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmf::R`](R) reader structure"]
impl crate::Readable for McmfSpec {}
#[doc = "`reset()` method sets MCMF to value 0"]
impl crate::Resettable for McmfSpec {
    const RESET_VALUE: u32 = 0;
}
