#[doc = "Register `IOCR0` reader"]
pub type R = crate::R<IOCR0_SPEC>;
#[doc = "Register `IOCR0` writer"]
pub type W = crate::W<IOCR0_SPEC>;
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC0_A {
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
impl From<PC0_A> for u8 {
    #[inline(always)]
    fn from(variant: PC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC0_A {
    type Ux = u8;
}
impl crate::IsEnum for PC0_A {}
#[doc = "Field `PC0` reader - Port Control for Port n Pin 0 to 3"]
pub type PC0_R = crate::FieldReader<PC0_A>;
impl PC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC0_A> {
        match self.bits {
            0 => Some(PC0_A::VALUE1),
            1 => Some(PC0_A::VALUE2),
            2 => Some(PC0_A::VALUE3),
            3 => Some(PC0_A::VALUE4),
            4 => Some(PC0_A::VALUE5),
            5 => Some(PC0_A::VALUE6),
            6 => Some(PC0_A::VALUE7),
            7 => Some(PC0_A::VALUE8),
            16 => Some(PC0_A::VALUE9),
            17 => Some(PC0_A::VALUE10),
            18 => Some(PC0_A::VALUE11),
            19 => Some(PC0_A::VALUE12),
            20 => Some(PC0_A::VALUE13),
            24 => Some(PC0_A::VALUE14),
            25 => Some(PC0_A::VALUE15),
            26 => Some(PC0_A::VALUE16),
            27 => Some(PC0_A::VALUE17),
            28 => Some(PC0_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC0_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC0_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC0_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC0_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC0_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC0_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC0_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC0_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC0_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC0_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC0_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC0_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC0_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC0_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC0_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC0_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC0_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC0_A::VALUE18
    }
}
#[doc = "Field `PC0` writer - Port Control for Port n Pin 0 to 3"]
pub type PC0_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC0_A>;
impl<'a, REG> PC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC0_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC1_A {
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
impl From<PC1_A> for u8 {
    #[inline(always)]
    fn from(variant: PC1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC1_A {
    type Ux = u8;
}
impl crate::IsEnum for PC1_A {}
#[doc = "Field `PC1` reader - Port Control for Port n Pin 0 to 3"]
pub type PC1_R = crate::FieldReader<PC1_A>;
impl PC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC1_A> {
        match self.bits {
            0 => Some(PC1_A::VALUE1),
            1 => Some(PC1_A::VALUE2),
            2 => Some(PC1_A::VALUE3),
            3 => Some(PC1_A::VALUE4),
            4 => Some(PC1_A::VALUE5),
            5 => Some(PC1_A::VALUE6),
            6 => Some(PC1_A::VALUE7),
            7 => Some(PC1_A::VALUE8),
            16 => Some(PC1_A::VALUE9),
            17 => Some(PC1_A::VALUE10),
            18 => Some(PC1_A::VALUE11),
            19 => Some(PC1_A::VALUE12),
            20 => Some(PC1_A::VALUE13),
            24 => Some(PC1_A::VALUE14),
            25 => Some(PC1_A::VALUE15),
            26 => Some(PC1_A::VALUE16),
            27 => Some(PC1_A::VALUE17),
            28 => Some(PC1_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC1_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC1_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC1_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC1_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC1_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC1_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC1_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC1_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC1_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC1_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC1_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC1_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC1_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC1_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC1_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC1_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC1_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC1_A::VALUE18
    }
}
#[doc = "Field `PC1` writer - Port Control for Port n Pin 0 to 3"]
pub type PC1_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC1_A>;
impl<'a, REG> PC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC1_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC2_A {
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
impl From<PC2_A> for u8 {
    #[inline(always)]
    fn from(variant: PC2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC2_A {
    type Ux = u8;
}
impl crate::IsEnum for PC2_A {}
#[doc = "Field `PC2` reader - Port Control for Port n Pin 0 to 3"]
pub type PC2_R = crate::FieldReader<PC2_A>;
impl PC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC2_A> {
        match self.bits {
            0 => Some(PC2_A::VALUE1),
            1 => Some(PC2_A::VALUE2),
            2 => Some(PC2_A::VALUE3),
            3 => Some(PC2_A::VALUE4),
            4 => Some(PC2_A::VALUE5),
            5 => Some(PC2_A::VALUE6),
            6 => Some(PC2_A::VALUE7),
            7 => Some(PC2_A::VALUE8),
            16 => Some(PC2_A::VALUE9),
            17 => Some(PC2_A::VALUE10),
            18 => Some(PC2_A::VALUE11),
            19 => Some(PC2_A::VALUE12),
            20 => Some(PC2_A::VALUE13),
            24 => Some(PC2_A::VALUE14),
            25 => Some(PC2_A::VALUE15),
            26 => Some(PC2_A::VALUE16),
            27 => Some(PC2_A::VALUE17),
            28 => Some(PC2_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC2_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC2_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC2_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC2_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC2_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC2_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC2_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC2_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC2_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC2_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC2_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC2_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC2_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC2_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC2_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC2_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC2_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC2_A::VALUE18
    }
}
#[doc = "Field `PC2` writer - Port Control for Port n Pin 0 to 3"]
pub type PC2_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC2_A>;
impl<'a, REG> PC2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC2_A::VALUE18)
    }
}
#[doc = "Port Control for Port n Pin 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC3_A {
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
impl From<PC3_A> for u8 {
    #[inline(always)]
    fn from(variant: PC3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC3_A {
    type Ux = u8;
}
impl crate::IsEnum for PC3_A {}
#[doc = "Field `PC3` reader - Port Control for Port n Pin 0 to 3"]
pub type PC3_R = crate::FieldReader<PC3_A>;
impl PC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PC3_A> {
        match self.bits {
            0 => Some(PC3_A::VALUE1),
            1 => Some(PC3_A::VALUE2),
            2 => Some(PC3_A::VALUE3),
            3 => Some(PC3_A::VALUE4),
            4 => Some(PC3_A::VALUE5),
            5 => Some(PC3_A::VALUE6),
            6 => Some(PC3_A::VALUE7),
            7 => Some(PC3_A::VALUE8),
            16 => Some(PC3_A::VALUE9),
            17 => Some(PC3_A::VALUE10),
            18 => Some(PC3_A::VALUE11),
            19 => Some(PC3_A::VALUE12),
            20 => Some(PC3_A::VALUE13),
            24 => Some(PC3_A::VALUE14),
            25 => Some(PC3_A::VALUE15),
            26 => Some(PC3_A::VALUE16),
            27 => Some(PC3_A::VALUE17),
            28 => Some(PC3_A::VALUE18),
            _ => None,
        }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PC3_A::VALUE1
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PC3_A::VALUE2
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PC3_A::VALUE3
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PC3_A::VALUE4
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PC3_A::VALUE5
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PC3_A::VALUE6
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PC3_A::VALUE7
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == PC3_A::VALUE8
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == PC3_A::VALUE9
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == PC3_A::VALUE10
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == PC3_A::VALUE11
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == PC3_A::VALUE12
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == PC3_A::VALUE13
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == PC3_A::VALUE14
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == PC3_A::VALUE15
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == PC3_A::VALUE16
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn is_value17(&self) -> bool {
        *self == PC3_A::VALUE17
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn is_value18(&self) -> bool {
        *self == PC3_A::VALUE18
    }
}
#[doc = "Field `PC3` writer - Port Control for Port n Pin 0 to 3"]
pub type PC3_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PC3_A>;
impl<'a, REG> PC3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input - No internal pull device active"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline(always)]
    pub fn value17(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline(always)]
    pub fn value18(self) -> &'a mut crate::W<REG> {
        self.variant(PC3_A::VALUE18)
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
    pub fn pc0(&mut self) -> PC0_W<IOCR0_SPEC> {
        PC0_W::new(self, 3)
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc1(&mut self) -> PC1_W<IOCR0_SPEC> {
        PC1_W::new(self, 11)
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc2(&mut self) -> PC2_W<IOCR0_SPEC> {
        PC2_W::new(self, 19)
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline(always)]
    pub fn pc3(&mut self) -> PC3_W<IOCR0_SPEC> {
        PC3_W::new(self, 27)
    }
}
#[doc = "Port 8 Input/Output Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`iocr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOCR0_SPEC;
impl crate::RegisterSpec for IOCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocr0::R`](R) reader structure"]
impl crate::Readable for IOCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iocr0::W`](W) writer structure"]
impl crate::Writable for IOCR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCR0 to value 0"]
impl crate::Resettable for IOCR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
