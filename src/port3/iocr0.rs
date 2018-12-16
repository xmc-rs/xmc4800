#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCR0 {
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
#[doc = "Possible values of the field `PC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC0R {
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
impl PC0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC0R::VALUE1 => 0,
            PC0R::VALUE2 => 1,
            PC0R::VALUE3 => 2,
            PC0R::VALUE4 => 3,
            PC0R::VALUE5 => 4,
            PC0R::VALUE6 => 5,
            PC0R::VALUE7 => 6,
            PC0R::VALUE8 => 7,
            PC0R::VALUE9 => 16,
            PC0R::VALUE10 => 17,
            PC0R::VALUE11 => 18,
            PC0R::VALUE12 => 19,
            PC0R::VALUE13 => 20,
            PC0R::VALUE14 => 24,
            PC0R::VALUE15 => 25,
            PC0R::VALUE16 => 26,
            PC0R::VALUE17 => 27,
            PC0R::VALUE18 => 28,
            PC0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC0R {
        match value {
            0 => PC0R::VALUE1,
            1 => PC0R::VALUE2,
            2 => PC0R::VALUE3,
            3 => PC0R::VALUE4,
            4 => PC0R::VALUE5,
            5 => PC0R::VALUE6,
            6 => PC0R::VALUE7,
            7 => PC0R::VALUE8,
            16 => PC0R::VALUE9,
            17 => PC0R::VALUE10,
            18 => PC0R::VALUE11,
            19 => PC0R::VALUE12,
            20 => PC0R::VALUE13,
            24 => PC0R::VALUE14,
            25 => PC0R::VALUE15,
            26 => PC0R::VALUE16,
            27 => PC0R::VALUE17,
            28 => PC0R::VALUE18,
            i => PC0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC0R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC0R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC0R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC0R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC0R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC0R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC0R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC0R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC0R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC0R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC0R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC0R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC0R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC0R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC0R::VALUE18
    }
}
#[doc = "Possible values of the field `PC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC1R {
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
impl PC1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC1R::VALUE1 => 0,
            PC1R::VALUE2 => 1,
            PC1R::VALUE3 => 2,
            PC1R::VALUE4 => 3,
            PC1R::VALUE5 => 4,
            PC1R::VALUE6 => 5,
            PC1R::VALUE7 => 6,
            PC1R::VALUE8 => 7,
            PC1R::VALUE9 => 16,
            PC1R::VALUE10 => 17,
            PC1R::VALUE11 => 18,
            PC1R::VALUE12 => 19,
            PC1R::VALUE13 => 20,
            PC1R::VALUE14 => 24,
            PC1R::VALUE15 => 25,
            PC1R::VALUE16 => 26,
            PC1R::VALUE17 => 27,
            PC1R::VALUE18 => 28,
            PC1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC1R {
        match value {
            0 => PC1R::VALUE1,
            1 => PC1R::VALUE2,
            2 => PC1R::VALUE3,
            3 => PC1R::VALUE4,
            4 => PC1R::VALUE5,
            5 => PC1R::VALUE6,
            6 => PC1R::VALUE7,
            7 => PC1R::VALUE8,
            16 => PC1R::VALUE9,
            17 => PC1R::VALUE10,
            18 => PC1R::VALUE11,
            19 => PC1R::VALUE12,
            20 => PC1R::VALUE13,
            24 => PC1R::VALUE14,
            25 => PC1R::VALUE15,
            26 => PC1R::VALUE16,
            27 => PC1R::VALUE17,
            28 => PC1R::VALUE18,
            i => PC1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC1R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC1R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC1R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC1R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC1R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC1R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC1R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC1R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC1R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC1R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC1R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC1R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC1R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC1R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC1R::VALUE18
    }
}
#[doc = "Possible values of the field `PC2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC2R {
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
impl PC2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC2R::VALUE1 => 0,
            PC2R::VALUE2 => 1,
            PC2R::VALUE3 => 2,
            PC2R::VALUE4 => 3,
            PC2R::VALUE5 => 4,
            PC2R::VALUE6 => 5,
            PC2R::VALUE7 => 6,
            PC2R::VALUE8 => 7,
            PC2R::VALUE9 => 16,
            PC2R::VALUE10 => 17,
            PC2R::VALUE11 => 18,
            PC2R::VALUE12 => 19,
            PC2R::VALUE13 => 20,
            PC2R::VALUE14 => 24,
            PC2R::VALUE15 => 25,
            PC2R::VALUE16 => 26,
            PC2R::VALUE17 => 27,
            PC2R::VALUE18 => 28,
            PC2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC2R {
        match value {
            0 => PC2R::VALUE1,
            1 => PC2R::VALUE2,
            2 => PC2R::VALUE3,
            3 => PC2R::VALUE4,
            4 => PC2R::VALUE5,
            5 => PC2R::VALUE6,
            6 => PC2R::VALUE7,
            7 => PC2R::VALUE8,
            16 => PC2R::VALUE9,
            17 => PC2R::VALUE10,
            18 => PC2R::VALUE11,
            19 => PC2R::VALUE12,
            20 => PC2R::VALUE13,
            24 => PC2R::VALUE14,
            25 => PC2R::VALUE15,
            26 => PC2R::VALUE16,
            27 => PC2R::VALUE17,
            28 => PC2R::VALUE18,
            i => PC2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC2R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC2R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC2R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC2R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC2R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC2R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC2R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC2R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC2R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC2R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC2R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC2R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC2R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC2R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC2R::VALUE18
    }
}
#[doc = "Possible values of the field `PC3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC3R {
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
impl PC3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PC3R::VALUE1 => 0,
            PC3R::VALUE2 => 1,
            PC3R::VALUE3 => 2,
            PC3R::VALUE4 => 3,
            PC3R::VALUE5 => 4,
            PC3R::VALUE6 => 5,
            PC3R::VALUE7 => 6,
            PC3R::VALUE8 => 7,
            PC3R::VALUE9 => 16,
            PC3R::VALUE10 => 17,
            PC3R::VALUE11 => 18,
            PC3R::VALUE12 => 19,
            PC3R::VALUE13 => 20,
            PC3R::VALUE14 => 24,
            PC3R::VALUE15 => 25,
            PC3R::VALUE16 => 26,
            PC3R::VALUE17 => 27,
            PC3R::VALUE18 => 28,
            PC3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PC3R {
        match value {
            0 => PC3R::VALUE1,
            1 => PC3R::VALUE2,
            2 => PC3R::VALUE3,
            3 => PC3R::VALUE4,
            4 => PC3R::VALUE5,
            5 => PC3R::VALUE6,
            6 => PC3R::VALUE7,
            7 => PC3R::VALUE8,
            16 => PC3R::VALUE9,
            17 => PC3R::VALUE10,
            18 => PC3R::VALUE11,
            19 => PC3R::VALUE12,
            20 => PC3R::VALUE13,
            24 => PC3R::VALUE14,
            25 => PC3R::VALUE15,
            26 => PC3R::VALUE16,
            27 => PC3R::VALUE17,
            28 => PC3R::VALUE18,
            i => PC3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PC3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PC3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PC3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PC3R::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == PC3R::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == PC3R::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == PC3R::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == PC3R::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline]
    pub fn is_value9(&self) -> bool {
        *self == PC3R::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline]
    pub fn is_value10(&self) -> bool {
        *self == PC3R::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline]
    pub fn is_value11(&self) -> bool {
        *self == PC3R::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline]
    pub fn is_value12(&self) -> bool {
        *self == PC3R::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline]
    pub fn is_value13(&self) -> bool {
        *self == PC3R::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline]
    pub fn is_value14(&self) -> bool {
        *self == PC3R::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline]
    pub fn is_value15(&self) -> bool {
        *self == PC3R::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline]
    pub fn is_value16(&self) -> bool {
        *self == PC3R::VALUE16
    }
    #[doc = "Checks if the value of the field is `VALUE17`"]
    #[inline]
    pub fn is_value17(&self) -> bool {
        *self == PC3R::VALUE17
    }
    #[doc = "Checks if the value of the field is `VALUE18`"]
    #[inline]
    pub fn is_value18(&self) -> bool {
        *self == PC3R::VALUE18
    }
}
#[doc = "Values that can be written to the field `PC0`"]
pub enum PC0W {
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
impl PC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC0W::VALUE1 => 0,
            PC0W::VALUE2 => 1,
            PC0W::VALUE3 => 2,
            PC0W::VALUE4 => 3,
            PC0W::VALUE5 => 4,
            PC0W::VALUE6 => 5,
            PC0W::VALUE7 => 6,
            PC0W::VALUE8 => 7,
            PC0W::VALUE9 => 16,
            PC0W::VALUE10 => 17,
            PC0W::VALUE11 => 18,
            PC0W::VALUE12 => 19,
            PC0W::VALUE13 => 20,
            PC0W::VALUE14 => 24,
            PC0W::VALUE15 => 25,
            PC0W::VALUE16 => 26,
            PC0W::VALUE17 => 27,
            PC0W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC0W<'a> {
    w: &'a mut W,
}
impl<'a> _PC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC0W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC0W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC0W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC0W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC0W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC0W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC0W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC0W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC0W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC0W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC0W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC0W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC0W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC0W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC0W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC0W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC0W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC0W::VALUE18)
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
#[doc = "Values that can be written to the field `PC1`"]
pub enum PC1W {
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
impl PC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC1W::VALUE1 => 0,
            PC1W::VALUE2 => 1,
            PC1W::VALUE3 => 2,
            PC1W::VALUE4 => 3,
            PC1W::VALUE5 => 4,
            PC1W::VALUE6 => 5,
            PC1W::VALUE7 => 6,
            PC1W::VALUE8 => 7,
            PC1W::VALUE9 => 16,
            PC1W::VALUE10 => 17,
            PC1W::VALUE11 => 18,
            PC1W::VALUE12 => 19,
            PC1W::VALUE13 => 20,
            PC1W::VALUE14 => 24,
            PC1W::VALUE15 => 25,
            PC1W::VALUE16 => 26,
            PC1W::VALUE17 => 27,
            PC1W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC1W<'a> {
    w: &'a mut W,
}
impl<'a> _PC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC1W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC1W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC1W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC1W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC1W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC1W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC1W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC1W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC1W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC1W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC1W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC1W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC1W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC1W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC1W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC1W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC1W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC1W::VALUE18)
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
#[doc = "Values that can be written to the field `PC2`"]
pub enum PC2W {
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
impl PC2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC2W::VALUE1 => 0,
            PC2W::VALUE2 => 1,
            PC2W::VALUE3 => 2,
            PC2W::VALUE4 => 3,
            PC2W::VALUE5 => 4,
            PC2W::VALUE6 => 5,
            PC2W::VALUE7 => 6,
            PC2W::VALUE8 => 7,
            PC2W::VALUE9 => 16,
            PC2W::VALUE10 => 17,
            PC2W::VALUE11 => 18,
            PC2W::VALUE12 => 19,
            PC2W::VALUE13 => 20,
            PC2W::VALUE14 => 24,
            PC2W::VALUE15 => 25,
            PC2W::VALUE16 => 26,
            PC2W::VALUE17 => 27,
            PC2W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC2W<'a> {
    w: &'a mut W,
}
impl<'a> _PC2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC2W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC2W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC2W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC2W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC2W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC2W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC2W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC2W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC2W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC2W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC2W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC2W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC2W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC2W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC2W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC2W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC2W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC2W::VALUE18)
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
#[doc = "Values that can be written to the field `PC3`"]
pub enum PC3W {
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
impl PC3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PC3W::VALUE1 => 0,
            PC3W::VALUE2 => 1,
            PC3W::VALUE3 => 2,
            PC3W::VALUE4 => 3,
            PC3W::VALUE5 => 4,
            PC3W::VALUE6 => 5,
            PC3W::VALUE7 => 6,
            PC3W::VALUE8 => 7,
            PC3W::VALUE9 => 16,
            PC3W::VALUE10 => 17,
            PC3W::VALUE11 => 18,
            PC3W::VALUE12 => 19,
            PC3W::VALUE13 => 20,
            PC3W::VALUE14 => 24,
            PC3W::VALUE15 => 25,
            PC3W::VALUE16 => 26,
            PC3W::VALUE17 => 27,
            PC3W::VALUE18 => 28,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PC3W<'a> {
    w: &'a mut W,
}
impl<'a> _PC3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PC3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Input - No internal pull device active"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PC3W::VALUE1)
    }
    #[doc = "Input - Internal pull-down device active"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PC3W::VALUE2)
    }
    #[doc = "Input - Internal pull-up device active"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PC3W::VALUE3)
    }
    #[doc = "Input - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PC3W::VALUE4)
    }
    #[doc = "Input inverted - No internal pull device active"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(PC3W::VALUE5)
    }
    #[doc = "Input inverted - Internal pull-down device active"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(PC3W::VALUE6)
    }
    #[doc = "Input inverted - Internal pull-up device active"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(PC3W::VALUE7)
    }
    #[doc = "Input inverted - No internal pull device, Pn_OUTx = input value"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(PC3W::VALUE8)
    }
    #[doc = "Output Push-Pull - General-purpose output"]
    #[inline]
    pub fn value9(self) -> &'a mut W {
        self.variant(PC3W::VALUE9)
    }
    #[doc = "Output Push-Pull - Alternate output function 1"]
    #[inline]
    pub fn value10(self) -> &'a mut W {
        self.variant(PC3W::VALUE10)
    }
    #[doc = "Output Push-Pull - Alternate output function 2"]
    #[inline]
    pub fn value11(self) -> &'a mut W {
        self.variant(PC3W::VALUE11)
    }
    #[doc = "Output Push-Pull - Alternate output function 3"]
    #[inline]
    pub fn value12(self) -> &'a mut W {
        self.variant(PC3W::VALUE12)
    }
    #[doc = "Output Push-Pull - Alternate output function 4"]
    #[inline]
    pub fn value13(self) -> &'a mut W {
        self.variant(PC3W::VALUE13)
    }
    #[doc = "Output Open Drain - General-purpose output"]
    #[inline]
    pub fn value14(self) -> &'a mut W {
        self.variant(PC3W::VALUE14)
    }
    #[doc = "Output Open Drain - Alternate output function 1"]
    #[inline]
    pub fn value15(self) -> &'a mut W {
        self.variant(PC3W::VALUE15)
    }
    #[doc = "Output Open Drain - Alternate output function 2"]
    #[inline]
    pub fn value16(self) -> &'a mut W {
        self.variant(PC3W::VALUE16)
    }
    #[doc = "Output Open Drain - Alternate output function 3"]
    #[inline]
    pub fn value17(self) -> &'a mut W {
        self.variant(PC3W::VALUE17)
    }
    #[doc = "Output Open Drain - Alternate output function 4"]
    #[inline]
    pub fn value18(self) -> &'a mut W {
        self.variant(PC3W::VALUE18)
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc0(&self) -> PC0R {
        PC0R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc1(&self) -> PC1R {
        PC1R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc2(&self) -> PC2R {
        PC2R::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc3(&self) -> PC3R {
        PC3R::_from({
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
    #[doc = "Bits 3:7 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc0(&mut self) -> _PC0W {
        _PC0W { w: self }
    }
    #[doc = "Bits 11:15 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc1(&mut self) -> _PC1W {
        _PC1W { w: self }
    }
    #[doc = "Bits 19:23 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc2(&mut self) -> _PC2W {
        _PC2W { w: self }
    }
    #[doc = "Bits 27:31 - Port Control for Port n Pin 0 to 3"]
    #[inline]
    pub fn pc3(&mut self) -> _PC3W {
        _PC3W { w: self }
    }
}
