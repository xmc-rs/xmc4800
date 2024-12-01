#[doc = "Register `IOCR4` reader"]
pub type R = crate::R<IOCR4_SPEC>;
#[doc = "Register `IOCR4` writer"]
pub type W = crate::W<IOCR4_SPEC>;
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC4_A {
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
impl From<PC4_A> for u8 {
    #[inline(always)]
    fn from(variant: PC4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC4_A {
    type Ux = u8;
}
impl crate::IsEnum for PC4_A {}
#[doc = "Field `PC4` reader - Port Control for Port n Pin 4 to 7"]
pub type PC4_R = crate::FieldReader<PC4_A>;
impl PC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC4_A> {
        match self.bits {
            0 => Some(PC4_A::VALUE1),
            1 => Some(PC4_A::VALUE2),
            2 => Some(PC4_A::VALUE3),
            3 => Some(PC4_A::VALUE4),
            4 => Some(PC4_A::VALUE5),
            5 => Some(PC4_A::VALUE6),
            6 => Some(PC4_A::VALUE7),
            7 => Some(PC4_A::VALUE8),
            16 => Some(PC4_A::VALUE9),
            17 => Some(PC4_A::VALUE10),
            18 => Some(PC4_A::VALUE11),
            19 => Some(PC4_A::VALUE12),
            20 => Some(PC4_A::VALUE13),
            24 => Some(PC4_A::VALUE14),
            25 => Some(PC4_A::VALUE15),
            26 => Some(PC4_A::VALUE16),
            27 => Some(PC4_A::VALUE17),
            28 => Some(PC4_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC4_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC4_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC4_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC4_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC4_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC4_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC4_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC4_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC4_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC4_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC4_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC4_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC4_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC4_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC4_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC4_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC4_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC4_A::VALUE18
    }
}
#[doc = "Field `PC4` writer - Port Control for Port n Pin 4 to 7"]
pub type PC4_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC4_A>;
impl<'a, REG> PC4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC4_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC5_A {
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
impl From<PC5_A> for u8 {
    #[inline(always)]
    fn from(variant: PC5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC5_A {
    type Ux = u8;
}
impl crate::IsEnum for PC5_A {}
#[doc = "Field `PC5` reader - Port Control for Port n Pin 4 to 7"]
pub type PC5_R = crate::FieldReader<PC5_A>;
impl PC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC5_A> {
        match self.bits {
            0 => Some(PC5_A::VALUE1),
            1 => Some(PC5_A::VALUE2),
            2 => Some(PC5_A::VALUE3),
            3 => Some(PC5_A::VALUE4),
            4 => Some(PC5_A::VALUE5),
            5 => Some(PC5_A::VALUE6),
            6 => Some(PC5_A::VALUE7),
            7 => Some(PC5_A::VALUE8),
            16 => Some(PC5_A::VALUE9),
            17 => Some(PC5_A::VALUE10),
            18 => Some(PC5_A::VALUE11),
            19 => Some(PC5_A::VALUE12),
            20 => Some(PC5_A::VALUE13),
            24 => Some(PC5_A::VALUE14),
            25 => Some(PC5_A::VALUE15),
            26 => Some(PC5_A::VALUE16),
            27 => Some(PC5_A::VALUE17),
            28 => Some(PC5_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC5_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC5_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC5_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC5_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC5_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC5_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC5_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC5_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC5_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC5_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC5_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC5_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC5_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC5_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC5_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC5_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC5_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC5_A::VALUE18
    }
}
#[doc = "Field `PC5` writer - Port Control for Port n Pin 4 to 7"]
pub type PC5_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC5_A>;
impl<'a, REG> PC5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC5_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC6_A {
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
impl From<PC6_A> for u8 {
    #[inline(always)]
    fn from(variant: PC6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC6_A {
    type Ux = u8;
}
impl crate::IsEnum for PC6_A {}
#[doc = "Field `PC6` reader - Port Control for Port n Pin 4 to 7"]
pub type PC6_R = crate::FieldReader<PC6_A>;
impl PC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC6_A> {
        match self.bits {
            0 => Some(PC6_A::VALUE1),
            1 => Some(PC6_A::VALUE2),
            2 => Some(PC6_A::VALUE3),
            3 => Some(PC6_A::VALUE4),
            4 => Some(PC6_A::VALUE5),
            5 => Some(PC6_A::VALUE6),
            6 => Some(PC6_A::VALUE7),
            7 => Some(PC6_A::VALUE8),
            16 => Some(PC6_A::VALUE9),
            17 => Some(PC6_A::VALUE10),
            18 => Some(PC6_A::VALUE11),
            19 => Some(PC6_A::VALUE12),
            20 => Some(PC6_A::VALUE13),
            24 => Some(PC6_A::VALUE14),
            25 => Some(PC6_A::VALUE15),
            26 => Some(PC6_A::VALUE16),
            27 => Some(PC6_A::VALUE17),
            28 => Some(PC6_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC6_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC6_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC6_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC6_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC6_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC6_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC6_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC6_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC6_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC6_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC6_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC6_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC6_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC6_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC6_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC6_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC6_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC6_A::VALUE18
    }
}
#[doc = "Field `PC6` writer - Port Control for Port n Pin 4 to 7"]
pub type PC6_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC6_A>;
impl<'a, REG> PC6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC6_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 4 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC7_A {
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
impl From<PC7_A> for u8 {
    #[inline(always)]
    fn from(variant: PC7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC7_A {
    type Ux = u8;
}
impl crate::IsEnum for PC7_A {}
#[doc = "Field `PC7` reader - Port Control for Port n Pin 4 to 7"]
pub type PC7_R = crate::FieldReader<PC7_A>;
impl PC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC7_A> {
        match self.bits {
            0 => Some(PC7_A::VALUE1),
            1 => Some(PC7_A::VALUE2),
            2 => Some(PC7_A::VALUE3),
            3 => Some(PC7_A::VALUE4),
            4 => Some(PC7_A::VALUE5),
            5 => Some(PC7_A::VALUE6),
            6 => Some(PC7_A::VALUE7),
            7 => Some(PC7_A::VALUE8),
            16 => Some(PC7_A::VALUE9),
            17 => Some(PC7_A::VALUE10),
            18 => Some(PC7_A::VALUE11),
            19 => Some(PC7_A::VALUE12),
            20 => Some(PC7_A::VALUE13),
            24 => Some(PC7_A::VALUE14),
            25 => Some(PC7_A::VALUE15),
            26 => Some(PC7_A::VALUE16),
            27 => Some(PC7_A::VALUE17),
            28 => Some(PC7_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC7_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC7_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC7_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC7_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC7_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC7_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC7_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC7_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC7_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC7_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC7_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC7_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC7_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC7_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC7_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC7_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC7_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC7_A::VALUE18
    }
}
#[doc = "Field `PC7` writer - Port Control for Port n Pin 4 to 7"]
pub type PC7_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC7_A>;
impl<'a, REG> PC7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC7_A::VALUE18)
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&self) -> PC4_R {
        PC4_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&self) -> PC5_R {
        PC5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&self) -> PC6_R {
        PC6_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&self) -> PC7_R {
        PC7_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc4(&mut self) -> PC4_W<IOCR4_SPEC> {
        PC4_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc5(&mut self) -> PC5_W<IOCR4_SPEC> {
        PC5_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc6(&mut self) -> PC6_W<IOCR4_SPEC> {
        PC6_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline(always)]
    pub fn pc7(&mut self) -> PC7_W<IOCR4_SPEC> {
        PC7_W::new(self, 27)
    }
}
#[doc = "Port 15 Input/Output Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`iocr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR4_SPEC;
impl crate::RegisterSpec for IOCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr4::R`](R) reader structure"]
impl crate::Readable for IOCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr4::W`](W) writer structure"]
impl crate::Writable for IOCR4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR4 to value 0"]
impl crate::Resettable for IOCR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
