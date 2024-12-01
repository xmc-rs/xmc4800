#[doc = "Register `CLC` reader"]
pub type R = crate::R<CLC_SPEC>;
#[doc = "Register `CLC` writer"]
pub type W = crate::W<CLC_SPEC>;
#[doc = "EBU Disable Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISR_A {
    #[doc = "0: EBU disable is not requested"]
    VALUE1 = 0,
    #[doc = "1: EBU disable is requested"]
    VALUE2 = 1,
}
impl From<DISR_A> for bool {
    #[inline(always)]
    fn from(variant: DISR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISR` reader - EBU Disable Request Bit"]
pub type DISR_R = crate::BitReader<DISR_A>;
impl DISR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISR_A {
        match self.bits {
            false => DISR_A::VALUE1,
            true => DISR_A::VALUE2,
        }
    }
    #[doc = "EBU disable is not requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISR_A::VALUE1
    }
    #[doc = "EBU disable is requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISR_A::VALUE2
    }
}
#[doc = "Field `DISR` writer - EBU Disable Request Bit"]
pub type DISR_W<'a, REG> = crate::BitWriter<'a, REG, DISR_A>;
impl<'a, REG> DISR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EBU disable is not requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DISR_A::VALUE1)
    }
    #[doc = "EBU disable is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DISR_A::VALUE2)
    }
}
#[doc = "EBU Disable Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISS_A {
    #[doc = "0: EBU is enabled (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: EBU is disabled"]
    VALUE2 = 1,
}
impl From<DISS_A> for bool {
    #[inline(always)]
    fn from(variant: DISS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISS` reader - EBU Disable Status Bit"]
pub type DISS_R = crate::BitReader<DISS_A>;
impl DISS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISS_A {
        match self.bits {
            false => DISS_A::VALUE1,
            true => DISS_A::VALUE2,
        }
    }
    #[doc = "EBU is enabled (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DISS_A::VALUE1
    }
    #[doc = "EBU is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DISS_A::VALUE2
    }
}
#[doc = "EBU Clocking Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    VALUE1 = 0,
    #[doc = "1: request EBU to run synchronously to ARM processor (default after reset)"]
    VALUE2 = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - EBU Clocking Mode"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::VALUE1,
            true => SYNC_A::VALUE2,
        }
    }
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNC_A::VALUE1
    }
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNC_A::VALUE2
    }
}
#[doc = "Field `SYNC` writer - EBU Clocking Mode"]
pub type SYNC_W<'a, REG> = crate::BitWriter<'a, REG, SYNC_A>;
impl<'a, REG> SYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::VALUE1)
    }
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_A::VALUE2)
    }
}
#[doc = "DIV2 Clocking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV2_A {
    #[doc = "0: standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: request EBU to run off AHB bus clock divided by 2."]
    VALUE2 = 1,
}
impl From<DIV2_A> for bool {
    #[inline(always)]
    fn from(variant: DIV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV2` reader - DIV2 Clocking Mode"]
pub type DIV2_R = crate::BitReader<DIV2_A>;
impl DIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIV2_A {
        match self.bits {
            false => DIV2_A::VALUE1,
            true => DIV2_A::VALUE2,
        }
    }
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV2_A::VALUE1
    }
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV2_A::VALUE2
    }
}
#[doc = "Field `DIV2` writer - DIV2 Clocking Mode"]
pub type DIV2_W<'a, REG> = crate::BitWriter<'a, REG, DIV2_A>;
impl<'a, REG> DIV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIV2_A::VALUE1)
    }
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIV2_A::VALUE2)
    }
}
#[doc = "EBU Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EBUDIV_A {
    #[doc = "0: request EBU to run off input clock (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: request EBU to run off input clock divided by 2"]
    VALUE2 = 1,
    #[doc = "2: request EBU to run off input clock divided by 3"]
    VALUE3 = 2,
    #[doc = "3: request EBU to run off input clock divided by 4"]
    VALUE4 = 3,
}
impl From<EBUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: EBUDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EBUDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for EBUDIV_A {}
#[doc = "Field `EBUDIV` reader - EBU Clock Divide Ratio"]
pub type EBUDIV_R = crate::FieldReader<EBUDIV_A>;
impl EBUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EBUDIV_A {
        match self.bits {
            0 => EBUDIV_A::VALUE1,
            1 => EBUDIV_A::VALUE2,
            2 => EBUDIV_A::VALUE3,
            3 => EBUDIV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "request EBU to run off input clock (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBUDIV_A::VALUE1
    }
    #[doc = "request EBU to run off input clock divided by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBUDIV_A::VALUE2
    }
    #[doc = "request EBU to run off input clock divided by 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EBUDIV_A::VALUE3
    }
    #[doc = "request EBU to run off input clock divided by 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EBUDIV_A::VALUE4
    }
}
#[doc = "Field `EBUDIV` writer - EBU Clock Divide Ratio"]
pub type EBUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EBUDIV_A, crate::Safe>;
impl<'a, REG> EBUDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "request EBU to run off input clock (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EBUDIV_A::VALUE1)
    }
    #[doc = "request EBU to run off input clock divided by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EBUDIV_A::VALUE2)
    }
    #[doc = "request EBU to run off input clock divided by 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EBUDIV_A::VALUE3)
    }
    #[doc = "request EBU to run off input clock divided by 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EBUDIV_A::VALUE4)
    }
}
#[doc = "EBU Clocking Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCACK_A {
    #[doc = "0: the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    VALUE1 = 0,
    #[doc = "1: EBU is synchronous to the AHB bus clock (default after reset)"]
    VALUE2 = 1,
}
impl From<SYNCACK_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCACK` reader - EBU Clocking Mode Status"]
pub type SYNCACK_R = crate::BitReader<SYNCACK_A>;
impl SYNCACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYNCACK_A {
        match self.bits {
            false => SYNCACK_A::VALUE1,
            true => SYNCACK_A::VALUE2,
        }
    }
    #[doc = "the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYNCACK_A::VALUE1
    }
    #[doc = "EBU is synchronous to the AHB bus clock (default after reset)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYNCACK_A::VALUE2
    }
}
#[doc = "DIV2 Clocking Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV2ACK_A {
    #[doc = "0: EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: EBU is running off AHB bus clock divided by 2."]
    VALUE2 = 1,
}
impl From<DIV2ACK_A> for bool {
    #[inline(always)]
    fn from(variant: DIV2ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV2ACK` reader - DIV2 Clocking Mode Status"]
pub type DIV2ACK_R = crate::BitReader<DIV2ACK_A>;
impl DIV2ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIV2ACK_A {
        match self.bits {
            false => DIV2ACK_A::VALUE1,
            true => DIV2ACK_A::VALUE2,
        }
    }
    #[doc = "EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIV2ACK_A::VALUE1
    }
    #[doc = "EBU is running off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIV2ACK_A::VALUE2
    }
}
#[doc = "EBU Clock Divide Ratio Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EBUDIVACK_A {
    #[doc = "0: EBU is running off input clock (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: EBU is running off input clock divided by 2"]
    VALUE2 = 1,
    #[doc = "2: EBU is running off input clock divided by 3"]
    VALUE3 = 2,
    #[doc = "3: EBU is running off input clock divided by 4"]
    VALUE4 = 3,
}
impl From<EBUDIVACK_A> for u8 {
    #[inline(always)]
    fn from(variant: EBUDIVACK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EBUDIVACK_A {
    type Ux = u8;
}
impl crate::IsEnum for EBUDIVACK_A {}
#[doc = "Field `EBUDIVACK` reader - EBU Clock Divide Ratio Status"]
pub type EBUDIVACK_R = crate::FieldReader<EBUDIVACK_A>;
impl EBUDIVACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EBUDIVACK_A {
        match self.bits {
            0 => EBUDIVACK_A::VALUE1,
            1 => EBUDIVACK_A::VALUE2,
            2 => EBUDIVACK_A::VALUE3,
            3 => EBUDIVACK_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "EBU is running off input clock (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBUDIVACK_A::VALUE1
    }
    #[doc = "EBU is running off input clock divided by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBUDIVACK_A::VALUE2
    }
    #[doc = "EBU is running off input clock divided by 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EBUDIVACK_A::VALUE3
    }
    #[doc = "EBU is running off input clock divided by 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EBUDIVACK_A::VALUE4
    }
}
impl R {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DISR_R {
        DISR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBU Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DISS_R {
        DISS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    pub fn div2(&self) -> DIV2_R {
        DIV2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - EBU Clocking Mode Status"]
    #[inline(always)]
    pub fn syncack(&self) -> SYNCACK_R {
        SYNCACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIV2 Clocking Mode Status"]
    #[inline(always)]
    pub fn div2ack(&self) -> DIV2ACK_R {
        DIV2ACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - EBU Clock Divide Ratio Status"]
    #[inline(always)]
    pub fn ebudivack(&self) -> EBUDIVACK_R {
        EBUDIVACK_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&mut self) -> DISR_W<CLC_SPEC> {
        DISR_W::new(self, 0)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<CLC_SPEC> {
        SYNC_W::new(self, 16)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    pub fn div2(&mut self) -> DIV2_W<CLC_SPEC> {
        DIV2_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    pub fn ebudiv(&mut self) -> EBUDIV_W<CLC_SPEC> {
        EBUDIV_W::new(self, 18)
    }
}
#[doc = "EBU Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLC_SPEC;
impl crate::RegisterSpec for CLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clc::R`](R) reader structure"]
impl crate::Readable for CLC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clc::W`](W) writer structure"]
impl crate::Writable for CLC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLC to value 0x0011_0000"]
impl crate::Resettable for CLC_SPEC {
    const RESET_VALUE: u32 = 0x0011_0000;
}
