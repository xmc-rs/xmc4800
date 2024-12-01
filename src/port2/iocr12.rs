#[doc = "Register `IOCR12` reader"]
pub type R = crate::R<IOCR12_SPEC>;
#[doc = "Register `IOCR12` writer"]
pub type W = crate::W<IOCR12_SPEC>;
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC12_A {
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
impl From<PC12_A> for u8 {
    #[inline(always)]
    fn from(variant: PC12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC12_A {
    type Ux = u8;
}
impl crate::IsEnum for PC12_A {}
#[doc = "Field `PC12` reader - Port Control for Port n Pin 12 to 15"]
pub type PC12_R = crate::FieldReader<PC12_A>;
impl PC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC12_A> {
        match self.bits {
            0 => Some(PC12_A::VALUE1),
            1 => Some(PC12_A::VALUE2),
            2 => Some(PC12_A::VALUE3),
            3 => Some(PC12_A::VALUE4),
            4 => Some(PC12_A::VALUE5),
            5 => Some(PC12_A::VALUE6),
            6 => Some(PC12_A::VALUE7),
            7 => Some(PC12_A::VALUE8),
            16 => Some(PC12_A::VALUE9),
            17 => Some(PC12_A::VALUE10),
            18 => Some(PC12_A::VALUE11),
            19 => Some(PC12_A::VALUE12),
            20 => Some(PC12_A::VALUE13),
            24 => Some(PC12_A::VALUE14),
            25 => Some(PC12_A::VALUE15),
            26 => Some(PC12_A::VALUE16),
            27 => Some(PC12_A::VALUE17),
            28 => Some(PC12_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC12_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC12_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC12_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC12_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC12_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC12_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC12_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC12_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC12_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC12_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC12_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC12_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC12_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC12_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC12_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC12_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC12_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC12_A::VALUE18
    }
}
#[doc = "Field `PC12` writer - Port Control for Port n Pin 12 to 15"]
pub type PC12_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC12_A>;
impl<'a, REG> PC12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC12_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC13_A {
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
impl From<PC13_A> for u8 {
    #[inline(always)]
    fn from(variant: PC13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC13_A {
    type Ux = u8;
}
impl crate::IsEnum for PC13_A {}
#[doc = "Field `PC13` reader - Port Control for Port n Pin 12 to 15"]
pub type PC13_R = crate::FieldReader<PC13_A>;
impl PC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC13_A> {
        match self.bits {
            0 => Some(PC13_A::VALUE1),
            1 => Some(PC13_A::VALUE2),
            2 => Some(PC13_A::VALUE3),
            3 => Some(PC13_A::VALUE4),
            4 => Some(PC13_A::VALUE5),
            5 => Some(PC13_A::VALUE6),
            6 => Some(PC13_A::VALUE7),
            7 => Some(PC13_A::VALUE8),
            16 => Some(PC13_A::VALUE9),
            17 => Some(PC13_A::VALUE10),
            18 => Some(PC13_A::VALUE11),
            19 => Some(PC13_A::VALUE12),
            20 => Some(PC13_A::VALUE13),
            24 => Some(PC13_A::VALUE14),
            25 => Some(PC13_A::VALUE15),
            26 => Some(PC13_A::VALUE16),
            27 => Some(PC13_A::VALUE17),
            28 => Some(PC13_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC13_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC13_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC13_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC13_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC13_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC13_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC13_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC13_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC13_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC13_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC13_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC13_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC13_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC13_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC13_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC13_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC13_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC13_A::VALUE18
    }
}
#[doc = "Field `PC13` writer - Port Control for Port n Pin 12 to 15"]
pub type PC13_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC13_A>;
impl<'a, REG> PC13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC13_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC14_A {
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
impl From<PC14_A> for u8 {
    #[inline(always)]
    fn from(variant: PC14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC14_A {
    type Ux = u8;
}
impl crate::IsEnum for PC14_A {}
#[doc = "Field `PC14` reader - Port Control for Port n Pin 12 to 15"]
pub type PC14_R = crate::FieldReader<PC14_A>;
impl PC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC14_A> {
        match self.bits {
            0 => Some(PC14_A::VALUE1),
            1 => Some(PC14_A::VALUE2),
            2 => Some(PC14_A::VALUE3),
            3 => Some(PC14_A::VALUE4),
            4 => Some(PC14_A::VALUE5),
            5 => Some(PC14_A::VALUE6),
            6 => Some(PC14_A::VALUE7),
            7 => Some(PC14_A::VALUE8),
            16 => Some(PC14_A::VALUE9),
            17 => Some(PC14_A::VALUE10),
            18 => Some(PC14_A::VALUE11),
            19 => Some(PC14_A::VALUE12),
            20 => Some(PC14_A::VALUE13),
            24 => Some(PC14_A::VALUE14),
            25 => Some(PC14_A::VALUE15),
            26 => Some(PC14_A::VALUE16),
            27 => Some(PC14_A::VALUE17),
            28 => Some(PC14_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC14_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC14_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC14_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC14_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC14_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC14_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC14_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC14_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC14_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC14_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC14_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC14_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC14_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC14_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC14_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC14_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC14_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC14_A::VALUE18
    }
}
#[doc = "Field `PC14` writer - Port Control for Port n Pin 12 to 15"]
pub type PC14_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC14_A>;
impl<'a, REG> PC14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC14_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 12 to 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC15_A {
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
impl From<PC15_A> for u8 {
    #[inline(always)]
    fn from(variant: PC15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC15_A {
    type Ux = u8;
}
impl crate::IsEnum for PC15_A {}
#[doc = "Field `PC15` reader - Port Control for Port n Pin 12 to 15"]
pub type PC15_R = crate::FieldReader<PC15_A>;
impl PC15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC15_A> {
        match self.bits {
            0 => Some(PC15_A::VALUE1),
            1 => Some(PC15_A::VALUE2),
            2 => Some(PC15_A::VALUE3),
            3 => Some(PC15_A::VALUE4),
            4 => Some(PC15_A::VALUE5),
            5 => Some(PC15_A::VALUE6),
            6 => Some(PC15_A::VALUE7),
            7 => Some(PC15_A::VALUE8),
            16 => Some(PC15_A::VALUE9),
            17 => Some(PC15_A::VALUE10),
            18 => Some(PC15_A::VALUE11),
            19 => Some(PC15_A::VALUE12),
            20 => Some(PC15_A::VALUE13),
            24 => Some(PC15_A::VALUE14),
            25 => Some(PC15_A::VALUE15),
            26 => Some(PC15_A::VALUE16),
            27 => Some(PC15_A::VALUE17),
            28 => Some(PC15_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC15_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC15_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC15_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC15_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC15_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC15_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC15_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC15_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC15_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC15_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC15_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC15_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC15_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC15_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC15_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC15_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC15_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC15_A::VALUE18
    }
}
#[doc = "Field `PC15` writer - Port Control for Port n Pin 12 to 15"]
pub type PC15_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC15_A>;
impl<'a, REG> PC15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC15_A::VALUE18)
    }
}
impl R {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&self) -> PC12_R {
        PC12_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&self) -> PC13_R {
        PC13_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&self) -> PC14_R {
        PC14_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&self) -> PC15_R {
        PC15_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc12(&mut self) -> PC12_W<IOCR12_SPEC> {
        PC12_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc13(&mut self) -> PC13_W<IOCR12_SPEC> {
        PC13_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc14(&mut self) -> PC14_W<IOCR12_SPEC> {
        PC14_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline(always)]
    pub fn pc15(&mut self) -> PC15_W<IOCR12_SPEC> {
        PC15_W::new(self, 27)
    }
}
#[doc = "Port 2 Input/Output Control Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`iocr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR12_SPEC;
impl crate::RegisterSpec for IOCR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr12::R`](R) reader structure"]
impl crate::Readable for IOCR12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr12::W`](W) writer structure"]
impl crate::Writable for IOCR12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR12 to value 0"]
impl crate::Resettable for IOCR12_SPEC {
    const RESET_VALUE: u32 = 0;
}
