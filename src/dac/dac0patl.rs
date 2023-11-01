#[doc = "Register `DAC0PATL` reader"]
pub type R = crate::R<DAC0PATL_SPEC>;
#[doc = "Register `DAC0PATL` writer"]
pub type W = crate::W<DAC0PATL_SPEC>;
#[doc = "Field `PAT0` reader - Pattern Number 0 for PATGEN of DAC0"]
pub type PAT0_R = crate::FieldReader;
#[doc = "Field `PAT0` writer - Pattern Number 0 for PATGEN of DAC0"]
pub type PAT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT1` reader - Pattern Number 1 for PATGEN of DAC0"]
pub type PAT1_R = crate::FieldReader;
#[doc = "Field `PAT1` writer - Pattern Number 1 for PATGEN of DAC0"]
pub type PAT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT2` reader - Pattern Number 2 for PATGEN of DAC0"]
pub type PAT2_R = crate::FieldReader;
#[doc = "Field `PAT2` writer - Pattern Number 2 for PATGEN of DAC0"]
pub type PAT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT3` reader - Pattern Number 3 for PATGEN of DAC0"]
pub type PAT3_R = crate::FieldReader;
#[doc = "Field `PAT3` writer - Pattern Number 3 for PATGEN of DAC0"]
pub type PAT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT4` reader - Pattern Number 4 for PATGEN of DAC0"]
pub type PAT4_R = crate::FieldReader;
#[doc = "Field `PAT4` writer - Pattern Number 4 for PATGEN of DAC0"]
pub type PAT4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PAT5` reader - Pattern Number 5 for PATGEN of DAC0"]
pub type PAT5_R = crate::FieldReader;
#[doc = "Field `PAT5` writer - Pattern Number 5 for PATGEN of DAC0"]
pub type PAT5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat0(&self) -> PAT0_R {
        PAT0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat1(&self) -> PAT1_R {
        PAT1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat2(&self) -> PAT2_R {
        PAT2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat3(&self) -> PAT3_R {
        PAT3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat4(&self) -> PAT4_R {
        PAT4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC0"]
    #[inline(always)]
    pub fn pat5(&self) -> PAT5_R {
        PAT5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 0 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat0(&mut self) -> PAT0_W<DAC0PATL_SPEC, 0> {
        PAT0_W::new(self)
    }
    #[doc = "Bits 5:9 - Pattern Number 1 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat1(&mut self) -> PAT1_W<DAC0PATL_SPEC, 5> {
        PAT1_W::new(self)
    }
    #[doc = "Bits 10:14 - Pattern Number 2 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat2(&mut self) -> PAT2_W<DAC0PATL_SPEC, 10> {
        PAT2_W::new(self)
    }
    #[doc = "Bits 15:19 - Pattern Number 3 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat3(&mut self) -> PAT3_W<DAC0PATL_SPEC, 15> {
        PAT3_W::new(self)
    }
    #[doc = "Bits 20:24 - Pattern Number 4 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat4(&mut self) -> PAT4_W<DAC0PATL_SPEC, 20> {
        PAT4_W::new(self)
    }
    #[doc = "Bits 25:29 - Pattern Number 5 for PATGEN of DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn pat5(&mut self) -> PAT5_W<DAC0PATL_SPEC, 25> {
        PAT5_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC0 Lower Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0patl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0patl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC0PATL_SPEC;
impl crate::RegisterSpec for DAC0PATL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0patl::R`](R) reader structure"]
impl crate::Readable for DAC0PATL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dac0patl::W`](W) writer structure"]
impl crate::Writable for DAC0PATL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC0PATL to value 0x3568_b0c0"]
impl crate::Resettable for DAC0PATL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3568_b0c0;
}
