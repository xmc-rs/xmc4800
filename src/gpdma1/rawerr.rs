#[doc = "Register `RAWERR` reader"]
pub type R = crate::R<RAWERR_SPEC>;
#[doc = "Register `RAWERR` writer"]
pub type W = crate::W<RAWERR_SPEC>;
#[doc = "Field `CH0` reader - Raw Interrupt Status for channel 0"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - Raw Interrupt Status for channel 0"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Raw Interrupt Status for channel 1"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - Raw Interrupt Status for channel 1"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Raw Interrupt Status for channel 2"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - Raw Interrupt Status for channel 2"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Raw Interrupt Status for channel 3"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - Raw Interrupt Status for channel 3"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<RAWERR_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<RAWERR_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<RAWERR_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<RAWERR_SPEC> {
        CH3_W::new(self, 3)
    }
}
#[doc = "Raw IntErr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWERR_SPEC;
impl crate::RegisterSpec for RAWERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawerr::R`](R) reader structure"]
impl crate::Readable for RAWERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rawerr::W`](W) writer structure"]
impl crate::Writable for RAWERR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAWERR to value 0"]
impl crate::Resettable for RAWERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
