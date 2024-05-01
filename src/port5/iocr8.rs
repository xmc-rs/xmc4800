#[doc = "Register `IOCR8` reader"]
pub type R = crate::R<Iocr8Spec>;
#[doc = "Register `IOCR8` writer"]
pub type W = crate::W<Iocr8Spec>;
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc8 {
    #[doc = "0: Input - No internal pull device active"]
    Value1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    Value2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    Value3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    Value4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    Value5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    Value6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    Value7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    Value8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    Value9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    Value10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    Value11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    Value12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    Value13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    Value14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    Value15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    Value16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    Value17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    Value18 = 28,
}
impl From<Pc8> for u8 {
    #[inline(always)]
    fn from(variant: Pc8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc8 {
    type Ux = u8;
}
impl crate::IsEnum for Pc8 {}
#[doc = "Field `PC8` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc8R = crate::FieldReader<Pc8>;
impl Pc8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc8> {
        match self.bits {
            0 => Some(Pc8::Value1),
            1 => Some(Pc8::Value2),
            2 => Some(Pc8::Value3),
            3 => Some(Pc8::Value4),
            4 => Some(Pc8::Value5),
            5 => Some(Pc8::Value6),
            6 => Some(Pc8::Value7),
            7 => Some(Pc8::Value8),
            16 => Some(Pc8::Value9),
            17 => Some(Pc8::Value10),
            18 => Some(Pc8::Value11),
            19 => Some(Pc8::Value12),
            20 => Some(Pc8::Value13),
            24 => Some(Pc8::Value14),
            25 => Some(Pc8::Value15),
            26 => Some(Pc8::Value16),
            27 => Some(Pc8::Value17),
            28 => Some(Pc8::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc8::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc8::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc8::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc8::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc8::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc8::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc8::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc8::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc8::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc8::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc8::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc8::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc8::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc8::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc8::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc8::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc8::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc8::Value18
    }
}
#[doc = "Field `PC8` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc8W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc8>;
impl<'a, REG> Pc8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc8::Value18)
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc9 {
    #[doc = "0: Input - No internal pull device active"]
    Value1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    Value2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    Value3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    Value4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    Value5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    Value6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    Value7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    Value8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    Value9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    Value10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    Value11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    Value12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    Value13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    Value14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    Value15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    Value16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    Value17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    Value18 = 28,
}
impl From<Pc9> for u8 {
    #[inline(always)]
    fn from(variant: Pc9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc9 {
    type Ux = u8;
}
impl crate::IsEnum for Pc9 {}
#[doc = "Field `PC9` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc9R = crate::FieldReader<Pc9>;
impl Pc9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc9> {
        match self.bits {
            0 => Some(Pc9::Value1),
            1 => Some(Pc9::Value2),
            2 => Some(Pc9::Value3),
            3 => Some(Pc9::Value4),
            4 => Some(Pc9::Value5),
            5 => Some(Pc9::Value6),
            6 => Some(Pc9::Value7),
            7 => Some(Pc9::Value8),
            16 => Some(Pc9::Value9),
            17 => Some(Pc9::Value10),
            18 => Some(Pc9::Value11),
            19 => Some(Pc9::Value12),
            20 => Some(Pc9::Value13),
            24 => Some(Pc9::Value14),
            25 => Some(Pc9::Value15),
            26 => Some(Pc9::Value16),
            27 => Some(Pc9::Value17),
            28 => Some(Pc9::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc9::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc9::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc9::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc9::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc9::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc9::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc9::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc9::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc9::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc9::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc9::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc9::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc9::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc9::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc9::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc9::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc9::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc9::Value18
    }
}
#[doc = "Field `PC9` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc9W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc9>;
impl<'a, REG> Pc9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc9::Value18)
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc10 {
    #[doc = "0: Input - No internal pull device active"]
    Value1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    Value2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    Value3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    Value4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    Value5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    Value6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    Value7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    Value8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    Value9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    Value10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    Value11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    Value12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    Value13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    Value14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    Value15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    Value16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    Value17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    Value18 = 28,
}
impl From<Pc10> for u8 {
    #[inline(always)]
    fn from(variant: Pc10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc10 {
    type Ux = u8;
}
impl crate::IsEnum for Pc10 {}
#[doc = "Field `PC10` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc10R = crate::FieldReader<Pc10>;
impl Pc10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc10> {
        match self.bits {
            0 => Some(Pc10::Value1),
            1 => Some(Pc10::Value2),
            2 => Some(Pc10::Value3),
            3 => Some(Pc10::Value4),
            4 => Some(Pc10::Value5),
            5 => Some(Pc10::Value6),
            6 => Some(Pc10::Value7),
            7 => Some(Pc10::Value8),
            16 => Some(Pc10::Value9),
            17 => Some(Pc10::Value10),
            18 => Some(Pc10::Value11),
            19 => Some(Pc10::Value12),
            20 => Some(Pc10::Value13),
            24 => Some(Pc10::Value14),
            25 => Some(Pc10::Value15),
            26 => Some(Pc10::Value16),
            27 => Some(Pc10::Value17),
            28 => Some(Pc10::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc10::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc10::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc10::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc10::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc10::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc10::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc10::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc10::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc10::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc10::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc10::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc10::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc10::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc10::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc10::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc10::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc10::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc10::Value18
    }
}
#[doc = "Field `PC10` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc10W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc10>;
impl<'a, REG> Pc10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc10::Value18)
    }
}
#[doc = "Port Control for Port n Pin 8 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc11 {
    #[doc = "0: Input - No internal pull device active"]
    Value1 = 0,
    #[doc = "1: Input - Internal pull-down device active"]
    Value2 = 1,
    #[doc = "2: Input - Internal pull-up device active"]
    Value3 = 2,
    #[doc = "3: Input - No internal pull device, Pn_OUTx = input value"]
    Value4 = 3,
    #[doc = "4: Input inverted - No internal pull device active"]
    Value5 = 4,
    #[doc = "5: Input inverted - Internal pull-down device active"]
    Value6 = 5,
    #[doc = "6: Input inverted - Internal pull-up device active"]
    Value7 = 6,
    #[doc = "7: Input inverted - No internal pull device, Pn_OUTx = input value"]
    Value8 = 7,
    #[doc = "16: Output Push-Pull - General-purpose output"]
    Value9 = 16,
    #[doc = "17: Output Push-Pull - Alternate output function 1"]
    Value10 = 17,
    #[doc = "18: Output Push-Pull - Alternate output function 2"]
    Value11 = 18,
    #[doc = "19: Output Push-Pull - Alternate output function 3"]
    Value12 = 19,
    #[doc = "20: Output Push-Pull - Alternate output function 4"]
    Value13 = 20,
    #[doc = "24: Output Open Drain - General-purpose output"]
    Value14 = 24,
    #[doc = "25: Output Open Drain - Alternate output function 1"]
    Value15 = 25,
    #[doc = "26: Output Open Drain - Alternate output function 2"]
    Value16 = 26,
    #[doc = "27: Output Open Drain - Alternate output function 3"]
    Value17 = 27,
    #[doc = "28: Output Open Drain - Alternate output function 4"]
    Value18 = 28,
}
impl From<Pc11> for u8 {
    #[inline(always)]
    fn from(variant: Pc11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc11 {
    type Ux = u8;
}
impl crate::IsEnum for Pc11 {}
#[doc = "Field `PC11` reader - Port Control for Port n Pin 8 to 11"]
pub type Pc11R = crate::FieldReader<Pc11>;
impl Pc11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc11> {
        match self.bits {
            0 => Some(Pc11::Value1),
            1 => Some(Pc11::Value2),
            2 => Some(Pc11::Value3),
            3 => Some(Pc11::Value4),
            4 => Some(Pc11::Value5),
            5 => Some(Pc11::Value6),
            6 => Some(Pc11::Value7),
            7 => Some(Pc11::Value8),
            16 => Some(Pc11::Value9),
            17 => Some(Pc11::Value10),
            18 => Some(Pc11::Value11),
            19 => Some(Pc11::Value12),
            20 => Some(Pc11::Value13),
            24 => Some(Pc11::Value14),
            25 => Some(Pc11::Value15),
            26 => Some(Pc11::Value16),
            27 => Some(Pc11::Value17),
            28 => Some(Pc11::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc11::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc11::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc11::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc11::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc11::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc11::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc11::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc11::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc11::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc11::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc11::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc11::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc11::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc11::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc11::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc11::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc11::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc11::Value18
    }
}
#[doc = "Field `PC11` writer - Port Control for Port n Pin 8 to 11"]
pub type Pc11W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc11>;
impl<'a, REG> Pc11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc11::Value18)
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc8(&self) -> Pc8R {
        Pc8R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc9(&self) -> Pc9R {
        Pc9R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc10(&self) -> Pc10R {
        Pc10R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    pub fn pc11(&self) -> Pc11R {
        Pc11R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc8(&mut self) -> Pc8W<Iocr8Spec> {
        Pc8W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc9(&mut self) -> Pc9W<Iocr8Spec> {
        Pc9W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc10(&mut self) -> Pc10W<Iocr8Spec> {
        Pc10W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline(always)]
    #[must_use]
    pub fn pc11(&mut self) -> Pc11W<Iocr8Spec> {
        Pc11W::new(self, 27)
    }
}
#[doc = "Port 5 Input/Output Control Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr8Spec;
impl crate::RegisterSpec for Iocr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr8::R`](R) reader structure"]
impl crate::Readable for Iocr8Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr8::W`](W) writer structure"]
impl crate::Writable for Iocr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR8 to value 0"]
impl crate::Resettable for Iocr8Spec {
    const RESET_VALUE: u32 = 0;
}
