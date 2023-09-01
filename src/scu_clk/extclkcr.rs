#[doc = "Register `EXTCLKCR` reader"]
pub type R = crate::R<EXTCLKCR_SPEC>;
#[doc = "Register `EXTCLKCR` writer"]
pub type W = crate::W<EXTCLKCR_SPEC>;
#[doc = "Field `ECKSEL` reader - External Clock Selection Value"]
pub type ECKSEL_R = crate::FieldReader<ECKSEL_A>;
#[doc = "External Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECKSEL_A {
    #[doc = "0: fSYS clock"]
    VALUE1 = 0,
    #[doc = "2: fUSB clock"]
    VALUE3 = 2,
    #[doc = "3: fPLL clock divided according to ECKDIV bit field configuration"]
    VALUE4 = 3,
}
impl From<ECKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ECKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ECKSEL_A {
    type Ux = u8;
}
impl ECKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECKSEL_A> {
        match self.bits {
            0 => Some(ECKSEL_A::VALUE1),
            2 => Some(ECKSEL_A::VALUE3),
            3 => Some(ECKSEL_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECKSEL_A::VALUE1
    }
    #[doc = "fUSB clock"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ECKSEL_A::VALUE3
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ECKSEL_A::VALUE4
    }
}
#[doc = "Field `ECKSEL` writer - External Clock Selection Value"]
pub type ECKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ECKSEL_A>;
impl<'a, REG, const O: u8> ECKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fSYS clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ECKSEL_A::VALUE1)
    }
    #[doc = "fUSB clock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ECKSEL_A::VALUE3)
    }
    #[doc = "fPLL clock divided according to ECKDIV bit field configuration"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ECKSEL_A::VALUE4)
    }
}
#[doc = "Field `ECKDIV` reader - External Clock Divider Value"]
pub type ECKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `ECKDIV` writer - External Clock Divider Value"]
pub type ECKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    pub fn ecksel(&self) -> ECKSEL_R {
        ECKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    pub fn eckdiv(&self) -> ECKDIV_R {
        ECKDIV_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn ecksel(&mut self) -> ECKSEL_W<EXTCLKCR_SPEC, 0> {
        ECKSEL_W::new(self)
    }
    #[doc = "Bits 16:24 - External Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn eckdiv(&mut self) -> ECKDIV_W<EXTCLKCR_SPEC, 16> {
        ECKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTCLKCR_SPEC;
impl crate::RegisterSpec for EXTCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extclkcr::R`](R) reader structure"]
impl crate::Readable for EXTCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extclkcr::W`](W) writer structure"]
impl crate::Writable for EXTCLKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTCLKCR to value 0"]
impl crate::Resettable for EXTCLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
