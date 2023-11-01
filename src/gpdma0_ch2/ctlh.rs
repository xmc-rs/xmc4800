#[doc = "Register `CTLH` reader"]
pub type R = crate::R<CTLH_SPEC>;
#[doc = "Register `CTLH` writer"]
pub type W = crate::W<CTLH_SPEC>;
#[doc = "Field `BLOCK_TS` reader - Block Transfer Size"]
pub type BLOCK_TS_R = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_TS` writer - Block Transfer Size"]
pub type BLOCK_TS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DONE` reader - Done bit"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Done bit"]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BLOCK_TS_R {
        BLOCK_TS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Block Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn block_ts(&mut self) -> BLOCK_TS_W<CTLH_SPEC, 0> {
        BLOCK_TS_W::new(self)
    }
    #[doc = "Bit 12 - Done bit"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<CTLH_SPEC, 12> {
        DONE_W::new(self)
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
#[doc = "Control Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLH_SPEC;
impl crate::RegisterSpec for CTLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlh::R`](R) reader structure"]
impl crate::Readable for CTLH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlh::W`](W) writer structure"]
impl crate::Writable for CTLH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLH to value 0x02"]
impl crate::Resettable for CTLH_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
