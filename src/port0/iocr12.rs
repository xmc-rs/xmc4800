#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCR12 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `PC12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC12R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC12R::VALUE1 => 0,
            PC12R::VALUE2 => 1,
            PC12R::VALUE3 => 2,
            PC12R::VALUE4 => 3,
            PC12R::VALUE5 => 4,
            PC12R::VALUE6 => 5,
            PC12R::VALUE7 => 6,
            PC12R::VALUE8 => 7,
            PC12R::VALUE9 => 16,
            PC12R::VALUE10 => 17,
            PC12R::VALUE11 => 18,
            PC12R::VALUE12 => 19,
            PC12R::VALUE13 => 20,
            PC12R::VALUE14 => 24,
            PC12R::VALUE15 => 25,
            PC12R::VALUE16 => 26,
            PC12R::VALUE17 => 27,
            PC12R::VALUE18 => 28,
            PC12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC12R {
        match value {
            0 => PC12R::VALUE1,
            1 => PC12R::VALUE2,
            2 => PC12R::VALUE3,
            3 => PC12R::VALUE4,
            4 => PC12R::VALUE5,
            5 => PC12R::VALUE6,
            6 => PC12R::VALUE7,
            7 => PC12R::VALUE8,
            16 => PC12R::VALUE9,
            17 => PC12R::VALUE10,
            18 => PC12R::VALUE11,
            19 => PC12R::VALUE12,
            20 => PC12R::VALUE13,
            24 => PC12R::VALUE14,
            25 => PC12R::VALUE15,
            26 => PC12R::VALUE16,
            27 => PC12R::VALUE17,
            28 => PC12R::VALUE18,
            i => PC12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC12R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC12R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC12R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC12R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC12R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC12R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC12R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC12R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC12R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC12R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC12R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC12R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC12R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC12R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC12R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC12R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC12R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC12R::VALUE18
    }
}
#[doc = "Possible values of the field `PC13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC13R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC13R::VALUE1 => 0,
            PC13R::VALUE2 => 1,
            PC13R::VALUE3 => 2,
            PC13R::VALUE4 => 3,
            PC13R::VALUE5 => 4,
            PC13R::VALUE6 => 5,
            PC13R::VALUE7 => 6,
            PC13R::VALUE8 => 7,
            PC13R::VALUE9 => 16,
            PC13R::VALUE10 => 17,
            PC13R::VALUE11 => 18,
            PC13R::VALUE12 => 19,
            PC13R::VALUE13 => 20,
            PC13R::VALUE14 => 24,
            PC13R::VALUE15 => 25,
            PC13R::VALUE16 => 26,
            PC13R::VALUE17 => 27,
            PC13R::VALUE18 => 28,
            PC13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC13R {
        match value {
            0 => PC13R::VALUE1,
            1 => PC13R::VALUE2,
            2 => PC13R::VALUE3,
            3 => PC13R::VALUE4,
            4 => PC13R::VALUE5,
            5 => PC13R::VALUE6,
            6 => PC13R::VALUE7,
            7 => PC13R::VALUE8,
            16 => PC13R::VALUE9,
            17 => PC13R::VALUE10,
            18 => PC13R::VALUE11,
            19 => PC13R::VALUE12,
            20 => PC13R::VALUE13,
            24 => PC13R::VALUE14,
            25 => PC13R::VALUE15,
            26 => PC13R::VALUE16,
            27 => PC13R::VALUE17,
            28 => PC13R::VALUE18,
            i => PC13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC13R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC13R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC13R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC13R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC13R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC13R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC13R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC13R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC13R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC13R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC13R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC13R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC13R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC13R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC13R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC13R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC13R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC13R::VALUE18
    }
}
#[doc = "Possible values of the field `PC14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC14R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC14R::VALUE1 => 0,
            PC14R::VALUE2 => 1,
            PC14R::VALUE3 => 2,
            PC14R::VALUE4 => 3,
            PC14R::VALUE5 => 4,
            PC14R::VALUE6 => 5,
            PC14R::VALUE7 => 6,
            PC14R::VALUE8 => 7,
            PC14R::VALUE9 => 16,
            PC14R::VALUE10 => 17,
            PC14R::VALUE11 => 18,
            PC14R::VALUE12 => 19,
            PC14R::VALUE13 => 20,
            PC14R::VALUE14 => 24,
            PC14R::VALUE15 => 25,
            PC14R::VALUE16 => 26,
            PC14R::VALUE17 => 27,
            PC14R::VALUE18 => 28,
            PC14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC14R {
        match value {
            0 => PC14R::VALUE1,
            1 => PC14R::VALUE2,
            2 => PC14R::VALUE3,
            3 => PC14R::VALUE4,
            4 => PC14R::VALUE5,
            5 => PC14R::VALUE6,
            6 => PC14R::VALUE7,
            7 => PC14R::VALUE8,
            16 => PC14R::VALUE9,
            17 => PC14R::VALUE10,
            18 => PC14R::VALUE11,
            19 => PC14R::VALUE12,
            20 => PC14R::VALUE13,
            24 => PC14R::VALUE14,
            25 => PC14R::VALUE15,
            26 => PC14R::VALUE16,
            27 => PC14R::VALUE17,
            28 => PC14R::VALUE18,
            i => PC14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC14R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC14R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC14R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC14R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC14R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC14R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC14R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC14R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC14R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC14R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC14R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC14R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC14R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC14R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC14R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC14R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC14R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC14R::VALUE18
    }
}
#[doc = "Possible values of the field `PC15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC15R {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PC15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC15R::VALUE1 => 0,
            PC15R::VALUE2 => 1,
            PC15R::VALUE3 => 2,
            PC15R::VALUE4 => 3,
            PC15R::VALUE5 => 4,
            PC15R::VALUE6 => 5,
            PC15R::VALUE7 => 6,
            PC15R::VALUE8 => 7,
            PC15R::VALUE9 => 16,
            PC15R::VALUE10 => 17,
            PC15R::VALUE11 => 18,
            PC15R::VALUE12 => 19,
            PC15R::VALUE13 => 20,
            PC15R::VALUE14 => 24,
            PC15R::VALUE15 => 25,
            PC15R::VALUE16 => 26,
            PC15R::VALUE17 => 27,
            PC15R::VALUE18 => 28,
            PC15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC15R {
        match value {
            0 => PC15R::VALUE1,
            1 => PC15R::VALUE2,
            2 => PC15R::VALUE3,
            3 => PC15R::VALUE4,
            4 => PC15R::VALUE5,
            5 => PC15R::VALUE6,
            6 => PC15R::VALUE7,
            7 => PC15R::VALUE8,
            16 => PC15R::VALUE9,
            17 => PC15R::VALUE10,
            18 => PC15R::VALUE11,
            19 => PC15R::VALUE12,
            20 => PC15R::VALUE13,
            24 => PC15R::VALUE14,
            25 => PC15R::VALUE15,
            26 => PC15R::VALUE16,
            27 => PC15R::VALUE17,
            28 => PC15R::VALUE18,
            i => PC15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC15R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC15R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC15R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC15R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC15R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC15R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC15R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC15R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC15R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC15R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC15R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC15R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC15R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC15R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC15R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC15R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC15R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC15R::VALUE18
    }
}
#[doc = "Values that can be written to the field `PC12`"]
pub enum PC12W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl PC12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC12W::VALUE1 => 0,
            PC12W::VALUE2 => 1,
            PC12W::VALUE3 => 2,
            PC12W::VALUE4 => 3,
            PC12W::VALUE5 => 4,
            PC12W::VALUE6 => 5,
            PC12W::VALUE7 => 6,
            PC12W::VALUE8 => 7,
            PC12W::VALUE9 => 16,
            PC12W::VALUE10 => 17,
            PC12W::VALUE11 => 18,
            PC12W::VALUE12 => 19,
            PC12W::VALUE13 => 20,
            PC12W::VALUE14 => 24,
            PC12W::VALUE15 => 25,
            PC12W::VALUE16 => 26,
            PC12W::VALUE17 => 27,
            PC12W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC12W<'a> {
    w: &'a mut W,
}
impl<'a> _PC12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC12W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC12W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC12W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC12W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC12W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC12W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC12W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC12W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC12W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC12W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC12W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC12W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC12W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC12W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC12W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC12W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC12W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC12W::VALUE18)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC13`"]
pub enum PC13W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl PC13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC13W::VALUE1 => 0,
            PC13W::VALUE2 => 1,
            PC13W::VALUE3 => 2,
            PC13W::VALUE4 => 3,
            PC13W::VALUE5 => 4,
            PC13W::VALUE6 => 5,
            PC13W::VALUE7 => 6,
            PC13W::VALUE8 => 7,
            PC13W::VALUE9 => 16,
            PC13W::VALUE10 => 17,
            PC13W::VALUE11 => 18,
            PC13W::VALUE12 => 19,
            PC13W::VALUE13 => 20,
            PC13W::VALUE14 => 24,
            PC13W::VALUE15 => 25,
            PC13W::VALUE16 => 26,
            PC13W::VALUE17 => 27,
            PC13W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC13W<'a> {
    w: &'a mut W,
}
impl<'a> _PC13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC13W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC13W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC13W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC13W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC13W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC13W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC13W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC13W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC13W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC13W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC13W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC13W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC13W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC13W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC13W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC13W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC13W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC13W::VALUE18)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC14`"]
pub enum PC14W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl PC14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC14W::VALUE1 => 0,
            PC14W::VALUE2 => 1,
            PC14W::VALUE3 => 2,
            PC14W::VALUE4 => 3,
            PC14W::VALUE5 => 4,
            PC14W::VALUE6 => 5,
            PC14W::VALUE7 => 6,
            PC14W::VALUE8 => 7,
            PC14W::VALUE9 => 16,
            PC14W::VALUE10 => 17,
            PC14W::VALUE11 => 18,
            PC14W::VALUE12 => 19,
            PC14W::VALUE13 => 20,
            PC14W::VALUE14 => 24,
            PC14W::VALUE15 => 25,
            PC14W::VALUE16 => 26,
            PC14W::VALUE17 => 27,
            PC14W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC14W<'a> {
    w: &'a mut W,
}
impl<'a> _PC14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC14W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC14W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC14W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC14W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC14W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC14W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC14W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC14W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC14W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC14W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC14W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC14W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC14W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC14W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC14W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC14W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC14W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC14W::VALUE18)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PC15`"]
pub enum PC15W {
    #[doc = "Input - No internal pull device active"]
    VALUE1,
    #[doc = "Input - Internal pull-down device active"]
    VALUE2,
    #[doc = "Input - Internal pull-up device active"]
    VALUE3,
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    VALUE4,
    #[doc = "Input inverted - No internal pull device active"]
    VALUE5,
    #[doc = "Input inverted - Internal pull-down device active"]
    VALUE6,
    #[doc = "Input inverted - Internal pull-up device active"]
    VALUE7,
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    VALUE8,
    #[doc = "Output Push-Pull - General-purpose output"]
    VALUE9,
    #[doc = "Output Push-Pull - Alternate output function 1"]
    VALUE10,
    #[doc = "Output Push-Pull - Alternate output function 2"]
    VALUE11,
    #[doc = "Output Push-Pull - Alternate output function 3"]
    VALUE12,
    #[doc = "Output Push-Pull - Alternate output function 4"]
    VALUE13,
    #[doc = "Output Open Drain - General-purpose output"]
    VALUE14,
    #[doc = "Output Open Drain - Alternate output function 1"]
    VALUE15,
    #[doc = "Output Open Drain - Alternate output function 2"]
    VALUE16,
    #[doc = "Output Open Drain - Alternate output function 3"]
    VALUE17,
    #[doc = "Output Open Drain - Alternate output function 4"]
    VALUE18,
}
impl PC15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC15W::VALUE1 => 0,
            PC15W::VALUE2 => 1,
            PC15W::VALUE3 => 2,
            PC15W::VALUE4 => 3,
            PC15W::VALUE5 => 4,
            PC15W::VALUE6 => 5,
            PC15W::VALUE7 => 6,
            PC15W::VALUE8 => 7,
            PC15W::VALUE9 => 16,
            PC15W::VALUE10 => 17,
            PC15W::VALUE11 => 18,
            PC15W::VALUE12 => 19,
            PC15W::VALUE13 => 20,
            PC15W::VALUE14 => 24,
            PC15W::VALUE15 => 25,
            PC15W::VALUE16 => 26,
            PC15W::VALUE17 => 27,
            PC15W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC15W<'a> {
    w: &'a mut W,
}
impl<'a> _PC15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC15W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC15W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC15W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC15W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC15W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC15W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC15W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC15W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC15W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC15W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC15W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC15W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC15W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC15W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC15W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC15W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC15W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC15W::VALUE18)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc12(&self) -> PC12R {
        PC12R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc13(&self) -> PC13R {
        PC13R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc14(&self) -> PC14R {
        PC14R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc15(&self) -> PC15R {
        PC15R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:7 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc12(&mut self) -> _PC12W {
        _PC12W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc13(&mut self) -> _PC13W {
        _PC13W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc14(&mut self) -> _PC14W {
        _PC14W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 12 to 15"]
    #[inline]
    pub fn pc15(&mut self) -> _PC15W {
        _PC15W { w: self }
    }
}
