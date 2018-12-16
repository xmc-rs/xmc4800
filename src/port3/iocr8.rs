#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCR8 {
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
#[doc = "Possible values of the field `PC8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC8R {
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
impl PC8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC8R::VALUE1 => 0,
            PC8R::VALUE2 => 1,
            PC8R::VALUE3 => 2,
            PC8R::VALUE4 => 3,
            PC8R::VALUE5 => 4,
            PC8R::VALUE6 => 5,
            PC8R::VALUE7 => 6,
            PC8R::VALUE8 => 7,
            PC8R::VALUE9 => 16,
            PC8R::VALUE10 => 17,
            PC8R::VALUE11 => 18,
            PC8R::VALUE12 => 19,
            PC8R::VALUE13 => 20,
            PC8R::VALUE14 => 24,
            PC8R::VALUE15 => 25,
            PC8R::VALUE16 => 26,
            PC8R::VALUE17 => 27,
            PC8R::VALUE18 => 28,
            PC8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC8R {
        match value {
            0 => PC8R::VALUE1,
            1 => PC8R::VALUE2,
            2 => PC8R::VALUE3,
            3 => PC8R::VALUE4,
            4 => PC8R::VALUE5,
            5 => PC8R::VALUE6,
            6 => PC8R::VALUE7,
            7 => PC8R::VALUE8,
            16 => PC8R::VALUE9,
            17 => PC8R::VALUE10,
            18 => PC8R::VALUE11,
            19 => PC8R::VALUE12,
            20 => PC8R::VALUE13,
            24 => PC8R::VALUE14,
            25 => PC8R::VALUE15,
            26 => PC8R::VALUE16,
            27 => PC8R::VALUE17,
            28 => PC8R::VALUE18,
            i => PC8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC8R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC8R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC8R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC8R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC8R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC8R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC8R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC8R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC8R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC8R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC8R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC8R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC8R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC8R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC8R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC8R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC8R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC8R::VALUE18
    }
}
#[doc = "Possible values of the field `PC9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC9R {
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
impl PC9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC9R::VALUE1 => 0,
            PC9R::VALUE2 => 1,
            PC9R::VALUE3 => 2,
            PC9R::VALUE4 => 3,
            PC9R::VALUE5 => 4,
            PC9R::VALUE6 => 5,
            PC9R::VALUE7 => 6,
            PC9R::VALUE8 => 7,
            PC9R::VALUE9 => 16,
            PC9R::VALUE10 => 17,
            PC9R::VALUE11 => 18,
            PC9R::VALUE12 => 19,
            PC9R::VALUE13 => 20,
            PC9R::VALUE14 => 24,
            PC9R::VALUE15 => 25,
            PC9R::VALUE16 => 26,
            PC9R::VALUE17 => 27,
            PC9R::VALUE18 => 28,
            PC9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC9R {
        match value {
            0 => PC9R::VALUE1,
            1 => PC9R::VALUE2,
            2 => PC9R::VALUE3,
            3 => PC9R::VALUE4,
            4 => PC9R::VALUE5,
            5 => PC9R::VALUE6,
            6 => PC9R::VALUE7,
            7 => PC9R::VALUE8,
            16 => PC9R::VALUE9,
            17 => PC9R::VALUE10,
            18 => PC9R::VALUE11,
            19 => PC9R::VALUE12,
            20 => PC9R::VALUE13,
            24 => PC9R::VALUE14,
            25 => PC9R::VALUE15,
            26 => PC9R::VALUE16,
            27 => PC9R::VALUE17,
            28 => PC9R::VALUE18,
            i => PC9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC9R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC9R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC9R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC9R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC9R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC9R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC9R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC9R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC9R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC9R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC9R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC9R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC9R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC9R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC9R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC9R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC9R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC9R::VALUE18
    }
}
#[doc = "Possible values of the field `PC10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC10R {
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
impl PC10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC10R::VALUE1 => 0,
            PC10R::VALUE2 => 1,
            PC10R::VALUE3 => 2,
            PC10R::VALUE4 => 3,
            PC10R::VALUE5 => 4,
            PC10R::VALUE6 => 5,
            PC10R::VALUE7 => 6,
            PC10R::VALUE8 => 7,
            PC10R::VALUE9 => 16,
            PC10R::VALUE10 => 17,
            PC10R::VALUE11 => 18,
            PC10R::VALUE12 => 19,
            PC10R::VALUE13 => 20,
            PC10R::VALUE14 => 24,
            PC10R::VALUE15 => 25,
            PC10R::VALUE16 => 26,
            PC10R::VALUE17 => 27,
            PC10R::VALUE18 => 28,
            PC10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC10R {
        match value {
            0 => PC10R::VALUE1,
            1 => PC10R::VALUE2,
            2 => PC10R::VALUE3,
            3 => PC10R::VALUE4,
            4 => PC10R::VALUE5,
            5 => PC10R::VALUE6,
            6 => PC10R::VALUE7,
            7 => PC10R::VALUE8,
            16 => PC10R::VALUE9,
            17 => PC10R::VALUE10,
            18 => PC10R::VALUE11,
            19 => PC10R::VALUE12,
            20 => PC10R::VALUE13,
            24 => PC10R::VALUE14,
            25 => PC10R::VALUE15,
            26 => PC10R::VALUE16,
            27 => PC10R::VALUE17,
            28 => PC10R::VALUE18,
            i => PC10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC10R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC10R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC10R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC10R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC10R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC10R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC10R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC10R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC10R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC10R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC10R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC10R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC10R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC10R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC10R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC10R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC10R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC10R::VALUE18
    }
}
#[doc = "Possible values of the field `PC11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC11R {
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
impl PC11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC11R::VALUE1 => 0,
            PC11R::VALUE2 => 1,
            PC11R::VALUE3 => 2,
            PC11R::VALUE4 => 3,
            PC11R::VALUE5 => 4,
            PC11R::VALUE6 => 5,
            PC11R::VALUE7 => 6,
            PC11R::VALUE8 => 7,
            PC11R::VALUE9 => 16,
            PC11R::VALUE10 => 17,
            PC11R::VALUE11 => 18,
            PC11R::VALUE12 => 19,
            PC11R::VALUE13 => 20,
            PC11R::VALUE14 => 24,
            PC11R::VALUE15 => 25,
            PC11R::VALUE16 => 26,
            PC11R::VALUE17 => 27,
            PC11R::VALUE18 => 28,
            PC11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC11R {
        match value {
            0 => PC11R::VALUE1,
            1 => PC11R::VALUE2,
            2 => PC11R::VALUE3,
            3 => PC11R::VALUE4,
            4 => PC11R::VALUE5,
            5 => PC11R::VALUE6,
            6 => PC11R::VALUE7,
            7 => PC11R::VALUE8,
            16 => PC11R::VALUE9,
            17 => PC11R::VALUE10,
            18 => PC11R::VALUE11,
            19 => PC11R::VALUE12,
            20 => PC11R::VALUE13,
            24 => PC11R::VALUE14,
            25 => PC11R::VALUE15,
            26 => PC11R::VALUE16,
            27 => PC11R::VALUE17,
            28 => PC11R::VALUE18,
            i => PC11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC11R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC11R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC11R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC11R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC11R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC11R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC11R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC11R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC11R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC11R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC11R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC11R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC11R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC11R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC11R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC11R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC11R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC11R::VALUE18
    }
}
#[doc = "Values that can be written to the field `PC8`"]
pub enum PC8W {
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
impl PC8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC8W::VALUE1 => 0,
            PC8W::VALUE2 => 1,
            PC8W::VALUE3 => 2,
            PC8W::VALUE4 => 3,
            PC8W::VALUE5 => 4,
            PC8W::VALUE6 => 5,
            PC8W::VALUE7 => 6,
            PC8W::VALUE8 => 7,
            PC8W::VALUE9 => 16,
            PC8W::VALUE10 => 17,
            PC8W::VALUE11 => 18,
            PC8W::VALUE12 => 19,
            PC8W::VALUE13 => 20,
            PC8W::VALUE14 => 24,
            PC8W::VALUE15 => 25,
            PC8W::VALUE16 => 26,
            PC8W::VALUE17 => 27,
            PC8W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC8W<'a> {
    w: &'a mut W,
}
impl<'a> _PC8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC8W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC8W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC8W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC8W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC8W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC8W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC8W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC8W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC8W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC8W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC8W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC8W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC8W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC8W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC8W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC8W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC8W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC8W::VALUE18)
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
#[doc = "Values that can be written to the field `PC9`"]
pub enum PC9W {
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
impl PC9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC9W::VALUE1 => 0,
            PC9W::VALUE2 => 1,
            PC9W::VALUE3 => 2,
            PC9W::VALUE4 => 3,
            PC9W::VALUE5 => 4,
            PC9W::VALUE6 => 5,
            PC9W::VALUE7 => 6,
            PC9W::VALUE8 => 7,
            PC9W::VALUE9 => 16,
            PC9W::VALUE10 => 17,
            PC9W::VALUE11 => 18,
            PC9W::VALUE12 => 19,
            PC9W::VALUE13 => 20,
            PC9W::VALUE14 => 24,
            PC9W::VALUE15 => 25,
            PC9W::VALUE16 => 26,
            PC9W::VALUE17 => 27,
            PC9W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC9W<'a> {
    w: &'a mut W,
}
impl<'a> _PC9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC9W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC9W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC9W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC9W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC9W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC9W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC9W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC9W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC9W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC9W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC9W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC9W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC9W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC9W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC9W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC9W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC9W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC9W::VALUE18)
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
#[doc = "Values that can be written to the field `PC10`"]
pub enum PC10W {
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
impl PC10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC10W::VALUE1 => 0,
            PC10W::VALUE2 => 1,
            PC10W::VALUE3 => 2,
            PC10W::VALUE4 => 3,
            PC10W::VALUE5 => 4,
            PC10W::VALUE6 => 5,
            PC10W::VALUE7 => 6,
            PC10W::VALUE8 => 7,
            PC10W::VALUE9 => 16,
            PC10W::VALUE10 => 17,
            PC10W::VALUE11 => 18,
            PC10W::VALUE12 => 19,
            PC10W::VALUE13 => 20,
            PC10W::VALUE14 => 24,
            PC10W::VALUE15 => 25,
            PC10W::VALUE16 => 26,
            PC10W::VALUE17 => 27,
            PC10W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC10W<'a> {
    w: &'a mut W,
}
impl<'a> _PC10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC10W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC10W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC10W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC10W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC10W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC10W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC10W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC10W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC10W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC10W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC10W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC10W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC10W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC10W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC10W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC10W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC10W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC10W::VALUE18)
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
#[doc = "Values that can be written to the field `PC11`"]
pub enum PC11W {
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
impl PC11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC11W::VALUE1 => 0,
            PC11W::VALUE2 => 1,
            PC11W::VALUE3 => 2,
            PC11W::VALUE4 => 3,
            PC11W::VALUE5 => 4,
            PC11W::VALUE6 => 5,
            PC11W::VALUE7 => 6,
            PC11W::VALUE8 => 7,
            PC11W::VALUE9 => 16,
            PC11W::VALUE10 => 17,
            PC11W::VALUE11 => 18,
            PC11W::VALUE12 => 19,
            PC11W::VALUE13 => 20,
            PC11W::VALUE14 => 24,
            PC11W::VALUE15 => 25,
            PC11W::VALUE16 => 26,
            PC11W::VALUE17 => 27,
            PC11W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC11W<'a> {
    w: &'a mut W,
}
impl<'a> _PC11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC11W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC11W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC11W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC11W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC11W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC11W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC11W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC11W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC11W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC11W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC11W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC11W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC11W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC11W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC11W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC11W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC11W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC11W::VALUE18)
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc8(&self) -> PC8R {
        PC8R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc9(&self) -> PC9R {
        PC9R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc10(&self) -> PC10R {
        PC10R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc11(&self) -> PC11R {
        PC11R::_from({
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc8(&mut self) -> _PC8W {
        _PC8W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc9(&mut self) -> _PC9W {
        _PC9W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc10(&mut self) -> _PC10W {
        _PC10W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 8 to 11"]
    #[inline]
    pub fn pc11(&mut self) -> _PC11W {
        _PC11W { w: self }
    }
}
