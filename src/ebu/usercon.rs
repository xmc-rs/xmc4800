#[doc = "Register `USERCON` reader"]
pub type R = crate::R<UserconSpec>;
#[doc = "Register `USERCON` writer"]
pub type W = crate::W<UserconSpec>;
#[doc = "Field `DIP` reader - Disable Internal Pipelining"]
pub type DipR = crate::BitReader;
#[doc = "Field `DIP` writer - Disable Internal Pipelining"]
pub type DipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Address Pins to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Addio {
    #[doc = "0: Address Bit is required for addressing memory"]
    Value1 = 0,
    #[doc = "1: Address Bit is available for GPIO function"]
    Value2 = 1,
}
impl From<Addio> for u16 {
    #[inline(always)]
    fn from(variant: Addio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Addio {
    type Ux = u16;
}
#[doc = "Field `ADDIO` reader - Address Pins to GPIO Mode"]
pub type AddioR = crate::FieldReader<Addio>;
impl AddioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Addio> {
        match self.bits {
            0 => Some(Addio::Value1),
            1 => Some(Addio::Value2),
            _ => None,
        }
    }
    #[doc = "Address Bit is required for addressing memory"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Addio::Value1
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Addio::Value2
    }
}
#[doc = "Field `ADDIO` writer - Address Pins to GPIO Mode"]
pub type AddioW<'a, REG> = crate::FieldWriter<'a, REG, 9, Addio>;
impl<'a, REG> AddioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Address Bit is required for addressing memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Addio::Value1)
    }
    #[doc = "Address Bit is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Addio::Value2)
    }
}
#[doc = "ADV Pin to GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advio {
    #[doc = "0: ADV pin is required for controlling memory"]
    Value1 = 0,
    #[doc = "1: ADV pin is available for GPIO function"]
    Value2 = 1,
}
impl From<Advio> for bool {
    #[inline(always)]
    fn from(variant: Advio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVIO` reader - ADV Pin to GPIO Mode"]
pub type AdvioR = crate::BitReader<Advio>;
impl AdvioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Advio {
        match self.bits {
            false => Advio::Value1,
            true => Advio::Value2,
        }
    }
    #[doc = "ADV pin is required for controlling memory"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Advio::Value1
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Advio::Value2
    }
}
#[doc = "Field `ADVIO` writer - ADV Pin to GPIO Mode"]
pub type AdvioW<'a, REG> = crate::BitWriter<'a, REG, Advio>;
impl<'a, REG> AdvioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADV pin is required for controlling memory"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Advio::Value1)
    }
    #[doc = "ADV pin is available for GPIO function"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Advio::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    pub fn dip(&self) -> DipR {
        DipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    pub fn addio(&self) -> AddioR {
        AddioR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    pub fn advio(&self) -> AdvioR {
        AdvioR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Internal Pipelining"]
    #[inline(always)]
    #[must_use]
    pub fn dip(&mut self) -> DipW<UserconSpec> {
        DipW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Address Pins to GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn addio(&mut self) -> AddioW<UserconSpec> {
        AddioW::new(self, 16)
    }
    #[doc = "Bit 25 - ADV Pin to GPIO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn advio(&mut self) -> AdvioW<UserconSpec> {
        AdvioW::new(self, 25)
    }
}
#[doc = "EBU Test/Control Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usercon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usercon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserconSpec;
impl crate::RegisterSpec for UserconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usercon::R`](R) reader structure"]
impl crate::Readable for UserconSpec {}
#[doc = "`write(|w| ..)` method takes [`usercon::W`](W) writer structure"]
impl crate::Writable for UserconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USERCON to value 0"]
impl crate::Resettable for UserconSpec {
    const RESET_VALUE: u32 = 0;
}
