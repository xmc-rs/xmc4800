#[doc = "Reader of register IOCR0"]
pub type R = crate::R<u32, super::IOCR0>;
#[doc = "Writer for register IOCR0"]
pub type W = crate::W<u32, super::IOCR0>;
#[doc = "Register IOCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC0_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl From<PC0_A> for u8 {
    #[inline(always)]
    fn from(variant: PC0_A) -> Self {
        match variant {
            PC0_A::VALUE1 => 0,
            PC0_A::VALUE2 => 1,
            PC0_A::VALUE3 => 2,
            PC0_A::VALUE4 => 3,
            PC0_A::VALUE5 => 4,
            PC0_A::VALUE6 => 5,
            PC0_A::VALUE7 => 6,
            PC0_A::VALUE8 => 7,
            PC0_A::VALUE9 => 16,
            PC0_A::VALUE10 => 17,
            PC0_A::VALUE11 => 18,
            PC0_A::VALUE12 => 19,
            PC0_A::VALUE13 => 20,
            PC0_A::VALUE14 => 24,
            PC0_A::VALUE15 => 25,
            PC0_A::VALUE16 => 26,
            PC0_A::VALUE17 => 27,
            PC0_A::VALUE18 => 28,
        }
    }
}
#[doc = "Reader of field `PC0`"]
pub type PC0_R = crate::R<u8, PC0_A>;
impl PC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC0_A::VALUE1),
            1 => Val(PC0_A::VALUE2),
            2 => Val(PC0_A::VALUE3),
            3 => Val(PC0_A::VALUE4),
            4 => Val(PC0_A::VALUE5),
            5 => Val(PC0_A::VALUE6),
            6 => Val(PC0_A::VALUE7),
            7 => Val(PC0_A::VALUE8),
            16 => Val(PC0_A::VALUE9),
            17 => Val(PC0_A::VALUE10),
            18 => Val(PC0_A::VALUE11),
            19 => Val(PC0_A::VALUE12),
            20 => Val(PC0_A::VALUE13),
            24 => Val(PC0_A::VALUE14),
            25 => Val(PC0_A::VALUE15),
            26 => Val(PC0_A::VALUE16),
            27 => Val(PC0_A::VALUE17),
            28 => Val(PC0_A::VALUE18),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC0_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC0_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC0_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC0_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC0_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC0_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC0_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC0_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC0_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC0_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC0_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC0_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC0_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC0_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC0_A::VALUE18
    }
}
#[doc = "Write proxy for field `PC0`"]
pub struct PC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC0_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC0_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC0_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC0_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC0_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC0_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC0_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC0_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC0_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC0_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC0_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC0_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC0_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC0_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC0_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC0_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC0_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC0_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC1_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl From<PC1_A> for u8 {
    #[inline(always)]
    fn from(variant: PC1_A) -> Self {
        match variant {
            PC1_A::VALUE1 => 0,
            PC1_A::VALUE2 => 1,
            PC1_A::VALUE3 => 2,
            PC1_A::VALUE4 => 3,
            PC1_A::VALUE5 => 4,
            PC1_A::VALUE6 => 5,
            PC1_A::VALUE7 => 6,
            PC1_A::VALUE8 => 7,
            PC1_A::VALUE9 => 16,
            PC1_A::VALUE10 => 17,
            PC1_A::VALUE11 => 18,
            PC1_A::VALUE12 => 19,
            PC1_A::VALUE13 => 20,
            PC1_A::VALUE14 => 24,
            PC1_A::VALUE15 => 25,
            PC1_A::VALUE16 => 26,
            PC1_A::VALUE17 => 27,
            PC1_A::VALUE18 => 28,
        }
    }
}
#[doc = "Reader of field `PC1`"]
pub type PC1_R = crate::R<u8, PC1_A>;
impl PC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC1_A::VALUE1),
            1 => Val(PC1_A::VALUE2),
            2 => Val(PC1_A::VALUE3),
            3 => Val(PC1_A::VALUE4),
            4 => Val(PC1_A::VALUE5),
            5 => Val(PC1_A::VALUE6),
            6 => Val(PC1_A::VALUE7),
            7 => Val(PC1_A::VALUE8),
            16 => Val(PC1_A::VALUE9),
            17 => Val(PC1_A::VALUE10),
            18 => Val(PC1_A::VALUE11),
            19 => Val(PC1_A::VALUE12),
            20 => Val(PC1_A::VALUE13),
            24 => Val(PC1_A::VALUE14),
            25 => Val(PC1_A::VALUE15),
            26 => Val(PC1_A::VALUE16),
            27 => Val(PC1_A::VALUE17),
            28 => Val(PC1_A::VALUE18),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC1_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC1_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC1_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC1_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC1_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC1_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC1_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC1_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC1_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC1_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC1_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC1_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC1_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC1_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC1_A::VALUE18
    }
}
#[doc = "Write proxy for field `PC1`"]
pub struct PC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC1_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC1_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC1_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC1_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC1_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC1_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC1_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC1_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC1_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC1_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC1_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC1_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC1_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC1_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC1_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC1_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC1_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC1_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC2_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl From<PC2_A> for u8 {
    #[inline(always)]
    fn from(variant: PC2_A) -> Self {
        match variant {
            PC2_A::VALUE1 => 0,
            PC2_A::VALUE2 => 1,
            PC2_A::VALUE3 => 2,
            PC2_A::VALUE4 => 3,
            PC2_A::VALUE5 => 4,
            PC2_A::VALUE6 => 5,
            PC2_A::VALUE7 => 6,
            PC2_A::VALUE8 => 7,
            PC2_A::VALUE9 => 16,
            PC2_A::VALUE10 => 17,
            PC2_A::VALUE11 => 18,
            PC2_A::VALUE12 => 19,
            PC2_A::VALUE13 => 20,
            PC2_A::VALUE14 => 24,
            PC2_A::VALUE15 => 25,
            PC2_A::VALUE16 => 26,
            PC2_A::VALUE17 => 27,
            PC2_A::VALUE18 => 28,
        }
    }
}
#[doc = "Reader of field `PC2`"]
pub type PC2_R = crate::R<u8, PC2_A>;
impl PC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC2_A::VALUE1),
            1 => Val(PC2_A::VALUE2),
            2 => Val(PC2_A::VALUE3),
            3 => Val(PC2_A::VALUE4),
            4 => Val(PC2_A::VALUE5),
            5 => Val(PC2_A::VALUE6),
            6 => Val(PC2_A::VALUE7),
            7 => Val(PC2_A::VALUE8),
            16 => Val(PC2_A::VALUE9),
            17 => Val(PC2_A::VALUE10),
            18 => Val(PC2_A::VALUE11),
            19 => Val(PC2_A::VALUE12),
            20 => Val(PC2_A::VALUE13),
            24 => Val(PC2_A::VALUE14),
            25 => Val(PC2_A::VALUE15),
            26 => Val(PC2_A::VALUE16),
            27 => Val(PC2_A::VALUE17),
            28 => Val(PC2_A::VALUE18),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC2_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC2_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC2_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC2_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC2_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC2_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC2_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC2_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC2_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC2_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC2_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC2_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC2_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC2_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC2_A::VALUE18
    }
}
#[doc = "Write proxy for field `PC2`"]
pub struct PC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC2_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC2_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC2_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC2_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC2_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC2_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC2_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC2_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC2_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC2_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC2_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC2_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC2_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC2_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC2_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC2_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC2_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC2_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC3_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl From<PC3_A> for u8 {
    #[inline(always)]
    fn from(variant: PC3_A) -> Self {
        match variant {
            PC3_A::VALUE1 => 0,
            PC3_A::VALUE2 => 1,
            PC3_A::VALUE3 => 2,
            PC3_A::VALUE4 => 3,
            PC3_A::VALUE5 => 4,
            PC3_A::VALUE6 => 5,
            PC3_A::VALUE7 => 6,
            PC3_A::VALUE8 => 7,
            PC3_A::VALUE9 => 16,
            PC3_A::VALUE10 => 17,
            PC3_A::VALUE11 => 18,
            PC3_A::VALUE12 => 19,
            PC3_A::VALUE13 => 20,
            PC3_A::VALUE14 => 24,
            PC3_A::VALUE15 => 25,
            PC3_A::VALUE16 => 26,
            PC3_A::VALUE17 => 27,
            PC3_A::VALUE18 => 28,
        }
    }
}
#[doc = "Reader of field `PC3`"]
pub type PC3_R = crate::R<u8, PC3_A>;
impl PC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PC3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PC3_A::VALUE1),
            1 => Val(PC3_A::VALUE2),
            2 => Val(PC3_A::VALUE3),
            3 => Val(PC3_A::VALUE4),
            4 => Val(PC3_A::VALUE5),
            5 => Val(PC3_A::VALUE6),
            6 => Val(PC3_A::VALUE7),
            7 => Val(PC3_A::VALUE8),
            16 => Val(PC3_A::VALUE9),
            17 => Val(PC3_A::VALUE10),
            18 => Val(PC3_A::VALUE11),
            19 => Val(PC3_A::VALUE12),
            20 => Val(PC3_A::VALUE13),
            24 => Val(PC3_A::VALUE14),
            25 => Val(PC3_A::VALUE15),
            26 => Val(PC3_A::VALUE16),
            27 => Val(PC3_A::VALUE17),
            28 => Val(PC3_A::VALUE18),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC3_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC3_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC3_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC3_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC3_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC3_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC3_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC3_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC3_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC3_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC3_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC3_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC3_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC3_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC3_A::VALUE18
    }
}
#[doc = "Write proxy for field `PC3`"]
pub struct PC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC3_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC3_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC3_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC3_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC3_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC3_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC3_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC3_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC3_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC3_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC3_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC3_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC3_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC3_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC3_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC3_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC3_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC3_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> PC0_R {
        PC0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> PC1_R {
        PC1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> PC2_R {
        PC2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> PC3_R {
        PC3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&mut self) -> PC0_W {
        PC0_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&mut self) -> PC1_W {
        PC1_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&mut self) -> PC2_W {
        PC2_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&mut self) -> PC3_W {
        PC3_W { w: self }
    }
}
