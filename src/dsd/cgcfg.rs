#[doc = "Register `CGCFG` reader"]
pub type R = crate::R<CGCFG_SPEC>;
#[doc = "Register `CGCFG` writer"]
pub type W = crate::W<CGCFG_SPEC>;
#[doc = "Field `CGMOD` reader - Carrier Generator Operating Mode"]
pub type CGMOD_R = crate::FieldReader<CGMOD_A>;
#[doc = "Carrier Generator Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CGMOD_A {
    #[doc = "0: Stopped"]
    VALUE1 = 0,
    #[doc = "1: Square wave"]
    VALUE2 = 1,
    #[doc = "2: Triangle"]
    VALUE3 = 2,
    #[doc = "3: Sine wave"]
    VALUE4 = 3,
}
impl From<CGMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CGMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CGMOD_A {
    type Ux = u8;
}
impl CGMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CGMOD_A {
        match self.bits {
            0 => CGMOD_A::VALUE1,
            1 => CGMOD_A::VALUE2,
            2 => CGMOD_A::VALUE3,
            3 => CGMOD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CGMOD_A::VALUE1
    }
    #[doc = "Square wave"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CGMOD_A::VALUE2
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CGMOD_A::VALUE3
    }
    #[doc = "Sine wave"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CGMOD_A::VALUE4
    }
}
#[doc = "Field `CGMOD` writer - Carrier Generator Operating Mode"]
pub type CGMOD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CGMOD_A>;
impl<'a, REG> CGMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CGMOD_A::VALUE1)
    }
    #[doc = "Square wave"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CGMOD_A::VALUE2)
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CGMOD_A::VALUE3)
    }
    #[doc = "Sine wave"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CGMOD_A::VALUE4)
    }
}
#[doc = "Field `BREV` reader - Bit-Reverse PWM Generation"]
pub type BREV_R = crate::BitReader<BREV_A>;
#[doc = "Bit-Reverse PWM Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREV_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Bit-reverse mode"]
    VALUE2 = 1,
}
impl From<BREV_A> for bool {
    #[inline(always)]
    fn from(variant: BREV_A) -> Self {
        variant as u8 != 0
    }
}
impl BREV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BREV_A {
        match self.bits {
            false => BREV_A::VALUE1,
            true => BREV_A::VALUE2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BREV_A::VALUE1
    }
    #[doc = "Bit-reverse mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BREV_A::VALUE2
    }
}
#[doc = "Field `BREV` writer - Bit-Reverse PWM Generation"]
pub type BREV_W<'a, REG> = crate::BitWriter<'a, REG, BREV_A>;
impl<'a, REG> BREV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BREV_A::VALUE1)
    }
    #[doc = "Bit-reverse mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BREV_A::VALUE2)
    }
}
#[doc = "Field `SIGPOL` reader - Signal Polarity"]
pub type SIGPOL_R = crate::BitReader<SIGPOL_A>;
#[doc = "Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGPOL_A {
    #[doc = "0: Normal: carrier signal begins with +1"]
    VALUE1 = 0,
    #[doc = "1: Inverted: carrier signal begins with -1"]
    VALUE2 = 1,
}
impl From<SIGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SIGPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIGPOL_A {
        match self.bits {
            false => SIGPOL_A::VALUE1,
            true => SIGPOL_A::VALUE2,
        }
    }
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIGPOL_A::VALUE1
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIGPOL_A::VALUE2
    }
}
#[doc = "Field `SIGPOL` writer - Signal Polarity"]
pub type SIGPOL_W<'a, REG> = crate::BitWriter<'a, REG, SIGPOL_A>;
impl<'a, REG> SIGPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIGPOL_A::VALUE1)
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIGPOL_A::VALUE2)
    }
}
#[doc = "Field `DIVCG` reader - Divider Factor for the PWM Pattern Signal Generator"]
pub type DIVCG_R = crate::FieldReader<DIVCG_A>;
#[doc = "Divider Factor for the PWM Pattern Signal Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVCG_A {
    #[doc = "0: fCG = fCLK / 2"]
    VALUE1 = 0,
    #[doc = "1: fCG = fCLK / 4"]
    VALUE2 = 1,
    #[doc = "2: fCG = fCLK / 6"]
    VALUE3 = 2,
    #[doc = "15: fCG = fCLK / 32"]
    VALUE4 = 15,
}
impl From<DIVCG_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVCG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVCG_A {
    type Ux = u8;
}
impl DIVCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIVCG_A> {
        match self.bits {
            0 => Some(DIVCG_A::VALUE1),
            1 => Some(DIVCG_A::VALUE2),
            2 => Some(DIVCG_A::VALUE3),
            15 => Some(DIVCG_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "fCG = fCLK / 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVCG_A::VALUE1
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVCG_A::VALUE2
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVCG_A::VALUE3
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVCG_A::VALUE4
    }
}
#[doc = "Field `DIVCG` writer - Divider Factor for the PWM Pattern Signal Generator"]
pub type DIVCG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DIVCG_A>;
impl<'a, REG> DIVCG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fCG = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVCG_A::VALUE1)
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVCG_A::VALUE2)
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DIVCG_A::VALUE3)
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVCG_A::VALUE4)
    }
}
#[doc = "Field `RUN` reader - Run Indicator"]
pub type RUN_R = crate::BitReader<RUN_A>;
#[doc = "Run Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_A {
    #[doc = "0: Stopped (cleared at the end of a period)"]
    VALUE1 = 0,
    #[doc = "1: Running"]
    VALUE2 = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::VALUE1,
            true => RUN_A::VALUE2,
        }
    }
    #[doc = "Stopped (cleared at the end of a period)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RUN_A::VALUE1
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RUN_A::VALUE2
    }
}
#[doc = "Field `BITCOUNT` reader - Bit Counter"]
pub type BITCOUNT_R = crate::FieldReader;
#[doc = "Field `STEPCOUNT` reader - Step Counter"]
pub type STEPCOUNT_R = crate::FieldReader;
#[doc = "Field `STEPS` reader - Step Counter Sign"]
pub type STEPS_R = crate::BitReader<STEPS_A>;
#[doc = "Step Counter Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STEPS_A {
    #[doc = "0: Step counter value is positive"]
    VALUE1 = 0,
    #[doc = "1: Step counter value is negative"]
    VALUE2 = 1,
}
impl From<STEPS_A> for bool {
    #[inline(always)]
    fn from(variant: STEPS_A) -> Self {
        variant as u8 != 0
    }
}
impl STEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STEPS_A {
        match self.bits {
            false => STEPS_A::VALUE1,
            true => STEPS_A::VALUE2,
        }
    }
    #[doc = "Step counter value is positive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STEPS_A::VALUE1
    }
    #[doc = "Step counter value is negative"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STEPS_A::VALUE2
    }
}
#[doc = "Field `STEPD` reader - Step Counter Direction"]
pub type STEPD_R = crate::BitReader<STEPD_A>;
#[doc = "Step Counter Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STEPD_A {
    #[doc = "0: Step counter is counting up"]
    VALUE1 = 0,
    #[doc = "1: Step counter is counting down"]
    VALUE2 = 1,
}
impl From<STEPD_A> for bool {
    #[inline(always)]
    fn from(variant: STEPD_A) -> Self {
        variant as u8 != 0
    }
}
impl STEPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STEPD_A {
        match self.bits {
            false => STEPD_A::VALUE1,
            true => STEPD_A::VALUE2,
        }
    }
    #[doc = "Step counter is counting up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STEPD_A::VALUE1
    }
    #[doc = "Step counter is counting down"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STEPD_A::VALUE2
    }
}
#[doc = "Field `SGNCG` reader - Sign Signal from Carrier Generator"]
pub type SGNCG_R = crate::BitReader<SGNCG_A>;
#[doc = "Sign Signal from Carrier Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGNCG_A {
    #[doc = "0: Positive values"]
    VALUE1 = 0,
    #[doc = "1: Negative values"]
    VALUE2 = 1,
}
impl From<SGNCG_A> for bool {
    #[inline(always)]
    fn from(variant: SGNCG_A) -> Self {
        variant as u8 != 0
    }
}
impl SGNCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SGNCG_A {
        match self.bits {
            false => SGNCG_A::VALUE1,
            true => SGNCG_A::VALUE2,
        }
    }
    #[doc = "Positive values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SGNCG_A::VALUE1
    }
    #[doc = "Negative values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGNCG_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    pub fn cgmod(&self) -> CGMOD_R {
        CGMOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    pub fn brev(&self) -> BREV_R {
        BREV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    pub fn sigpol(&self) -> SIGPOL_R {
        SIGPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    pub fn divcg(&self) -> DIVCG_R {
        DIVCG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Run Indicator"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Bit Counter"]
    #[inline(always)]
    pub fn bitcount(&self) -> BITCOUNT_R {
        BITCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Step Counter"]
    #[inline(always)]
    pub fn stepcount(&self) -> STEPCOUNT_R {
        STEPCOUNT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Step Counter Sign"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Step Counter Direction"]
    #[inline(always)]
    pub fn stepd(&self) -> STEPD_R {
        STEPD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Sign Signal from Carrier Generator"]
    #[inline(always)]
    pub fn sgncg(&self) -> SGNCG_R {
        SGNCG_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cgmod(&mut self) -> CGMOD_W<CGCFG_SPEC> {
        CGMOD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    #[must_use]
    pub fn brev(&mut self) -> BREV_W<CGCFG_SPEC> {
        BREV_W::new(self, 2)
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn sigpol(&mut self) -> SIGPOL_W<CGCFG_SPEC> {
        SIGPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    #[must_use]
    pub fn divcg(&mut self) -> DIVCG_W<CGCFG_SPEC> {
        DIVCG_W::new(self, 4)
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
#[doc = "Carrier Generator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGCFG_SPEC;
impl crate::RegisterSpec for CGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgcfg::R`](R) reader structure"]
impl crate::Readable for CGCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cgcfg::W`](W) writer structure"]
impl crate::Writable for CGCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGCFG to value 0x0710_0000"]
impl crate::Resettable for CGCFG_SPEC {
    const RESET_VALUE: u32 = 0x0710_0000;
}
