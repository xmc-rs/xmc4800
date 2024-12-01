#[doc = "Register `RECTCFG` reader"]
pub type R = crate::R<RECTCFG_SPEC>;
#[doc = "Register `RECTCFG` writer"]
pub type W = crate::W<RECTCFG_SPEC>;
#[doc = "Rectification Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEN_A {
    #[doc = "0: No rectification, data not altered"]
    VALUE1 = 0,
    #[doc = "1: Data are rectified according to SGND"]
    VALUE2 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEN` reader - Rectification Enable"]
pub type RFEN_R = crate::BitReader<RFEN_A>;
impl RFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::VALUE1,
            true => RFEN_A::VALUE2,
        }
    }
    #[doc = "No rectification, data not altered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RFEN_A::VALUE1
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RFEN_A::VALUE2
    }
}
#[doc = "Field `RFEN` writer - Rectification Enable"]
pub type RFEN_W<'a, REG> = crate::BitWriter<'a, REG, RFEN_A>;
impl<'a, REG> RFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No rectification, data not altered"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RFEN_A::VALUE1)
    }
    #[doc = "Data are rectified according to SGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RFEN_A::VALUE2)
    }
}
#[doc = "Sign Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSRC_A {
    #[doc = "0: On-chip carrier generator"]
    VALUE1 = 0,
    #[doc = "1: Sign of result of next channel"]
    VALUE2 = 1,
    #[doc = "2: External sign signal A"]
    VALUE3 = 2,
    #[doc = "3: External sign signal B"]
    VALUE4 = 3,
}
impl From<SSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SSRC_A {
    type Ux = u8;
}
impl crate::IsEnum for SSRC_A {}
#[doc = "Field `SSRC` reader - Sign Source"]
pub type SSRC_R = crate::FieldReader<SSRC_A>;
impl SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSRC_A {
        match self.bits {
            0 => SSRC_A::VALUE1,
            1 => SSRC_A::VALUE2,
            2 => SSRC_A::VALUE3,
            3 => SSRC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "On-chip carrier generator"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SSRC_A::VALUE1
    }
    #[doc = "Sign of result of next channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSRC_A::VALUE2
    }
    #[doc = "External sign signal A"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSRC_A::VALUE3
    }
    #[doc = "External sign signal B"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SSRC_A::VALUE4
    }
}
#[doc = "Field `SSRC` writer - Sign Source"]
pub type SSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SSRC_A, crate::Safe>;
impl<'a, REG> SSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "On-chip carrier generator"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SSRC_A::VALUE1)
    }
    #[doc = "Sign of result of next channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SSRC_A::VALUE2)
    }
    #[doc = "External sign signal A"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SSRC_A::VALUE3)
    }
    #[doc = "External sign signal B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SSRC_A::VALUE4)
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDVAL_A {
    #[doc = "0: No new result available"]
    VALUE1 = 0,
    #[doc = "1: Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    VALUE2 = 1,
}
impl From<SDVAL_A> for bool {
    #[inline(always)]
    fn from(variant: SDVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDVAL` reader - Valid Flag"]
pub type SDVAL_R = crate::BitReader<SDVAL_A>;
impl SDVAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDVAL_A {
        match self.bits {
            false => SDVAL_A::VALUE1,
            true => SDVAL_A::VALUE2,
        }
    }
    #[doc = "No new result available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDVAL_A::VALUE1
    }
    #[doc = "Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDVAL_A::VALUE2
    }
}
#[doc = "Selected Carrier Sign Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGNCS_A {
    #[doc = "0: Positive values"]
    VALUE1 = 0,
    #[doc = "1: Negative values"]
    VALUE2 = 1,
}
impl From<SGNCS_A> for bool {
    #[inline(always)]
    fn from(variant: SGNCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGNCS` reader - Selected Carrier Sign Signal"]
pub type SGNCS_R = crate::BitReader<SGNCS_A>;
impl SGNCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SGNCS_A {
        match self.bits {
            false => SGNCS_A::VALUE1,
            true => SGNCS_A::VALUE2,
        }
    }
    #[doc = "Positive values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SGNCS_A::VALUE1
    }
    #[doc = "Negative values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGNCS_A::VALUE2
    }
}
#[doc = "Sign Signal Delayed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGND_A {
    #[doc = "0: Positive values"]
    VALUE1 = 0,
    #[doc = "1: Negative values"]
    VALUE2 = 1,
}
impl From<SGND_A> for bool {
    #[inline(always)]
    fn from(variant: SGND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGND` reader - Sign Signal Delayed"]
pub type SGND_R = crate::BitReader<SGND_A>;
impl SGND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SGND_A {
        match self.bits {
            false => SGND_A::VALUE1,
            true => SGND_A::VALUE2,
        }
    }
    #[doc = "Positive values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SGND_A::VALUE1
    }
    #[doc = "Negative values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGND_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&self) -> SSRC_R {
        SSRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 15 - Valid Flag"]
    #[inline(always)]
    pub fn sdval(&self) -> SDVAL_R {
        SDVAL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Selected Carrier Sign Signal"]
    #[inline(always)]
    pub fn sgncs(&self) -> SGNCS_R {
        SGNCS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sign Signal Delayed"]
    #[inline(always)]
    pub fn sgnd(&self) -> SGND_R {
        SGND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rectification Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W<RECTCFG_SPEC> {
        RFEN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Sign Source"]
    #[inline(always)]
    pub fn ssrc(&mut self) -> SSRC_W<RECTCFG_SPEC> {
        SSRC_W::new(self, 4)
    }
}
#[doc = "Rectification Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rectcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rectcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECTCFG_SPEC;
impl crate::RegisterSpec for RECTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rectcfg::R`](R) reader structure"]
impl crate::Readable for RECTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rectcfg::W`](W) writer structure"]
impl crate::Writable for RECTCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RECTCFG to value 0x8000_0000"]
impl crate::Resettable for RECTCFG_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
