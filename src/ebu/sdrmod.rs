#[doc = "Register `SDRMOD` reader"]
pub type R = crate::R<SDRMOD_SPEC>;
#[doc = "Register `SDRMOD` writer"]
pub type W = crate::W<SDRMOD_SPEC>;
#[doc = "Field `BURSTL` reader - Burst length"]
pub type BURSTL_R = crate::FieldReader<BURSTL_A>;
#[doc = "Burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURSTL_A {
    #[doc = "0: 1 (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: 2"]
    VALUE2 = 1,
    #[doc = "2: 4"]
    VALUE3 = 2,
    #[doc = "3: 8"]
    VALUE4 = 3,
    #[doc = "4: 16"]
    VALUE5 = 4,
}
impl From<BURSTL_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BURSTL_A {
    type Ux = u8;
}
impl BURSTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BURSTL_A> {
        match self.bits {
            0 => Some(BURSTL_A::VALUE1),
            1 => Some(BURSTL_A::VALUE2),
            2 => Some(BURSTL_A::VALUE3),
            3 => Some(BURSTL_A::VALUE4),
            4 => Some(BURSTL_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "1 (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BURSTL_A::VALUE1
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BURSTL_A::VALUE2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BURSTL_A::VALUE3
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BURSTL_A::VALUE4
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BURSTL_A::VALUE5
    }
}
#[doc = "Field `BURSTL` writer - Burst length"]
pub type BURSTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, BURSTL_A>;
impl<'a, REG, const O: u8> BURSTL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTL_A::VALUE1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTL_A::VALUE2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTL_A::VALUE3)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTL_A::VALUE4)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(BURSTL_A::VALUE5)
    }
}
#[doc = "Field `BTYP` reader - Burst type"]
pub type BTYP_R = crate::BitReader<BTYP_A>;
#[doc = "Burst type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTYP_A {
    #[doc = "0: Only this value should be written (default after reset)"]
    VALUE1 = 0,
}
impl From<BTYP_A> for bool {
    #[inline(always)]
    fn from(variant: BTYP_A) -> Self {
        variant as u8 != 0
    }
}
impl BTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BTYP_A> {
        match self.bits {
            false => Some(BTYP_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Only this value should be written (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BTYP_A::VALUE1
    }
}
#[doc = "Field `BTYP` writer - Burst type"]
pub type BTYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, BTYP_A>;
impl<'a, REG, const O: u8> BTYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only this value should be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BTYP_A::VALUE1)
    }
}
#[doc = "Field `CASLAT` reader - CAS latency"]
pub type CASLAT_R = crate::FieldReader<CASLAT_A>;
#[doc = "CAS latency\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CASLAT_A {
    #[doc = "2: Two clocks (default after reset)"]
    VALUE1 = 2,
    #[doc = "3: Three clocks"]
    VALUE2 = 3,
}
impl From<CASLAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CASLAT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CASLAT_A {
    type Ux = u8;
}
impl CASLAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CASLAT_A> {
        match self.bits {
            2 => Some(CASLAT_A::VALUE1),
            3 => Some(CASLAT_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Two clocks (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CASLAT_A::VALUE1
    }
    #[doc = "Three clocks"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CASLAT_A::VALUE2
    }
}
#[doc = "Field `CASLAT` writer - CAS latency"]
pub type CASLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CASLAT_A>;
impl<'a, REG, const O: u8> CASLAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Two clocks (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CASLAT_A::VALUE1)
    }
    #[doc = "Three clocks"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CASLAT_A::VALUE2)
    }
}
#[doc = "Field `OPMODE` reader - Operation Mode"]
pub type OPMODE_R = crate::FieldReader<OPMODE_A>;
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: Only this value must be written (default after reset)"]
    VALUE1 = 0,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPMODE_A {
    type Ux = u8;
}
impl OPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OPMODE_A> {
        match self.bits {
            0 => Some(OPMODE_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Only this value must be written (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OPMODE_A::VALUE1
    }
}
#[doc = "Field `OPMODE` writer - Operation Mode"]
pub type OPMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, OPMODE_A>;
impl<'a, REG, const O: u8> OPMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only this value must be written (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODE_A::VALUE1)
    }
}
#[doc = "Field `COLDSTART` writer - SDRAM coldstart"]
pub type COLDSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XOPM` reader - Extended Operation Mode"]
pub type XOPM_R = crate::FieldReader<u16>;
#[doc = "Field `XOPM` writer - Extended Operation Mode"]
pub type XOPM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `XBA` reader - Extended Operation Bank Select"]
pub type XBA_R = crate::FieldReader;
#[doc = "Field `XBA` writer - Extended Operation Bank Select"]
pub type XBA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    pub fn burstl(&self) -> BURSTL_R {
        BURSTL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    pub fn btyp(&self) -> BTYP_R {
        BTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    pub fn caslat(&self) -> CASLAT_R {
        CASLAT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    pub fn xopm(&self) -> XOPM_R {
        XOPM_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    pub fn xba(&self) -> XBA_R {
        XBA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst length"]
    #[inline(always)]
    #[must_use]
    pub fn burstl(&mut self) -> BURSTL_W<SDRMOD_SPEC, 0> {
        BURSTL_W::new(self)
    }
    #[doc = "Bit 3 - Burst type"]
    #[inline(always)]
    #[must_use]
    pub fn btyp(&mut self) -> BTYP_W<SDRMOD_SPEC, 3> {
        BTYP_W::new(self)
    }
    #[doc = "Bits 4:6 - CAS latency"]
    #[inline(always)]
    #[must_use]
    pub fn caslat(&mut self) -> CASLAT_W<SDRMOD_SPEC, 4> {
        CASLAT_W::new(self)
    }
    #[doc = "Bits 7:13 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OPMODE_W<SDRMOD_SPEC, 7> {
        OPMODE_W::new(self)
    }
    #[doc = "Bit 15 - SDRAM coldstart"]
    #[inline(always)]
    #[must_use]
    pub fn coldstart(&mut self) -> COLDSTART_W<SDRMOD_SPEC, 15> {
        COLDSTART_W::new(self)
    }
    #[doc = "Bits 16:27 - Extended Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xopm(&mut self) -> XOPM_W<SDRMOD_SPEC, 16> {
        XOPM_W::new(self)
    }
    #[doc = "Bits 28:31 - Extended Operation Bank Select"]
    #[inline(always)]
    #[must_use]
    pub fn xba(&mut self) -> XBA_W<SDRMOD_SPEC, 28> {
        XBA_W::new(self)
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
#[doc = "EBU SDRAM Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDRMOD_SPEC;
impl crate::RegisterSpec for SDRMOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrmod::R`](R) reader structure"]
impl crate::Readable for SDRMOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdrmod::W`](W) writer structure"]
impl crate::Writable for SDRMOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDRMOD to value 0x20"]
impl crate::Resettable for SDRMOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
