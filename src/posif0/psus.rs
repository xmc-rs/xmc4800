#[doc = "Register `PSUS` reader"]
pub type R = crate::R<PsusSpec>;
#[doc = "Register `PSUS` writer"]
pub type W = crate::W<PsusSpec>;
#[doc = "Quadrature Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qsus {
    #[doc = "0: Suspend request ignored"]
    Value1 = 0,
    #[doc = "1: Stop immediately"]
    Value2 = 1,
    #[doc = "2: Suspend in the next index occurrence"]
    Value3 = 2,
    #[doc = "3: Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    Value4 = 3,
}
impl From<Qsus> for u8 {
    #[inline(always)]
    fn from(variant: Qsus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qsus {
    type Ux = u8;
}
#[doc = "Field `QSUS` reader - Quadrature Mode Suspend Config"]
pub type QsusR = crate::FieldReader<Qsus>;
impl QsusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qsus {
        match self.bits {
            0 => Qsus::Value1,
            1 => Qsus::Value2,
            2 => Qsus::Value3,
            3 => Qsus::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Qsus::Value1
    }
    #[doc = "Stop immediately"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Qsus::Value2
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Qsus::Value3
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Qsus::Value4
    }
}
#[doc = "Field `QSUS` writer - Quadrature Mode Suspend Config"]
pub type QsusW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Qsus>;
impl<'a, REG> QsusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Qsus::Value1)
    }
    #[doc = "Stop immediately"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Qsus::Value2)
    }
    #[doc = "Suspend in the next index occurrence"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Qsus::Value3)
    }
    #[doc = "Suspend in the next phase (PhaseA or PhaseB) occurrence"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Qsus::Value4)
    }
}
#[doc = "Multi-Channel Mode Suspend Config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msus {
    #[doc = "0: Suspend request ignored"]
    Value1 = 0,
    #[doc = "1: Stop immediately. Multi-Channel pattern is not set to the reset value."]
    Value2 = 1,
    #[doc = "2: Stop immediately. Multi-Channel pattern is set to the reset value."]
    Value3 = 2,
    #[doc = "3: Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    Value4 = 3,
}
impl From<Msus> for u8 {
    #[inline(always)]
    fn from(variant: Msus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msus {
    type Ux = u8;
}
#[doc = "Field `MSUS` reader - Multi-Channel Mode Suspend Config"]
pub type MsusR = crate::FieldReader<Msus>;
impl MsusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msus {
        match self.bits {
            0 => Msus::Value1,
            1 => Msus::Value2,
            2 => Msus::Value3,
            3 => Msus::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Msus::Value1
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Msus::Value2
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Msus::Value3
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Msus::Value4
    }
}
#[doc = "Field `MSUS` writer - Multi-Channel Mode Suspend Config"]
pub type MsusW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Msus>;
impl<'a, REG> MsusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Suspend request ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Msus::Value1)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is not set to the reset value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Msus::Value2)
    }
    #[doc = "Stop immediately. Multi-Channel pattern is set to the reset value."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Msus::Value3)
    }
    #[doc = "Suspend with the synchronization of the PWM signal. Multi-Channel pattern is set to the reset value at the same time of the synchronization."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Msus::Value4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    pub fn qsus(&self) -> QsusR {
        QsusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    pub fn msus(&self) -> MsusR {
        MsusR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Quadrature Mode Suspend Config"]
    #[inline(always)]
    #[must_use]
    pub fn qsus(&mut self) -> QsusW<PsusSpec> {
        QsusW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Multi-Channel Mode Suspend Config"]
    #[inline(always)]
    #[must_use]
    pub fn msus(&mut self) -> MsusW<PsusSpec> {
        MsusW::new(self, 2)
    }
}
#[doc = "POSIF Suspend Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsusSpec;
impl crate::RegisterSpec for PsusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psus::R`](R) reader structure"]
impl crate::Readable for PsusSpec {}
#[doc = "`write(|w| ..)` method takes [`psus::W`](W) writer structure"]
impl crate::Writable for PsusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSUS to value 0"]
impl crate::Resettable for PsusSpec {
    const RESET_VALUE: u32 = 0;
}
