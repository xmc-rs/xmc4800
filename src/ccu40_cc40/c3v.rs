#[doc = "Register `C3V` reader"]
pub type R = crate::R<C3V_SPEC>;
#[doc = "Field `CAPTV` reader - Capture Value"]
pub type CAPTV_R = crate::FieldReader<u16>;
#[doc = "Field `FPCV` reader - Prescaler Value"]
pub type FPCV_R = crate::FieldReader;
#[doc = "Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFL_A {
    #[doc = "0: No new value was captured into the specific capture register"]
    VALUE1 = 0,
    #[doc = "1: A new value was captured into the specific register"]
    VALUE2 = 1,
}
impl From<FFL_A> for bool {
    #[inline(always)]
    fn from(variant: FFL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFL` reader - Full Flag"]
pub type FFL_R = crate::BitReader<FFL_A>;
impl FFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FFL_A {
        match self.bits {
            false => FFL_A::VALUE1,
            true => FFL_A::VALUE2,
        }
    }
    #[doc = "No new value was captured into the specific capture register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FFL_A::VALUE1
    }
    #[doc = "A new value was captured into the specific register"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FFL_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture Value"]
    #[inline(always)]
    pub fn captv(&self) -> CAPTV_R {
        CAPTV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Prescaler Value"]
    #[inline(always)]
    pub fn fpcv(&self) -> FPCV_R {
        FPCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Full Flag"]
    #[inline(always)]
    pub fn ffl(&self) -> FFL_R {
        FFL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Capture Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3v::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3V_SPEC;
impl crate::RegisterSpec for C3V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3v::R`](R) reader structure"]
impl crate::Readable for C3V_SPEC {}
#[doc = "`reset()` method sets C3V to value 0"]
impl crate::Resettable for C3V_SPEC {
    const RESET_VALUE: u32 = 0;
}
