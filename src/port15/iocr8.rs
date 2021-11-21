#[doc = "Register `IOCR8` reader"]
pub struct R(crate::R<IOCR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCR8` writer"]
pub struct W(crate::W<IOCR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IOCR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC8_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18 = 28,
}
impl From<PC8_A> for u8 {
    #[inline(always)]
    fn from(variant: PC8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC8` reader - Port Control for Port n Pin 8 to 11"]
pub struct PC8_R(crate::FieldReader<u8, PC8_A>);
impl PC8_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC8_A> {
        match self.bits {
            0 => Some(PC8_A::VALUE1),
            1 => Some(PC8_A::VALUE2),
            2 => Some(PC8_A::VALUE3),
            3 => Some(PC8_A::VALUE4),
            4 => Some(PC8_A::VALUE5),
            5 => Some(PC8_A::VALUE6),
            6 => Some(PC8_A::VALUE7),
            7 => Some(PC8_A::VALUE8),
            16 => Some(PC8_A::VALUE9),
            17 => Some(PC8_A::VALUE10),
            18 => Some(PC8_A::VALUE11),
            19 => Some(PC8_A::VALUE12),
            20 => Some(PC8_A::VALUE13),
            24 => Some(PC8_A::VALUE14),
            25 => Some(PC8_A::VALUE15),
            26 => Some(PC8_A::VALUE16),
            27 => Some(PC8_A::VALUE17),
            28 => Some(PC8_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PC8_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PC8_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PC8_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PC8_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PC8_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == PC8_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == PC8_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == PC8_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        **self == PC8_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        **self == PC8_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        **self == PC8_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        **self == PC8_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        **self == PC8_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        **self == PC8_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        **self == PC8_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        **self == PC8_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        **self == PC8_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        **self == PC8_A::VALUE18
    }
}
impl core::ops::Deref for PC8_R {
    type Target = crate::FieldReader<u8, PC8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC8` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC8_W<'a> {
    w: &'a mut W,
}
impl<'a> PC8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC8_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC8_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC8_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC8_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC8_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC8_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC8_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC8_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC8_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC8_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC8_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC8_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC8_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC8_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC8_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC8_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC8_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC8_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC9_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18 = 28,
}
impl From<PC9_A> for u8 {
    #[inline(always)]
    fn from(variant: PC9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC9` reader - Port Control for Port n Pin 8 to 11"]
pub struct PC9_R(crate::FieldReader<u8, PC9_A>);
impl PC9_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC9_A> {
        match self.bits {
            0 => Some(PC9_A::VALUE1),
            1 => Some(PC9_A::VALUE2),
            2 => Some(PC9_A::VALUE3),
            3 => Some(PC9_A::VALUE4),
            4 => Some(PC9_A::VALUE5),
            5 => Some(PC9_A::VALUE6),
            6 => Some(PC9_A::VALUE7),
            7 => Some(PC9_A::VALUE8),
            16 => Some(PC9_A::VALUE9),
            17 => Some(PC9_A::VALUE10),
            18 => Some(PC9_A::VALUE11),
            19 => Some(PC9_A::VALUE12),
            20 => Some(PC9_A::VALUE13),
            24 => Some(PC9_A::VALUE14),
            25 => Some(PC9_A::VALUE15),
            26 => Some(PC9_A::VALUE16),
            27 => Some(PC9_A::VALUE17),
            28 => Some(PC9_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PC9_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PC9_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PC9_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PC9_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PC9_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == PC9_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == PC9_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == PC9_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        **self == PC9_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        **self == PC9_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        **self == PC9_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        **self == PC9_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        **self == PC9_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        **self == PC9_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        **self == PC9_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        **self == PC9_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        **self == PC9_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        **self == PC9_A::VALUE18
    }
}
impl core::ops::Deref for PC9_R {
    type Target = crate::FieldReader<u8, PC9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC9` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC9_W<'a> {
    w: &'a mut W,
}
impl<'a> PC9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC9_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC9_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC9_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC9_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC9_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC9_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC9_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC9_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC9_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC9_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC9_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC9_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC9_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC9_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC9_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC9_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC9_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC9_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC10_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18 = 28,
}
impl From<PC10_A> for u8 {
    #[inline(always)]
    fn from(variant: PC10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC10` reader - Port Control for Port n Pin 8 to 11"]
pub struct PC10_R(crate::FieldReader<u8, PC10_A>);
impl PC10_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC10_A> {
        match self.bits {
            0 => Some(PC10_A::VALUE1),
            1 => Some(PC10_A::VALUE2),
            2 => Some(PC10_A::VALUE3),
            3 => Some(PC10_A::VALUE4),
            4 => Some(PC10_A::VALUE5),
            5 => Some(PC10_A::VALUE6),
            6 => Some(PC10_A::VALUE7),
            7 => Some(PC10_A::VALUE8),
            16 => Some(PC10_A::VALUE9),
            17 => Some(PC10_A::VALUE10),
            18 => Some(PC10_A::VALUE11),
            19 => Some(PC10_A::VALUE12),
            20 => Some(PC10_A::VALUE13),
            24 => Some(PC10_A::VALUE14),
            25 => Some(PC10_A::VALUE15),
            26 => Some(PC10_A::VALUE16),
            27 => Some(PC10_A::VALUE17),
            28 => Some(PC10_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PC10_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PC10_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PC10_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PC10_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PC10_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == PC10_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == PC10_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == PC10_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        **self == PC10_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        **self == PC10_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        **self == PC10_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        **self == PC10_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        **self == PC10_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        **self == PC10_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        **self == PC10_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        **self == PC10_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        **self == PC10_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        **self == PC10_A::VALUE18
    }
}
impl core::ops::Deref for PC10_R {
    type Target = crate::FieldReader<u8, PC10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC10` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC10_W<'a> {
    w: &'a mut W,
}
impl<'a> PC10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC10_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC10_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC10_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC10_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC10_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC10_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC10_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC10_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC10_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC10_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC10_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC10_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC10_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC10_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC10_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC10_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC10_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC10_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PC11_A {
    #[doc = "0: Input - No internal pull device active"]
    VALUE1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    VALUE2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    VALUE3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    VALUE5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    VALUE6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    VALUE7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    VALUE9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    VALUE10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    VALUE11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    VALUE12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    VALUE13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    VALUE14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    VALUE15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    VALUE16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    VALUE17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    VALUE18 = 28,
}
impl From<PC11_A> for u8 {
    #[inline(always)]
    fn from(variant: PC11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PC11` reader - Port Control for Port n Pin 8 to 11"]
pub struct PC11_R(crate::FieldReader<u8, PC11_A>);
impl PC11_R {
    pub(crate) fn new(bits: u8) -> Self {
        PC11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PC11_A> {
        match self.bits {
            0 => Some(PC11_A::VALUE1),
            1 => Some(PC11_A::VALUE2),
            2 => Some(PC11_A::VALUE3),
            3 => Some(PC11_A::VALUE4),
            4 => Some(PC11_A::VALUE5),
            5 => Some(PC11_A::VALUE6),
            6 => Some(PC11_A::VALUE7),
            7 => Some(PC11_A::VALUE8),
            16 => Some(PC11_A::VALUE9),
            17 => Some(PC11_A::VALUE10),
            18 => Some(PC11_A::VALUE11),
            19 => Some(PC11_A::VALUE12),
            20 => Some(PC11_A::VALUE13),
            24 => Some(PC11_A::VALUE14),
            25 => Some(PC11_A::VALUE15),
            26 => Some(PC11_A::VALUE16),
            27 => Some(PC11_A::VALUE17),
            28 => Some(PC11_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PC11_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PC11_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PC11_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PC11_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == PC11_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == PC11_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == PC11_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == PC11_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        **self == PC11_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        **self == PC11_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        **self == PC11_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        **self == PC11_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        **self == PC11_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        **self == PC11_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        **self == PC11_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        **self == PC11_A::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        **self == PC11_A::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        **self == PC11_A::VALUE18
    }
}
impl core::ops::Deref for PC11_R {
    type Target = crate::FieldReader<u8, PC11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC11` writer - Port Control for Port n Pin 8 to 11"]
pub struct PC11_W<'a> {
    w: &'a mut W,
}
impl<'a> PC11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PC11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC11_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC11_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC11_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC11_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC11_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC11_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC11_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC11_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC11_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC11_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC11_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC11_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC11_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC11_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC11_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC11_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC11_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC11_A::VALUE18)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> PC8_R {
        PC8_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> PC9_R {
        PC9_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> PC10_R {
        PC10_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> PC11_R {
        PC11_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&mut self) -> PC8_W {
        PC8_W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&mut self) -> PC9_W {
        PC9_W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&mut self) -> PC10_W {
        PC10_W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&mut self) -> PC11_W {
        PC11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 15 Input/Output Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocr8](index.html) module"]
pub struct IOCR8_SPEC;
impl crate::RegisterSpec for IOCR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iocr8::R](R) reader structure"]
impl crate::Readable for IOCR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iocr8::W](W) writer structure"]
impl crate::Writable for IOCR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCR8 to value 0"]
impl crate::Resettable for IOCR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
