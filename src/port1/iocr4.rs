#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCR4 {
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
#[doc = "Possible values of the field `PC4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC4R {
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
impl PC4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC4R::VALUE1 => 0,
            PC4R::VALUE2 => 1,
            PC4R::VALUE3 => 2,
            PC4R::VALUE4 => 3,
            PC4R::VALUE5 => 4,
            PC4R::VALUE6 => 5,
            PC4R::VALUE7 => 6,
            PC4R::VALUE8 => 7,
            PC4R::VALUE9 => 16,
            PC4R::VALUE10 => 17,
            PC4R::VALUE11 => 18,
            PC4R::VALUE12 => 19,
            PC4R::VALUE13 => 20,
            PC4R::VALUE14 => 24,
            PC4R::VALUE15 => 25,
            PC4R::VALUE16 => 26,
            PC4R::VALUE17 => 27,
            PC4R::VALUE18 => 28,
            PC4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC4R {
        match value {
            0 => PC4R::VALUE1,
            1 => PC4R::VALUE2,
            2 => PC4R::VALUE3,
            3 => PC4R::VALUE4,
            4 => PC4R::VALUE5,
            5 => PC4R::VALUE6,
            6 => PC4R::VALUE7,
            7 => PC4R::VALUE8,
            16 => PC4R::VALUE9,
            17 => PC4R::VALUE10,
            18 => PC4R::VALUE11,
            19 => PC4R::VALUE12,
            20 => PC4R::VALUE13,
            24 => PC4R::VALUE14,
            25 => PC4R::VALUE15,
            26 => PC4R::VALUE16,
            27 => PC4R::VALUE17,
            28 => PC4R::VALUE18,
            i => PC4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC4R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC4R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC4R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC4R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC4R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC4R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC4R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC4R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC4R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC4R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC4R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC4R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC4R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC4R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC4R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC4R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC4R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC4R::VALUE18
    }
}
#[doc = "Possible values of the field `PC5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC5R {
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
impl PC5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC5R::VALUE1 => 0,
            PC5R::VALUE2 => 1,
            PC5R::VALUE3 => 2,
            PC5R::VALUE4 => 3,
            PC5R::VALUE5 => 4,
            PC5R::VALUE6 => 5,
            PC5R::VALUE7 => 6,
            PC5R::VALUE8 => 7,
            PC5R::VALUE9 => 16,
            PC5R::VALUE10 => 17,
            PC5R::VALUE11 => 18,
            PC5R::VALUE12 => 19,
            PC5R::VALUE13 => 20,
            PC5R::VALUE14 => 24,
            PC5R::VALUE15 => 25,
            PC5R::VALUE16 => 26,
            PC5R::VALUE17 => 27,
            PC5R::VALUE18 => 28,
            PC5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC5R {
        match value {
            0 => PC5R::VALUE1,
            1 => PC5R::VALUE2,
            2 => PC5R::VALUE3,
            3 => PC5R::VALUE4,
            4 => PC5R::VALUE5,
            5 => PC5R::VALUE6,
            6 => PC5R::VALUE7,
            7 => PC5R::VALUE8,
            16 => PC5R::VALUE9,
            17 => PC5R::VALUE10,
            18 => PC5R::VALUE11,
            19 => PC5R::VALUE12,
            20 => PC5R::VALUE13,
            24 => PC5R::VALUE14,
            25 => PC5R::VALUE15,
            26 => PC5R::VALUE16,
            27 => PC5R::VALUE17,
            28 => PC5R::VALUE18,
            i => PC5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC5R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC5R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC5R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC5R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC5R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC5R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC5R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC5R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC5R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC5R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC5R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC5R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC5R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC5R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC5R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC5R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC5R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC5R::VALUE18
    }
}
#[doc = "Possible values of the field `PC6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC6R {
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
impl PC6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC6R::VALUE1 => 0,
            PC6R::VALUE2 => 1,
            PC6R::VALUE3 => 2,
            PC6R::VALUE4 => 3,
            PC6R::VALUE5 => 4,
            PC6R::VALUE6 => 5,
            PC6R::VALUE7 => 6,
            PC6R::VALUE8 => 7,
            PC6R::VALUE9 => 16,
            PC6R::VALUE10 => 17,
            PC6R::VALUE11 => 18,
            PC6R::VALUE12 => 19,
            PC6R::VALUE13 => 20,
            PC6R::VALUE14 => 24,
            PC6R::VALUE15 => 25,
            PC6R::VALUE16 => 26,
            PC6R::VALUE17 => 27,
            PC6R::VALUE18 => 28,
            PC6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC6R {
        match value {
            0 => PC6R::VALUE1,
            1 => PC6R::VALUE2,
            2 => PC6R::VALUE3,
            3 => PC6R::VALUE4,
            4 => PC6R::VALUE5,
            5 => PC6R::VALUE6,
            6 => PC6R::VALUE7,
            7 => PC6R::VALUE8,
            16 => PC6R::VALUE9,
            17 => PC6R::VALUE10,
            18 => PC6R::VALUE11,
            19 => PC6R::VALUE12,
            20 => PC6R::VALUE13,
            24 => PC6R::VALUE14,
            25 => PC6R::VALUE15,
            26 => PC6R::VALUE16,
            27 => PC6R::VALUE17,
            28 => PC6R::VALUE18,
            i => PC6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC6R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC6R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC6R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC6R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC6R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC6R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC6R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC6R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC6R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC6R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC6R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC6R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC6R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC6R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC6R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC6R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC6R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC6R::VALUE18
    }
}
#[doc = "Possible values of the field `PC7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC7R {
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
impl PC7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC7R::VALUE1 => 0,
            PC7R::VALUE2 => 1,
            PC7R::VALUE3 => 2,
            PC7R::VALUE4 => 3,
            PC7R::VALUE5 => 4,
            PC7R::VALUE6 => 5,
            PC7R::VALUE7 => 6,
            PC7R::VALUE8 => 7,
            PC7R::VALUE9 => 16,
            PC7R::VALUE10 => 17,
            PC7R::VALUE11 => 18,
            PC7R::VALUE12 => 19,
            PC7R::VALUE13 => 20,
            PC7R::VALUE14 => 24,
            PC7R::VALUE15 => 25,
            PC7R::VALUE16 => 26,
            PC7R::VALUE17 => 27,
            PC7R::VALUE18 => 28,
            PC7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC7R {
        match value {
            0 => PC7R::VALUE1,
            1 => PC7R::VALUE2,
            2 => PC7R::VALUE3,
            3 => PC7R::VALUE4,
            4 => PC7R::VALUE5,
            5 => PC7R::VALUE6,
            6 => PC7R::VALUE7,
            7 => PC7R::VALUE8,
            16 => PC7R::VALUE9,
            17 => PC7R::VALUE10,
            18 => PC7R::VALUE11,
            19 => PC7R::VALUE12,
            20 => PC7R::VALUE13,
            24 => PC7R::VALUE14,
            25 => PC7R::VALUE15,
            26 => PC7R::VALUE16,
            27 => PC7R::VALUE17,
            28 => PC7R::VALUE18,
            i => PC7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC7R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC7R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC7R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC7R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC7R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC7R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC7R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC7R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC7R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC7R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC7R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC7R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC7R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC7R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC7R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC7R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC7R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC7R::VALUE18
    }
}
#[doc = "Values that can be written to the field `PC4`"]
pub enum PC4W {
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
impl PC4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC4W::VALUE1 => 0,
            PC4W::VALUE2 => 1,
            PC4W::VALUE3 => 2,
            PC4W::VALUE4 => 3,
            PC4W::VALUE5 => 4,
            PC4W::VALUE6 => 5,
            PC4W::VALUE7 => 6,
            PC4W::VALUE8 => 7,
            PC4W::VALUE9 => 16,
            PC4W::VALUE10 => 17,
            PC4W::VALUE11 => 18,
            PC4W::VALUE12 => 19,
            PC4W::VALUE13 => 20,
            PC4W::VALUE14 => 24,
            PC4W::VALUE15 => 25,
            PC4W::VALUE16 => 26,
            PC4W::VALUE17 => 27,
            PC4W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC4W<'a> {
    w: &'a mut W,
}
impl<'a> _PC4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC4W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC4W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC4W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC4W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC4W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC4W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC4W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC4W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC4W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC4W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC4W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC4W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC4W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC4W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC4W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC4W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC4W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC4W::VALUE18)
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
#[doc = "Values that can be written to the field `PC5`"]
pub enum PC5W {
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
impl PC5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC5W::VALUE1 => 0,
            PC5W::VALUE2 => 1,
            PC5W::VALUE3 => 2,
            PC5W::VALUE4 => 3,
            PC5W::VALUE5 => 4,
            PC5W::VALUE6 => 5,
            PC5W::VALUE7 => 6,
            PC5W::VALUE8 => 7,
            PC5W::VALUE9 => 16,
            PC5W::VALUE10 => 17,
            PC5W::VALUE11 => 18,
            PC5W::VALUE12 => 19,
            PC5W::VALUE13 => 20,
            PC5W::VALUE14 => 24,
            PC5W::VALUE15 => 25,
            PC5W::VALUE16 => 26,
            PC5W::VALUE17 => 27,
            PC5W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC5W<'a> {
    w: &'a mut W,
}
impl<'a> _PC5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC5W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC5W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC5W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC5W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC5W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC5W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC5W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC5W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC5W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC5W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC5W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC5W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC5W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC5W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC5W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC5W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC5W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC5W::VALUE18)
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
#[doc = "Values that can be written to the field `PC6`"]
pub enum PC6W {
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
impl PC6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC6W::VALUE1 => 0,
            PC6W::VALUE2 => 1,
            PC6W::VALUE3 => 2,
            PC6W::VALUE4 => 3,
            PC6W::VALUE5 => 4,
            PC6W::VALUE6 => 5,
            PC6W::VALUE7 => 6,
            PC6W::VALUE8 => 7,
            PC6W::VALUE9 => 16,
            PC6W::VALUE10 => 17,
            PC6W::VALUE11 => 18,
            PC6W::VALUE12 => 19,
            PC6W::VALUE13 => 20,
            PC6W::VALUE14 => 24,
            PC6W::VALUE15 => 25,
            PC6W::VALUE16 => 26,
            PC6W::VALUE17 => 27,
            PC6W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC6W<'a> {
    w: &'a mut W,
}
impl<'a> _PC6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC6W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC6W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC6W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC6W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC6W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC6W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC6W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC6W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC6W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC6W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC6W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC6W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC6W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC6W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC6W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC6W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC6W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC6W::VALUE18)
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
#[doc = "Values that can be written to the field `PC7`"]
pub enum PC7W {
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
impl PC7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC7W::VALUE1 => 0,
            PC7W::VALUE2 => 1,
            PC7W::VALUE3 => 2,
            PC7W::VALUE4 => 3,
            PC7W::VALUE5 => 4,
            PC7W::VALUE6 => 5,
            PC7W::VALUE7 => 6,
            PC7W::VALUE8 => 7,
            PC7W::VALUE9 => 16,
            PC7W::VALUE10 => 17,
            PC7W::VALUE11 => 18,
            PC7W::VALUE12 => 19,
            PC7W::VALUE13 => 20,
            PC7W::VALUE14 => 24,
            PC7W::VALUE15 => 25,
            PC7W::VALUE16 => 26,
            PC7W::VALUE17 => 27,
            PC7W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC7W<'a> {
    w: &'a mut W,
}
impl<'a> _PC7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC7W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC7W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC7W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC7W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC7W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC7W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC7W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC7W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC7W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC7W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC7W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC7W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC7W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC7W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC7W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC7W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC7W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC7W::VALUE18)
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc4(&self) -> PC4R {
        PC4R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc5(&self) -> PC5R {
        PC5R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc6(&self) -> PC6R {
        PC6R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc7(&self) -> PC7R {
        PC7R::_from({
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc4(&mut self) -> _PC4W {
        _PC4W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc5(&mut self) -> _PC5W {
        _PC5W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc6(&mut self) -> _PC6W {
        _PC6W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 4 to 7"]
    #[inline]
    pub fn pc7(&mut self) -> _PC7W {
        _PC7W { w: self }
    }
}
