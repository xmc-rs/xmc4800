#[doc = "Register `QDC` reader"]
pub type R = crate::R<QDC_SPEC>;
#[doc = "Register `QDC` writer"]
pub type W = crate::W<QDC_SPEC>;
#[doc = "Phase A Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PALS_A {
    #[doc = "0: Phase A is active HIGH"]
    VALUE1 = 0,
    #[doc = "1: Phase A is active LOW"]
    VALUE2 = 1,
}
impl From<PALS_A> for bool {
    #[inline(always)]
    fn from(variant: PALS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PALS` reader - Phase A Level selector"]
pub type PALS_R = crate::BitReader<PALS_A>;
impl PALS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PALS_A {
        match self.bits {
            false => PALS_A::VALUE1,
            true => PALS_A::VALUE2,
        }
    }
    #[doc = "Phase A is active HIGH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PALS_A::VALUE1
    }
    #[doc = "Phase A is active LOW"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PALS_A::VALUE2
    }
}
#[doc = "Field `PALS` writer - Phase A Level selector"]
pub type PALS_W<'a, REG> = crate::BitWriter<'a, REG, PALS_A>;
impl<'a, REG> PALS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase A is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PALS_A::VALUE1)
    }
    #[doc = "Phase A is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PALS_A::VALUE2)
    }
}
#[doc = "Phase B Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBLS_A {
    #[doc = "0: Phase B is active HIGH"]
    VALUE1 = 0,
    #[doc = "1: Phase B is active LOW"]
    VALUE2 = 1,
}
impl From<PBLS_A> for bool {
    #[inline(always)]
    fn from(variant: PBLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBLS` reader - Phase B Level selector"]
pub type PBLS_R = crate::BitReader<PBLS_A>;
impl PBLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PBLS_A {
        match self.bits {
            false => PBLS_A::VALUE1,
            true => PBLS_A::VALUE2,
        }
    }
    #[doc = "Phase B is active HIGH"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBLS_A::VALUE1
    }
    #[doc = "Phase B is active LOW"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBLS_A::VALUE2
    }
}
#[doc = "Field `PBLS` writer - Phase B Level selector"]
pub type PBLS_W<'a, REG> = crate::BitWriter<'a, REG, PBLS_A>;
impl<'a, REG> PBLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase B is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PBLS_A::VALUE1)
    }
    #[doc = "Phase B is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PBLS_A::VALUE2)
    }
}
#[doc = "Phase signals swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHS_A {
    #[doc = "0: Phase A is the leading signal for clockwise rotation"]
    VALUE1 = 0,
    #[doc = "1: Phase B is the leading signal for clockwise rotation"]
    VALUE2 = 1,
}
impl From<PHS_A> for bool {
    #[inline(always)]
    fn from(variant: PHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHS` reader - Phase signals swap"]
pub type PHS_R = crate::BitReader<PHS_A>;
impl PHS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PHS_A {
        match self.bits {
            false => PHS_A::VALUE1,
            true => PHS_A::VALUE2,
        }
    }
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PHS_A::VALUE1
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHS_A::VALUE2
    }
}
#[doc = "Field `PHS` writer - Phase signals swap"]
pub type PHS_W<'a, REG> = crate::BitWriter<'a, REG, PHS_A>;
impl<'a, REG> PHS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PHS_A::VALUE1)
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PHS_A::VALUE2)
    }
}
#[doc = "Index Marker generations control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ICM_A {
    #[doc = "0: No index marker generation on POSIFx.OUT3"]
    VALUE1 = 0,
    #[doc = "1: Only first index occurrence generated on POSIFx.OUT3"]
    VALUE2 = 1,
    #[doc = "2: All index occurrences generated on POSIFx.OUT3"]
    VALUE3 = 2,
}
impl From<ICM_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ICM_A {
    type Ux = u8;
}
impl crate::IsEnum for ICM_A {}
#[doc = "Field `ICM` reader - Index Marker generations control"]
pub type ICM_R = crate::FieldReader<ICM_A>;
impl ICM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ICM_A> {
        match self.bits {
            0 => Some(ICM_A::VALUE1),
            1 => Some(ICM_A::VALUE2),
            2 => Some(ICM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ICM_A::VALUE1
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ICM_A::VALUE2
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ICM_A::VALUE3
    }
}
#[doc = "Field `ICM` writer - Index Marker generations control"]
pub type ICM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ICM_A>;
impl<'a, REG> ICM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ICM_A::VALUE1)
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ICM_A::VALUE2)
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ICM_A::VALUE3)
    }
}
#[doc = "Current rotation direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVAL_A {
    #[doc = "0: Counterclockwise rotation"]
    VALUE1 = 0,
    #[doc = "1: Clockwise rotation"]
    VALUE2 = 1,
}
impl From<DVAL_A> for bool {
    #[inline(always)]
    fn from(variant: DVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVAL` reader - Current rotation direction"]
pub type DVAL_R = crate::BitReader<DVAL_A>;
impl DVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DVAL_A {
        match self.bits {
            false => DVAL_A::VALUE1,
            true => DVAL_A::VALUE2,
        }
    }
    #[doc = "Counterclockwise rotation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DVAL_A::VALUE1
    }
    #[doc = "Clockwise rotation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DVAL_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    pub fn pals(&self) -> PALS_R {
        PALS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    pub fn pbls(&self) -> PBLS_R {
        PBLS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    pub fn phs(&self) -> PHS_R {
        PHS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    pub fn icm(&self) -> ICM_R {
        ICM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Current rotation direction"]
    #[inline(always)]
    pub fn dval(&self) -> DVAL_R {
        DVAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    pub fn pals(&mut self) -> PALS_W<QDC_SPEC> {
        PALS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    pub fn pbls(&mut self) -> PBLS_W<QDC_SPEC> {
        PBLS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    pub fn phs(&mut self) -> PHS_W<QDC_SPEC> {
        PHS_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    pub fn icm(&mut self) -> ICM_W<QDC_SPEC> {
        ICM_W::new(self, 4)
    }
}
#[doc = "Quadrature Decoder Control\n\nYou can [`read`](crate::Reg::read) this register and get [`qdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QDC_SPEC;
impl crate::RegisterSpec for QDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdc::R`](R) reader structure"]
impl crate::Readable for QDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qdc::W`](W) writer structure"]
impl crate::Writable for QDC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDC to value 0"]
impl crate::Resettable for QDC_SPEC {
    const RESET_VALUE: u32 = 0;
}
