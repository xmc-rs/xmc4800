#[doc = "Register `USERCON` reader"]
pub type R = crate::R<USERCON_SPEC>;
#[doc = "Register `USERCON` writer"]
pub type W = crate::W<USERCON_SPEC>;
#[doc = "Field `DIP` reader - Disable Internal Pipelining"]
pub type DIP_R = crate::BitReader;
#[doc = "Field `DIP` writer - Disable Internal Pipelining"]
pub type DIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address Pins to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ADDIO_A {
    #[doc = "0: Address Bit is required for addressing memory"]
    VALUE1 = 0,
    #[doc = "1: Address Bit is available for GPIO function"]
    VALUE2 = 1,
}
impl From<ADDIO_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDIO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADDIO_A {
    type Ux = u16;
}
impl crate::IsEnum for ADDIO_A {}
#[doc = "Field `ADDIO` reader - Address Pins to GPIO Mode"]
pub type ADDIO_R = crate::FieldReader<ADDIO_A>;
impl ADDIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADDIO_A> {
        match self.bits {
            0 => Some(ADDIO_A::VALUE1),
            1 => Some(ADDIO_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Address Bit is required for addressing memory"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADDIO_A::VALUE1
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADDIO_A::VALUE2
    }
}
#[doc = "Field `ADDIO` writer - Address Pins to GPIO Mode"]
pub type ADDIO_W<'a, REG> = crate::FieldWriter<'a, REG, 9, ADDIO_A>;
impl<'a, REG> ADDIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Address Bit is required for addressing memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDIO_A::VALUE1)
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ADDIO_A::VALUE2)
    }
}
#[doc = "ADV Pin to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVIO_A {
    #[doc = "0: ADV pin is required for controlling memory"]
    VALUE1 = 0,
    #[doc = "1: ADV pin is available for GPIO function"]
    VALUE2 = 1,
}
impl From<ADVIO_A> for bool {
    #[inline(always)]
    fn from(variant: ADVIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVIO` reader - ADV Pin to GPIO Mode"]
pub type ADVIO_R = crate::BitReader<ADVIO_A>;
impl ADVIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADVIO_A {
        match self.bits {
            false => ADVIO_A::VALUE1,
            true => ADVIO_A::VALUE2,
        }
    }
    #[doc = "ADV pin is required for controlling memory"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ADVIO_A::VALUE1
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ADVIO_A::VALUE2
    }
}
#[doc = "Field `ADVIO` writer - ADV Pin to GPIO Mode"]
pub type ADVIO_W<'a, REG> = crate::BitWriter<'a, REG, ADVIO_A>;
impl<'a, REG> ADVIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADV pin is required for controlling memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ADVIO_A::VALUE1)
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ADVIO_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&self) -> DIP_R {
        DIP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&self) -> ADDIO_R {
        ADDIO_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&self) -> ADVIO_R {
        ADVIO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    #[must_use]
    pub fn dip(&mut self) -> DIP_W<USERCON_SPEC> {
        DIP_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn addio(&mut self) -> ADDIO_W<USERCON_SPEC> {
        ADDIO_W::new(self, 16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn advio(&mut self) -> ADVIO_W<USERCON_SPEC> {
        ADVIO_W::new(self, 25)
    }
}
#[doc = "EBU Test/Control Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usercon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usercon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USERCON_SPEC;
impl crate::RegisterSpec for USERCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usercon::R`](R) reader structure"]
impl crate::Readable for USERCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usercon::W`](W) writer structure"]
impl crate::Writable for USERCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USERCON to value 0"]
impl crate::Resettable for USERCON_SPEC {
    const RESET_VALUE: u32 = 0;
}
