#[doc = "Register `IOCR0` reader"]
pub type R = crate::R<Iocr0Spec>;
#[doc = "Register `IOCR0` writer"]
pub type W = crate::W<Iocr0Spec>;
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc0 {
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
impl From<Pc0> for u8 {
    #[inline(always)]
    fn from(variant: Pc0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc0 {
    type Ux = u8;
}
impl crate::IsEnum for Pc0 {}
#[doc = "Field `PC0` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc0R = crate::FieldReader<Pc0>;
impl Pc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc0> {
        match self.bits {
            0 => Some(Pc0::Value1),
            1 => Some(Pc0::Value2),
            2 => Some(Pc0::Value3),
            3 => Some(Pc0::Value4),
            4 => Some(Pc0::Value5),
            5 => Some(Pc0::Value6),
            6 => Some(Pc0::Value7),
            7 => Some(Pc0::Value8),
            16 => Some(Pc0::Value9),
            17 => Some(Pc0::Value10),
            18 => Some(Pc0::Value11),
            19 => Some(Pc0::Value12),
            20 => Some(Pc0::Value13),
            24 => Some(Pc0::Value14),
            25 => Some(Pc0::Value15),
            26 => Some(Pc0::Value16),
            27 => Some(Pc0::Value17),
            28 => Some(Pc0::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc0::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc0::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc0::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc0::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc0::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc0::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc0::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc0::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc0::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc0::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc0::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc0::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc0::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc0::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc0::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc0::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc0::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc0::Value18
    }
}
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc0W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc0>;
impl<'a, REG> Pc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc0::Value18)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc1 {
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
impl From<Pc1> for u8 {
    #[inline(always)]
    fn from(variant: Pc1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc1 {
    type Ux = u8;
}
impl crate::IsEnum for Pc1 {}
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc1R = crate::FieldReader<Pc1>;
impl Pc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc1> {
        match self.bits {
            0 => Some(Pc1::Value1),
            1 => Some(Pc1::Value2),
            2 => Some(Pc1::Value3),
            3 => Some(Pc1::Value4),
            4 => Some(Pc1::Value5),
            5 => Some(Pc1::Value6),
            6 => Some(Pc1::Value7),
            7 => Some(Pc1::Value8),
            16 => Some(Pc1::Value9),
            17 => Some(Pc1::Value10),
            18 => Some(Pc1::Value11),
            19 => Some(Pc1::Value12),
            20 => Some(Pc1::Value13),
            24 => Some(Pc1::Value14),
            25 => Some(Pc1::Value15),
            26 => Some(Pc1::Value16),
            27 => Some(Pc1::Value17),
            28 => Some(Pc1::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc1::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc1::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc1::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc1::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc1::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc1::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc1::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc1::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc1::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc1::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc1::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc1::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc1::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc1::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc1::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc1::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc1::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc1::Value18
    }
}
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc1W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc1>;
impl<'a, REG> Pc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc1::Value18)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc2 {
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
impl From<Pc2> for u8 {
    #[inline(always)]
    fn from(variant: Pc2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc2 {
    type Ux = u8;
}
impl crate::IsEnum for Pc2 {}
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc2R = crate::FieldReader<Pc2>;
impl Pc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc2> {
        match self.bits {
            0 => Some(Pc2::Value1),
            1 => Some(Pc2::Value2),
            2 => Some(Pc2::Value3),
            3 => Some(Pc2::Value4),
            4 => Some(Pc2::Value5),
            5 => Some(Pc2::Value6),
            6 => Some(Pc2::Value7),
            7 => Some(Pc2::Value8),
            16 => Some(Pc2::Value9),
            17 => Some(Pc2::Value10),
            18 => Some(Pc2::Value11),
            19 => Some(Pc2::Value12),
            20 => Some(Pc2::Value13),
            24 => Some(Pc2::Value14),
            25 => Some(Pc2::Value15),
            26 => Some(Pc2::Value16),
            27 => Some(Pc2::Value17),
            28 => Some(Pc2::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc2::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc2::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc2::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc2::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc2::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc2::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc2::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc2::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc2::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc2::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc2::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc2::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc2::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc2::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc2::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc2::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc2::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc2::Value18
    }
}
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc2W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc2>;
impl<'a, REG> Pc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc2::Value18)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pc3 {
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
impl From<Pc3> for u8 {
    #[inline(always)]
    fn from(variant: Pc3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pc3 {
    type Ux = u8;
}
impl crate::IsEnum for Pc3 {}
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub type Pc3R = crate::FieldReader<Pc3>;
impl Pc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pc3> {
        match self.bits {
            0 => Some(Pc3::Value1),
            1 => Some(Pc3::Value2),
            2 => Some(Pc3::Value3),
            3 => Some(Pc3::Value4),
            4 => Some(Pc3::Value5),
            5 => Some(Pc3::Value6),
            6 => Some(Pc3::Value7),
            7 => Some(Pc3::Value8),
            16 => Some(Pc3::Value9),
            17 => Some(Pc3::Value10),
            18 => Some(Pc3::Value11),
            19 => Some(Pc3::Value12),
            20 => Some(Pc3::Value13),
            24 => Some(Pc3::Value14),
            25 => Some(Pc3::Value15),
            26 => Some(Pc3::Value16),
            27 => Some(Pc3::Value17),
            28 => Some(Pc3::Value18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pc3::Value1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pc3::Value2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pc3::Value3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pc3::Value4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Pc3::Value5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Pc3::Value6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Pc3::Value7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Pc3::Value8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Pc3::Value9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Pc3::Value10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == Pc3::Value11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == Pc3::Value12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == Pc3::Value13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == Pc3::Value14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == Pc3::Value15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == Pc3::Value16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == Pc3::Value17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == Pc3::Value18
    }
}
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub type Pc3W<'a, REG> = crate::FieldWriter<'a, REG, 5, Pc3>;
impl<'a, REG> Pc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(Pc3::Value18)
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc0(&self) -> Pc0R {
        Pc0R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&self) -> Pc1R {
        Pc1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&self) -> Pc2R {
        Pc2R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&self) -> Pc3R {
        Pc3R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc0(&mut self) -> Pc0W<Iocr0Spec> {
        Pc0W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc1(&mut self) -> Pc1W<Iocr0Spec> {
        Pc1W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc2(&mut self) -> Pc2W<Iocr0Spec> {
        Pc2W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    #[must_use]
    pub fn pc3(&mut self) -> Pc3W<Iocr0Spec> {
        Pc3W::new(self, 27)
    }
}
#[doc = "Port 0 Input/Output Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocr0Spec;
impl crate::RegisterSpec for Iocr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr0::R`](R) reader structure"]
impl crate::Readable for Iocr0Spec {}
#[doc = "`write(|w| ..)` method takes [`iocr0::W`](W) writer structure"]
impl crate::Writable for Iocr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for Iocr0Spec {
    const RESET_VALUE: u32 = 0;
}
