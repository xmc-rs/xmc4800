#[doc = "Register `IOCR12` reader"]
pub type R = crate::R<Iocr12Spec>;
#[doc = "Register `IOCR12` writer"]
pub type W = crate::W<Iocr12Spec>;
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc12 {
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
impl From<Pc12> for u8 {
    #[inline(always)]
    fn from(variant: Pc12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc12 {
    type Ux = u8;
}
#[doc = "Field `PC12` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc12R = crate::FieldReader<Pc12>;
impl Pc12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc12> {
        match self.bits {
            0 => Some(Pc12::Value1),
            1 => Some(Pc12::Value2),
            2 => Some(Pc12::Value3),
            3 => Some(Pc12::Value4),
            4 => Some(Pc12::Value5),
            5 => Some(Pc12::Value6),
            6 => Some(Pc12::Value7),
            7 => Some(Pc12::Value8),
            16 => Some(Pc12::Value9),
            17 => Some(Pc12::Value10),
            18 => Some(Pc12::Value11),
            19 => Some(Pc12::Value12),
            20 => Some(Pc12::Value13),
            24 => Some(Pc12::Value14),
            25 => Some(Pc12::Value15),
            26 => Some(Pc12::Value16),
            27 => Some(Pc12::Value17),
            28 => Some(Pc12::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc12::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc12::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc12::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc12::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc12::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc12::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc12::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc12::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc12::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc12::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc12::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc12::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc12::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc12::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc12::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc12::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc12::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc12::Value18
    }
}
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc12W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc12>;
impl<'a, REG> Pc12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc12::Value18)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc13 {
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
impl From<Pc13> for u8 {
    #[inline(always)]
    fn from(variant: Pc13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc13 {
    type Ux = u8;
}
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc13R = crate::FieldReader<Pc13>;
impl Pc13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc13> {
        match self.bits {
            0 => Some(Pc13::Value1),
            1 => Some(Pc13::Value2),
            2 => Some(Pc13::Value3),
            3 => Some(Pc13::Value4),
            4 => Some(Pc13::Value5),
            5 => Some(Pc13::Value6),
            6 => Some(Pc13::Value7),
            7 => Some(Pc13::Value8),
            16 => Some(Pc13::Value9),
            17 => Some(Pc13::Value10),
            18 => Some(Pc13::Value11),
            19 => Some(Pc13::Value12),
            20 => Some(Pc13::Value13),
            24 => Some(Pc13::Value14),
            25 => Some(Pc13::Value15),
            26 => Some(Pc13::Value16),
            27 => Some(Pc13::Value17),
            28 => Some(Pc13::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc13::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc13::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc13::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc13::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc13::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc13::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc13::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc13::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc13::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc13::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc13::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc13::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc13::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc13::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc13::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc13::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc13::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc13::Value18
    }
}
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc13W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc13>;
impl<'a, REG> Pc13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc13::Value18)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc14 {
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
impl From<Pc14> for u8 {
    #[inline(always)]
    fn from(variant: Pc14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc14 {
    type Ux = u8;
}
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc14R = crate::FieldReader<Pc14>;
impl Pc14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc14> {
        match self.bits {
            0 => Some(Pc14::Value1),
            1 => Some(Pc14::Value2),
            2 => Some(Pc14::Value3),
            3 => Some(Pc14::Value4),
            4 => Some(Pc14::Value5),
            5 => Some(Pc14::Value6),
            6 => Some(Pc14::Value7),
            7 => Some(Pc14::Value8),
            16 => Some(Pc14::Value9),
            17 => Some(Pc14::Value10),
            18 => Some(Pc14::Value11),
            19 => Some(Pc14::Value12),
            20 => Some(Pc14::Value13),
            24 => Some(Pc14::Value14),
            25 => Some(Pc14::Value15),
            26 => Some(Pc14::Value16),
            27 => Some(Pc14::Value17),
            28 => Some(Pc14::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc14::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc14::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc14::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc14::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc14::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc14::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc14::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc14::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc14::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc14::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc14::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc14::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc14::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc14::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc14::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc14::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc14::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc14::Value18
    }
}
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc14W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc14>;
impl<'a, REG> Pc14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc14::Value18)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc15 {
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
impl From<Pc15> for u8 {
    #[inline(always)]
    fn from(variant: Pc15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc15 {
    type Ux = u8;
}
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub type Pc15R = crate::FieldReader<Pc15>;
impl Pc15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc15> {
        match self.bits {
            0 => Some(Pc15::Value1),
            1 => Some(Pc15::Value2),
            2 => Some(Pc15::Value3),
            3 => Some(Pc15::Value4),
            4 => Some(Pc15::Value5),
            5 => Some(Pc15::Value6),
            6 => Some(Pc15::Value7),
            7 => Some(Pc15::Value8),
            16 => Some(Pc15::Value9),
            17 => Some(Pc15::Value10),
            18 => Some(Pc15::Value11),
            19 => Some(Pc15::Value12),
            20 => Some(Pc15::Value13),
            24 => Some(Pc15::Value14),
            25 => Some(Pc15::Value15),
            26 => Some(Pc15::Value16),
            27 => Some(Pc15::Value17),
            28 => Some(Pc15::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc15::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc15::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc15::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc15::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc15::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc15::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc15::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc15::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc15::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc15::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc15::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc15::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc15::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc15::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc15::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc15::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc15::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc15::Value18
    }
}
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub type Pc15W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc15>;
impl<'a, REG> Pc15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc15::Value18)
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> Pc12R {
        Pc12R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> Pc13R {
        Pc13R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> Pc14R {
        Pc14R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> Pc15R {
        Pc15R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc12(&mut self) -> Pc12W<Iocr12Spec> {
        Pc12W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc13(&mut self) -> Pc13W<Iocr12Spec> {
        Pc13W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc14(&mut self) -> Pc14W<Iocr12Spec> {
        Pc14W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    #[must_use]
    pub fn pc15(&mut self) -> Pc15W<Iocr12Spec> {
        Pc15W::new(self, 27)
    }
}
#[doc = "Port 1 Input/Output Control Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr12Spec;
impl crate::RegisterSpec for Iocr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr12::R`](R) reader structure"]
impl crate::Readable for Iocr12Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr12::W`](W) writer structure"]
impl crate::Writable for Iocr12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for Iocr12Spec {
    const RESET_VALUE: u32 = 0;
}
