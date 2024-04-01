#[doc = "Register `IOCR4` reader"]
pub type R = crate::R<Iocr4Spec>;
#[doc = "Register `IOCR4` writer"]
pub type W = crate::W<Iocr4Spec>;
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc4 {
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
impl From<Pc4> for u8 {
    #[inline(always)]
    fn from(variant: Pc4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc4 {
    type Ux = u8;
}
impl crate::IsEnum for Pc4 {}
#[doc = "Field `PC4` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc4R = crate::FieldReader<Pc4>;
impl Pc4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc4> {
        match self.bits {
            0 => Some(Pc4::Value1),
            1 => Some(Pc4::Value2),
            2 => Some(Pc4::Value3),
            3 => Some(Pc4::Value4),
            4 => Some(Pc4::Value5),
            5 => Some(Pc4::Value6),
            6 => Some(Pc4::Value7),
            7 => Some(Pc4::Value8),
            16 => Some(Pc4::Value9),
            17 => Some(Pc4::Value10),
            18 => Some(Pc4::Value11),
            19 => Some(Pc4::Value12),
            20 => Some(Pc4::Value13),
            24 => Some(Pc4::Value14),
            25 => Some(Pc4::Value15),
            26 => Some(Pc4::Value16),
            27 => Some(Pc4::Value17),
            28 => Some(Pc4::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc4::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc4::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc4::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc4::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc4::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc4::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc4::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc4::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc4::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc4::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc4::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc4::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc4::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc4::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc4::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc4::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc4::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc4::Value18
    }
}
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc4W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc4>;
impl<'a, REG> Pc4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc4::Value18)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc5 {
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
impl From<Pc5> for u8 {
    #[inline(always)]
    fn from(variant: Pc5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc5 {
    type Ux = u8;
}
impl crate::IsEnum for Pc5 {}
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc5R = crate::FieldReader<Pc5>;
impl Pc5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc5> {
        match self.bits {
            0 => Some(Pc5::Value1),
            1 => Some(Pc5::Value2),
            2 => Some(Pc5::Value3),
            3 => Some(Pc5::Value4),
            4 => Some(Pc5::Value5),
            5 => Some(Pc5::Value6),
            6 => Some(Pc5::Value7),
            7 => Some(Pc5::Value8),
            16 => Some(Pc5::Value9),
            17 => Some(Pc5::Value10),
            18 => Some(Pc5::Value11),
            19 => Some(Pc5::Value12),
            20 => Some(Pc5::Value13),
            24 => Some(Pc5::Value14),
            25 => Some(Pc5::Value15),
            26 => Some(Pc5::Value16),
            27 => Some(Pc5::Value17),
            28 => Some(Pc5::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc5::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc5::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc5::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc5::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc5::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc5::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc5::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc5::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc5::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc5::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc5::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc5::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc5::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc5::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc5::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc5::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc5::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc5::Value18
    }
}
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc5W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc5>;
impl<'a, REG> Pc5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc5::Value18)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc6 {
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
impl From<Pc6> for u8 {
    #[inline(always)]
    fn from(variant: Pc6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc6 {
    type Ux = u8;
}
impl crate::IsEnum for Pc6 {}
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc6R = crate::FieldReader<Pc6>;
impl Pc6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc6> {
        match self.bits {
            0 => Some(Pc6::Value1),
            1 => Some(Pc6::Value2),
            2 => Some(Pc6::Value3),
            3 => Some(Pc6::Value4),
            4 => Some(Pc6::Value5),
            5 => Some(Pc6::Value6),
            6 => Some(Pc6::Value7),
            7 => Some(Pc6::Value8),
            16 => Some(Pc6::Value9),
            17 => Some(Pc6::Value10),
            18 => Some(Pc6::Value11),
            19 => Some(Pc6::Value12),
            20 => Some(Pc6::Value13),
            24 => Some(Pc6::Value14),
            25 => Some(Pc6::Value15),
            26 => Some(Pc6::Value16),
            27 => Some(Pc6::Value17),
            28 => Some(Pc6::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc6::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc6::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc6::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc6::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc6::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc6::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc6::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc6::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc6::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc6::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc6::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc6::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc6::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc6::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc6::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc6::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc6::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc6::Value18
    }
}
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc6W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc6>;
impl<'a, REG> Pc6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc6::Value18)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc7 {
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
impl From<Pc7> for u8 {
    #[inline(always)]
    fn from(variant: Pc7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc7 {
    type Ux = u8;
}
impl crate::IsEnum for Pc7 {}
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub type Pc7R = crate::FieldReader<Pc7>;
impl Pc7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc7> {
        match self.bits {
            0 => Some(Pc7::Value1),
            1 => Some(Pc7::Value2),
            2 => Some(Pc7::Value3),
            3 => Some(Pc7::Value4),
            4 => Some(Pc7::Value5),
            5 => Some(Pc7::Value6),
            6 => Some(Pc7::Value7),
            7 => Some(Pc7::Value8),
            16 => Some(Pc7::Value9),
            17 => Some(Pc7::Value10),
            18 => Some(Pc7::Value11),
            19 => Some(Pc7::Value12),
            20 => Some(Pc7::Value13),
            24 => Some(Pc7::Value14),
            25 => Some(Pc7::Value15),
            26 => Some(Pc7::Value16),
            27 => Some(Pc7::Value17),
            28 => Some(Pc7::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc7::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc7::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc7::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc7::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc7::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc7::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc7::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc7::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc7::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc7::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc7::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc7::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc7::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc7::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc7::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc7::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc7::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc7::Value18
    }
}
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub type Pc7W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc7>;
impl<'a, REG> Pc7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc7::Value18)
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> Pc4R {
        Pc4R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> Pc5R {
        Pc5R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> Pc6R {
        Pc6R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> Pc7R {
        Pc7R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc4(&mut self) -> Pc4W<Iocr4Spec> {
        Pc4W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc5(&mut self) -> Pc5W<Iocr4Spec> {
        Pc5W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc6(&mut self) -> Pc6W<Iocr4Spec> {
        Pc6W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    #[must_use]
    pub fn pc7(&mut self) -> Pc7W<Iocr4Spec> {
        Pc7W::new(self, 27)
    }
}
#[doc = "Port 1 Input/Output Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr4Spec;
impl crate::RegisterSpec for Iocr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr4::R`](R) reader structure"]
impl crate::Readable for Iocr4Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr4::W`](W) writer structure"]
impl crate::Writable for Iocr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for Iocr4Spec {
    const RESET_VALUE: u32 = 0;
}
