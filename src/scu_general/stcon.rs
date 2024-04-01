#[doc = "Register `STCON` reader"]
pub type R = crate::R<StconSpec>;
#[doc = "Register `STCON` writer"]
pub type W = crate::W<StconSpec>;
#[doc = "HW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hwcon {
    #[doc = "0: Normal mode, JTAG"]
    Value1 = 0,
    #[doc = "1: ASC BSL enabled"]
    Value2 = 1,
    #[doc = "2: BMI customized boot enabled"]
    Value3 = 2,
    #[doc = "3: CAN BSL enabled"]
    Value4 = 3,
}
impl From<Hwcon> for u8 {
    #[inline(always)]
    fn from(variant: Hwcon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hwcon {
    type Ux = u8;
}
impl crate::IsEnum for Hwcon {}
#[doc = "Field `HWCON` reader - HW Configuration"]
pub type HwconR = crate::FieldReader<Hwcon>;
impl HwconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwcon {
        match self.bits {
            0 => Hwcon::Value1,
            1 => Hwcon::Value2,
            2 => Hwcon::Value3,
            3 => Hwcon::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal mode, JTAG"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hwcon::Value1
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hwcon::Value2
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Hwcon::Value3
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Hwcon::Value4
    }
}
#[doc = "SW Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swcon {
    #[doc = "0: Normal mode, boot from Boot ROM"]
    Value1 = 0,
    #[doc = "1: ASC BSL enabled"]
    Value2 = 1,
    #[doc = "2: BMI customized boot enabled"]
    Value3 = 2,
    #[doc = "3: CAN BSL enabled"]
    Value4 = 3,
    #[doc = "4: Boot from Code SRAM"]
    Value5 = 4,
    #[doc = "8: Boot from alternate Flash Address 0"]
    Value6 = 8,
    #[doc = "12: Boot from alternate Flash Address 1"]
    Value7 = 12,
    #[doc = "14: Enable fallback Alternate Boot Mode (ABM)"]
    Value8 = 14,
}
impl From<Swcon> for u8 {
    #[inline(always)]
    fn from(variant: Swcon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swcon {
    type Ux = u8;
}
impl crate::IsEnum for Swcon {}
#[doc = "Field `SWCON` reader - SW Configuration"]
pub type SwconR = crate::FieldReader<Swcon>;
impl SwconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Swcon> {
        match self.bits {
            0 => Some(Swcon::Value1),
            1 => Some(Swcon::Value2),
            2 => Some(Swcon::Value3),
            3 => Some(Swcon::Value4),
            4 => Some(Swcon::Value5),
            8 => Some(Swcon::Value6),
            12 => Some(Swcon::Value7),
            14 => Some(Swcon::Value8),
            _ => None,
        }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Swcon::Value1
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Swcon::Value2
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Swcon::Value3
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Swcon::Value4
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Swcon::Value5
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Swcon::Value6
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Swcon::Value7
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Swcon::Value8
    }
}
#[doc = "Field `SWCON` writer - SW Configuration"]
pub type SwconW<'a, REG> = crate::FieldWriter<'a, REG, 4, Swcon>;
impl<'a, REG> SwconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value1)
    }
    #[doc = "ASC BSL enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value2)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value3)
    }
    #[doc = "CAN BSL enabled"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value4)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value5)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value6)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value7)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Swcon::Value8)
    }
}
impl R {
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline(always)]
    pub fn hwcon(&self) -> HwconR {
        HwconR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    pub fn swcon(&self) -> SwconR {
        SwconR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn swcon(&mut self) -> SwconW<StconSpec> {
        SwconW::new(self, 8)
    }
}
#[doc = "Startup Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StconSpec;
impl crate::RegisterSpec for StconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcon::R`](R) reader structure"]
impl crate::Readable for StconSpec {}
#[doc = "`write(|w| ..)` method takes [`stcon::W`](W) writer structure"]
impl crate::Writable for StconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCON to value 0"]
impl crate::Resettable for StconSpec {
    const RESET_VALUE: u32 = 0;
}
