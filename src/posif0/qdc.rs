#[doc = "Reader of register QDC"]
pub type R = crate::R<u32, super::QDC>;
#[doc = "Writer for register QDC"]
pub type W = crate::W<u32, super::QDC>;
#[doc = "Register QDC `reset()`'s with value 0"]
impl crate::ResetValue for super::QDC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Phase A Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PALS_A {
    #[doc = "0: Phase A is active HIGH"]
    VALUE1,
    #[doc = "1: Phase A is active LOW"]
    VALUE2,
}
impl From<PALS_A> for bool {
    #[inline(always)]
    fn from(variant: PALS_A) -> Self {
        match variant {
            PALS_A::VALUE1 => false,
            PALS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PALS`"]
pub type PALS_R = crate::R<bool, PALS_A>;
impl PALS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PALS_A {
        match self.bits {
            false => PALS_A::VALUE1,
            true => PALS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PALS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PALS_A::VALUE2
    }
}
#[doc = "Write proxy for field `PALS`"]
pub struct PALS_W<'a> {
    w: &'a mut W,
}
impl<'a> PALS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase A is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PALS_A::VALUE1)
    }
    #[doc = "Phase A is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PALS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Phase B Level selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBLS_A {
    #[doc = "0: Phase B is active HIGH"]
    VALUE1,
    #[doc = "1: Phase B is active LOW"]
    VALUE2,
}
impl From<PBLS_A> for bool {
    #[inline(always)]
    fn from(variant: PBLS_A) -> Self {
        match variant {
            PBLS_A::VALUE1 => false,
            PBLS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PBLS`"]
pub type PBLS_R = crate::R<bool, PBLS_A>;
impl PBLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBLS_A {
        match self.bits {
            false => PBLS_A::VALUE1,
            true => PBLS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBLS_A::VALUE2
    }
}
#[doc = "Write proxy for field `PBLS`"]
pub struct PBLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBLS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase B is active HIGH"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PBLS_A::VALUE1)
    }
    #[doc = "Phase B is active LOW"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PBLS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Phase signals swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHS_A {
    #[doc = "0: Phase A is the leading signal for clockwise rotation"]
    VALUE1,
    #[doc = "1: Phase B is the leading signal for clockwise rotation"]
    VALUE2,
}
impl From<PHS_A> for bool {
    #[inline(always)]
    fn from(variant: PHS_A) -> Self {
        match variant {
            PHS_A::VALUE1 => false,
            PHS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PHS`"]
pub type PHS_R = crate::R<bool, PHS_A>;
impl PHS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHS_A {
        match self.bits {
            false => PHS_A::VALUE1,
            true => PHS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PHS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHS_A::VALUE2
    }
}
#[doc = "Write proxy for field `PHS`"]
pub struct PHS_W<'a> {
    w: &'a mut W,
}
impl<'a> PHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase A is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PHS_A::VALUE1)
    }
    #[doc = "Phase B is the leading signal for clockwise rotation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PHS_A::VALUE2)
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
#[doc = "Index Marker generations control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICM_A {
    #[doc = "0: No index marker generation on POSIFx.OUT3"]
    VALUE1,
    #[doc = "1: Only first index occurrence generated on POSIFx.OUT3"]
    VALUE2,
    #[doc = "2: All index occurrences generated on POSIFx.OUT3"]
    VALUE3,
}
impl From<ICM_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM_A) -> Self {
        match variant {
            ICM_A::VALUE1 => 0,
            ICM_A::VALUE2 => 1,
            ICM_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `ICM`"]
pub type ICM_R = crate::R<u8, ICM_A>;
impl ICM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ICM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ICM_A::VALUE1),
            1 => Val(ICM_A::VALUE2),
            2 => Val(ICM_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ICM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ICM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ICM_A::VALUE3
    }
}
#[doc = "Write proxy for field `ICM`"]
pub struct ICM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No index marker generation on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ICM_A::VALUE1)
    }
    #[doc = "Only first index occurrence generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ICM_A::VALUE2)
    }
    #[doc = "All index occurrences generated on POSIFx.OUT3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ICM_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Current rotation direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVAL_A {
    #[doc = "0: Counterclockwise rotation"]
    VALUE1,
    #[doc = "1: Clockwise rotation"]
    VALUE2,
}
impl From<DVAL_A> for bool {
    #[inline(always)]
    fn from(variant: DVAL_A) -> Self {
        match variant {
            DVAL_A::VALUE1 => false,
            DVAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DVAL`"]
pub type DVAL_R = crate::R<bool, DVAL_A>;
impl DVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DVAL_A {
        match self.bits {
            false => DVAL_A::VALUE1,
            true => DVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DVAL_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    pub fn pals(&self) -> PALS_R {
        PALS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    pub fn pbls(&self) -> PBLS_R {
        PBLS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    pub fn phs(&self) -> PHS_R {
        PHS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    pub fn icm(&self) -> ICM_R {
        ICM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Current rotation direction"]
    #[inline(always)]
    pub fn dval(&self) -> DVAL_R {
        DVAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Phase A Level selector"]
    #[inline(always)]
    pub fn pals(&mut self) -> PALS_W {
        PALS_W { w: self }
    }
    #[doc = "Bit 1 - Phase B Level selector"]
    #[inline(always)]
    pub fn pbls(&mut self) -> PBLS_W {
        PBLS_W { w: self }
    }
    #[doc = "Bit 2 - Phase signals swap"]
    #[inline(always)]
    pub fn phs(&mut self) -> PHS_W {
        PHS_W { w: self }
    }
    #[doc = "Bits 4:5 - Index Marker generations control"]
    #[inline(always)]
    pub fn icm(&mut self) -> ICM_W {
        ICM_W { w: self }
    }
}
