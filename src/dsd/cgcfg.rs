#[doc = "Reader of register CGCFG"]
pub type R = crate::R<u32, super::CGCFG>;
#[doc = "Writer for register CGCFG"]
pub type W = crate::W<u32, super::CGCFG>;
#[doc = "Register CGCFG `reset()`'s with value 0x0710_0000"]
impl crate::ResetValue for super::CGCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0710_0000
    }
}
#[doc = "Carrier Generator Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CGMOD`"]
pub type CGMOD_R = crate::R<u8, CGMOD_A>;
impl CGMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGMOD_A {
        match self.bits {
            0 => CGMOD_A::VALUE1,
            1 => CGMOD_A::VALUE2,
            2 => CGMOD_A::VALUE3,
            3 => CGMOD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CGMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CGMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CGMOD_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CGMOD_A::VALUE4
    }
}
#[doc = "Write proxy for field `CGMOD`"]
pub struct CGMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CGMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stopped"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE1)
    }
    #[doc = "Square wave"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE2)
    }
    #[doc = "Triangle"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE3)
    }
    #[doc = "Sine wave"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CGMOD_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Bit-Reverse PWM Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `BREV`"]
pub type BREV_R = crate::R<bool, BREV_A>;
impl BREV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREV_A {
        match self.bits {
            false => BREV_A::VALUE1,
            true => BREV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BREV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BREV_A::VALUE2
    }
}
#[doc = "Write proxy for field `BREV`"]
pub struct BREV_W<'a> {
    w: &'a mut W,
}
impl<'a> BREV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BREV_A::VALUE1)
    }
    #[doc = "Bit-reverse mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BREV_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Signal Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SIGPOL`"]
pub type SIGPOL_R = crate::R<bool, SIGPOL_A>;
impl SIGPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGPOL_A {
        match self.bits {
            false => SIGPOL_A::VALUE1,
            true => SIGPOL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIGPOL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIGPOL_A::VALUE2
    }
}
#[doc = "Write proxy for field `SIGPOL`"]
pub struct SIGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal: carrier signal begins with +1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIGPOL_A::VALUE1)
    }
    #[doc = "Inverted: carrier signal begins with -1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIGPOL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Divider Factor for the PWM Pattern Signal Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DIVCG`"]
pub type DIVCG_R = crate::R<u8, DIVCG_A>;
impl DIVCG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVCG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVCG_A::VALUE1),
            1 => Val(DIVCG_A::VALUE2),
            2 => Val(DIVCG_A::VALUE3),
            15 => Val(DIVCG_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIVCG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIVCG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DIVCG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DIVCG_A::VALUE4
    }
}
#[doc = "Write proxy for field `DIVCG`"]
pub struct DIVCG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVCG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVCG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "fCG = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE1)
    }
    #[doc = "fCG = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE2)
    }
    #[doc = "fCG = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE3)
    }
    #[doc = "fCG = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DIVCG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Run Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, RUN_A>;
impl RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::VALUE1,
            true => RUN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RUN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RUN_A::VALUE2
    }
}
#[doc = "Reader of field `BITCOUNT`"]
pub type BITCOUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `STEPCOUNT`"]
pub type STEPCOUNT_R = crate::R<u8, u8>;
#[doc = "Step Counter Sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `STEPS`"]
pub type STEPS_R = crate::R<bool, STEPS_A>;
impl STEPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STEPS_A {
        match self.bits {
            false => STEPS_A::VALUE1,
            true => STEPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STEPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STEPS_A::VALUE2
    }
}
#[doc = "Step Counter Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `STEPD`"]
pub type STEPD_R = crate::R<bool, STEPD_A>;
impl STEPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STEPD_A {
        match self.bits {
            false => STEPD_A::VALUE1,
            true => STEPD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STEPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STEPD_A::VALUE2
    }
}
#[doc = "Sign Signal from Carrier Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SGNCG`"]
pub type SGNCG_R = crate::R<bool, SGNCG_A>;
impl SGNCG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGNCG_A {
        match self.bits {
            false => SGNCG_A::VALUE1,
            true => SGNCG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SGNCG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SGNCG_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    pub fn cgmod(&self) -> CGMOD_R {
        CGMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    pub fn brev(&self) -> BREV_R {
        BREV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    pub fn sigpol(&self) -> SIGPOL_R {
        SIGPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    pub fn divcg(&self) -> DIVCG_R {
        DIVCG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Run Indicator"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 15) & 0x01) != 0)
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
        STEPS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Step Counter Direction"]
    #[inline(always)]
    pub fn stepd(&self) -> STEPD_R {
        STEPD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Sign Signal from Carrier Generator"]
    #[inline(always)]
    pub fn sgncg(&self) -> SGNCG_R {
        SGNCG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Carrier Generator Operating Mode"]
    #[inline(always)]
    pub fn cgmod(&mut self) -> CGMOD_W {
        CGMOD_W { w: self }
    }
    #[doc = "Bit 2 - Bit-Reverse PWM Generation"]
    #[inline(always)]
    pub fn brev(&mut self) -> BREV_W {
        BREV_W { w: self }
    }
    #[doc = "Bit 3 - Signal Polarity"]
    #[inline(always)]
    pub fn sigpol(&mut self) -> SIGPOL_W {
        SIGPOL_W { w: self }
    }
    #[doc = "Bits 4:7 - Divider Factor for the PWM Pattern Signal Generator"]
    #[inline(always)]
    pub fn divcg(&mut self) -> DIVCG_W {
        DIVCG_W { w: self }
    }
}
