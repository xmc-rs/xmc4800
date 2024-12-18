#[doc = "Register `EEP_STATE` reader"]
pub type R = crate::R<EEP_STATE_SPEC>;
#[doc = "Register `EEP_STATE` writer"]
pub type W = crate::W<EEP_STATE_SPEC>;
#[doc = "Access to EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCESS_A {
    #[doc = "0: PDI releases EEPROM access"]
    VALUE1 = 0,
    #[doc = "1: PDI takes EEPROM access (PDI has EEPROM control)"]
    VALUE2 = 1,
}
impl From<ACCESS_A> for bool {
    #[inline(always)]
    fn from(variant: ACCESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCESS` reader - Access to EEPROM"]
pub type ACCESS_R = crate::BitReader<ACCESS_A>;
impl ACCESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACCESS_A {
        match self.bits {
            false => ACCESS_A::VALUE1,
            true => ACCESS_A::VALUE2,
        }
    }
    #[doc = "PDI releases EEPROM access"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ACCESS_A::VALUE1
    }
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ACCESS_A::VALUE2
    }
}
#[doc = "Field `ACCESS` writer - Access to EEPROM"]
pub type ACCESS_W<'a, REG> = crate::BitWriter<'a, REG, ACCESS_A>;
impl<'a, REG> ACCESS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PDI releases EEPROM access"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ACCESS_A::VALUE1)
    }
    #[doc = "PDI takes EEPROM access (PDI has EEPROM control)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ACCESS_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Access to EEPROM"]
    #[inline(always)]
    pub fn access(&self) -> ACCESS_R {
        ACCESS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access to EEPROM"]
    #[inline(always)]
    pub fn access(&mut self) -> ACCESS_W<EEP_STATE_SPEC> {
        ACCESS_W::new(self, 0)
    }
}
#[doc = "EEPROM PDI Access State\n\nYou can [`read`](crate::Reg::read) this register and get [`eep_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEP_STATE_SPEC;
impl crate::RegisterSpec for EEP_STATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`eep_state::R`](R) reader structure"]
impl crate::Readable for EEP_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eep_state::W`](W) writer structure"]
impl crate::Writable for EEP_STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EEP_STATE to value 0"]
impl crate::Resettable for EEP_STATE_SPEC {
    const RESET_VALUE: u8 = 0;
}
