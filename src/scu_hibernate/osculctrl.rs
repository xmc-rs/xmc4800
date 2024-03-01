#[doc = "Register `OSCULCTRL` reader"]
pub type R = crate::R<OsculctrlSpec>;
#[doc = "Register `OSCULCTRL` writer"]
pub type W = crate::W<OsculctrlSpec>;
#[doc = "XTAL1 Data General Purpose Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum X1den {
    #[doc = "0: Data input inactivated, power down"]
    Value1 = 0,
    #[doc = "1: Data input active"]
    Value2 = 1,
}
impl From<X1den> for bool {
    #[inline(always)]
    fn from(variant: X1den) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `X1DEN` reader - XTAL1 Data General Purpose Input Enable"]
pub type X1denR = crate::BitReader<X1den>;
impl X1denR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> X1den {
        match self.bits {
            false => X1den::Value1,
            true => X1den::Value2,
        }
    }
    #[doc = "Data input inactivated, power down"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == X1den::Value1
    }
    #[doc = "Data input active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == X1den::Value2
    }
}
#[doc = "Field `X1DEN` writer - XTAL1 Data General Purpose Input Enable"]
pub type X1denW<'a, REG> = crate::BitWriter<'a, REG, X1den>;
impl<'a, REG> X1denW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data input inactivated, power down"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(X1den::Value1)
    }
    #[doc = "Data input active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(X1den::Value2)
    }
}
#[doc = "Oscillator Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Oscillator is enabled, in operation"]
    Value1 = 0,
    #[doc = "1: Oscillator is enabled, in bypass mode"]
    Value2 = 1,
    #[doc = "2: Oscillator in power down"]
    Value3 = 2,
    #[doc = "3: Oscillator in power down, can be used as GPI"]
    Value4 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Value1,
            1 => Mode::Value2,
            2 => Mode::Value3,
            3 => Mode::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Oscillator is enabled, in operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mode::Value1
    }
    #[doc = "Oscillator is enabled, in bypass mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mode::Value2
    }
    #[doc = "Oscillator in power down"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mode::Value3
    }
    #[doc = "Oscillator in power down, can be used as GPI"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mode::Value4
    }
}
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type ModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Oscillator is enabled, in operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value1)
    }
    #[doc = "Oscillator is enabled, in bypass mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value2)
    }
    #[doc = "Oscillator in power down"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value3)
    }
    #[doc = "Oscillator in power down, can be used as GPI"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Value4)
    }
}
impl R {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    pub fn x1den(&self) -> X1denR {
        X1denR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - XTAL1 Data General Purpose Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn x1den(&mut self) -> X1denW<OsculctrlSpec> {
        X1denW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<OsculctrlSpec> {
        ModeW::new(self, 4)
    }
}
#[doc = "OSC_ULP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osculctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsculctrlSpec;
impl crate::RegisterSpec for OsculctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osculctrl::R`](R) reader structure"]
impl crate::Readable for OsculctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`osculctrl::W`](W) writer structure"]
impl crate::Writable for OsculctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSCULCTRL to value 0x20"]
impl crate::Resettable for OsculctrlSpec {
    const RESET_VALUE: u32 = 0x20;
}
