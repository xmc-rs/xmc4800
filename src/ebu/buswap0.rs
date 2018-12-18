#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUSWAP0 {
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
#[doc = "Possible values of the field `WRDTACS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRDTACSR {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRDTACSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRDTACSR::VALUE1 => 0,
            WRDTACSR::VALUE2 => 1,
            WRDTACSR::VALUE3 => 14,
            WRDTACSR::VALUE4 => 15,
            WRDTACSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRDTACSR {
        match value {
            0 => WRDTACSR::VALUE1,
            1 => WRDTACSR::VALUE2,
            14 => WRDTACSR::VALUE3,
            15 => WRDTACSR::VALUE4,
            i => WRDTACSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WRDTACSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WRDTACSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WRDTACSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == WRDTACSR::VALUE4
    }
}
#[doc = "Possible values of the field `WRRECOVC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRRECOVCR {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "6 clock cycles selected."]
    VALUE3,
    #[doc = "7 clock cycles selected."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRRECOVCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRRECOVCR::VALUE1 => 0,
            WRRECOVCR::VALUE2 => 1,
            WRRECOVCR::VALUE3 => 6,
            WRRECOVCR::VALUE4 => 7,
            WRRECOVCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRRECOVCR {
        match value {
            0 => WRRECOVCR::VALUE1,
            1 => WRRECOVCR::VALUE2,
            6 => WRRECOVCR::VALUE3,
            7 => WRRECOVCR::VALUE4,
            i => WRRECOVCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WRRECOVCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WRRECOVCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WRRECOVCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == WRRECOVCR::VALUE4
    }
}
#[doc = "Possible values of the field `WAITWRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITWRCR {
    #[doc = "1 wait state."]
    VALUE1,
    #[doc = "1 wait states."]
    VALUE2,
    #[doc = "2 wait state."]
    VALUE3,
    #[doc = "30 wait states."]
    VALUE4,
    #[doc = "31 wait states."]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WAITWRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAITWRCR::VALUE1 => 0,
            WAITWRCR::VALUE2 => 1,
            WAITWRCR::VALUE3 => 2,
            WAITWRCR::VALUE4 => 30,
            WAITWRCR::VALUE5 => 31,
            WAITWRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAITWRCR {
        match value {
            0 => WAITWRCR::VALUE1,
            1 => WAITWRCR::VALUE2,
            2 => WAITWRCR::VALUE3,
            30 => WAITWRCR::VALUE4,
            31 => WAITWRCR::VALUE5,
            i => WAITWRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAITWRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAITWRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WAITWRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == WAITWRCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == WAITWRCR::VALUE5
    }
}
#[doc = "Possible values of the field `DATAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATACR {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATACR::VALUE1 => 0,
            DATACR::VALUE2 => 1,
            DATACR::VALUE3 => 14,
            DATACR::VALUE4 => 15,
            DATACR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATACR {
        match value {
            0 => DATACR::VALUE1,
            1 => DATACR::VALUE2,
            14 => DATACR::VALUE3,
            15 => DATACR::VALUE4,
            i => DATACR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATACR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATACR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == DATACR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == DATACR::VALUE4
    }
}
#[doc = "Possible values of the field `EXTCLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTCLOCKR {
    #[doc = "Equal to INT_CLK frequency."]
    VALUE1,
    #[doc = "1/2 of INT_CLK frequency."]
    VALUE2,
    #[doc = "1/3 of INT_CLK frequency."]
    VALUE3,
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    VALUE4,
}
impl EXTCLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTCLOCKR::VALUE1 => 0,
            EXTCLOCKR::VALUE2 => 1,
            EXTCLOCKR::VALUE3 => 2,
            EXTCLOCKR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTCLOCKR {
        match value {
            0 => EXTCLOCKR::VALUE1,
            1 => EXTCLOCKR::VALUE2,
            2 => EXTCLOCKR::VALUE3,
            3 => EXTCLOCKR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXTCLOCKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXTCLOCKR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXTCLOCKR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXTCLOCKR::VALUE4
    }
}
#[doc = "Possible values of the field `EXTDATA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTDATAR {
    #[doc = "external memory outputs data every BFCLK cycle"]
    VALUE1,
    #[doc = "external memory outputs data every two BFCLK cycles"]
    VALUE2,
    #[doc = "external memory outputs data every four BFCLK cycles"]
    VALUE3,
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    VALUE4,
}
impl EXTDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTDATAR::VALUE1 => 0,
            EXTDATAR::VALUE2 => 1,
            EXTDATAR::VALUE3 => 2,
            EXTDATAR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTDATAR {
        match value {
            0 => EXTDATAR::VALUE1,
            1 => EXTDATAR::VALUE2,
            2 => EXTDATAR::VALUE3,
            3 => EXTDATAR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EXTDATAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EXTDATAR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == EXTDATAR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == EXTDATAR::VALUE4
    }
}
#[doc = "Possible values of the field `CMDDELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDDELAYR {
    #[doc = "0 clock cycle selected."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDDELAYR::VALUE1 => 0,
            CMDDELAYR::VALUE2 => 1,
            CMDDELAYR::VALUE3 => 14,
            CMDDELAYR::VALUE4 => 15,
            CMDDELAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDDELAYR {
        match value {
            0 => CMDDELAYR::VALUE1,
            1 => CMDDELAYR::VALUE2,
            14 => CMDDELAYR::VALUE3,
            15 => CMDDELAYR::VALUE4,
            i => CMDDELAYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CMDDELAYR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CMDDELAYR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CMDDELAYR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CMDDELAYR::VALUE4
    }
}
#[doc = "Possible values of the field `AHOLDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHOLDCR {
    #[doc = "0 clock cycle selected"]
    VALUE1,
    #[doc = "1 clock cycle selected"]
    VALUE2,
    #[doc = "14 clock cycles selected"]
    VALUE3,
    #[doc = "15 clock cycles selected"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AHOLDCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHOLDCR::VALUE1 => 0,
            AHOLDCR::VALUE2 => 1,
            AHOLDCR::VALUE3 => 14,
            AHOLDCR::VALUE4 => 15,
            AHOLDCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHOLDCR {
        match value {
            0 => AHOLDCR::VALUE1,
            1 => AHOLDCR::VALUE2,
            14 => AHOLDCR::VALUE3,
            15 => AHOLDCR::VALUE4,
            i => AHOLDCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHOLDCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHOLDCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == AHOLDCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == AHOLDCR::VALUE4
    }
}
#[doc = "Possible values of the field `ADDRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRCR {
    #[doc = "1 clock cycle selected"]
    VALUE1,
    #[doc = "1 clock cycle selected"]
    VALUE2,
    #[doc = "14 clock cycles selected"]
    VALUE3,
    #[doc = "15 clock cycles selected"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADDRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADDRCR::VALUE1 => 0,
            ADDRCR::VALUE2 => 1,
            ADDRCR::VALUE3 => 14,
            ADDRCR::VALUE4 => 15,
            ADDRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADDRCR {
        match value {
            0 => ADDRCR::VALUE1,
            1 => ADDRCR::VALUE2,
            14 => ADDRCR::VALUE3,
            15 => ADDRCR::VALUE4,
            i => ADDRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ADDRCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ADDRCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == ADDRCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == ADDRCR::VALUE4
    }
}
#[doc = "Values that can be written to the field `WRDTACS`"]
pub enum WRDTACSW {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
}
impl WRDTACSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRDTACSW::VALUE1 => 0,
            WRDTACSW::VALUE2 => 1,
            WRDTACSW::VALUE3 => 14,
            WRDTACSW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRDTACSW<'a> {
    w: &'a mut W,
}
impl<'a> _WRDTACSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRDTACSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WRDTACSW::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WRDTACSW::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WRDTACSW::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(WRDTACSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRRECOVC`"]
pub enum WRRECOVCW {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "6 clock cycles selected."]
    VALUE3,
    #[doc = "7 clock cycles selected."]
    VALUE4,
}
impl WRRECOVCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRRECOVCW::VALUE1 => 0,
            WRRECOVCW::VALUE2 => 1,
            WRRECOVCW::VALUE3 => 6,
            WRRECOVCW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRRECOVCW<'a> {
    w: &'a mut W,
}
impl<'a> _WRRECOVCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRRECOVCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WRRECOVCW::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WRRECOVCW::VALUE2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WRRECOVCW::VALUE3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(WRRECOVCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAITWRC`"]
pub enum WAITWRCW {
    #[doc = "1 wait state."]
    VALUE1,
    #[doc = "1 wait states."]
    VALUE2,
    #[doc = "2 wait state."]
    VALUE3,
    #[doc = "30 wait states."]
    VALUE4,
    #[doc = "31 wait states."]
    VALUE5,
}
impl WAITWRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAITWRCW::VALUE1 => 0,
            WAITWRCW::VALUE2 => 1,
            WAITWRCW::VALUE3 => 2,
            WAITWRCW::VALUE4 => 30,
            WAITWRCW::VALUE5 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITWRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITWRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITWRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 wait state."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITWRCW::VALUE1)
    }
    #[doc = "1 wait states."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITWRCW::VALUE2)
    }
    #[doc = "2 wait state."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WAITWRCW::VALUE3)
    }
    #[doc = "30 wait states."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(WAITWRCW::VALUE4)
    }
    #[doc = "31 wait states."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(WAITWRCW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATAC`"]
pub enum DATACW {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
}
impl DATACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATACW::VALUE1 => 0,
            DATACW::VALUE2 => 1,
            DATACW::VALUE3 => 14,
            DATACW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATACW<'a> {
    w: &'a mut W,
}
impl<'a> _DATACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATACW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATACW::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATACW::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(DATACW::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(DATACW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTCLOCK`"]
pub enum EXTCLOCKW {
    #[doc = "Equal to INT_CLK frequency."]
    VALUE1,
    #[doc = "1/2 of INT_CLK frequency."]
    VALUE2,
    #[doc = "1/3 of INT_CLK frequency."]
    VALUE3,
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    VALUE4,
}
impl EXTCLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTCLOCKW::VALUE1 => 0,
            EXTCLOCKW::VALUE2 => 1,
            EXTCLOCKW::VALUE3 => 2,
            EXTCLOCKW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTCLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTCLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTCLOCKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Equal to INT_CLK frequency."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTCLOCKW::VALUE1)
    }
    #[doc = "1/2 of INT_CLK frequency."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTCLOCKW::VALUE2)
    }
    #[doc = "1/3 of INT_CLK frequency."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXTCLOCKW::VALUE3)
    }
    #[doc = "1/4 of INT_CLK frequency (default after reset)."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXTCLOCKW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTDATA`"]
pub enum EXTDATAW {
    #[doc = "external memory outputs data every BFCLK cycle"]
    VALUE1,
    #[doc = "external memory outputs data every two BFCLK cycles"]
    VALUE2,
    #[doc = "external memory outputs data every four BFCLK cycles"]
    VALUE3,
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    VALUE4,
}
impl EXTDATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTDATAW::VALUE1 => 0,
            EXTDATAW::VALUE2 => 1,
            EXTDATAW::VALUE3 => 2,
            EXTDATAW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTDATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTDATAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "external memory outputs data every BFCLK cycle"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EXTDATAW::VALUE1)
    }
    #[doc = "external memory outputs data every two BFCLK cycles"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EXTDATAW::VALUE2)
    }
    #[doc = "external memory outputs data every four BFCLK cycles"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(EXTDATAW::VALUE3)
    }
    #[doc = "external memory outputs data every eight BFCLK cycles"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(EXTDATAW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDDELAY`"]
pub enum CMDDELAYW {
    #[doc = "0 clock cycle selected."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
}
impl CMDDELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDDELAYW::VALUE1 => 0,
            CMDDELAYW::VALUE2 => 1,
            CMDDELAYW::VALUE3 => 14,
            CMDDELAYW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDDELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDDELAYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 clock cycle selected."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMDDELAYW::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMDDELAYW::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMDDELAYW::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMDDELAYW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHOLDC`"]
pub enum AHOLDCW {
    #[doc = "0 clock cycle selected"]
    VALUE1,
    #[doc = "1 clock cycle selected"]
    VALUE2,
    #[doc = "14 clock cycles selected"]
    VALUE3,
    #[doc = "15 clock cycles selected"]
    VALUE4,
}
impl AHOLDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AHOLDCW::VALUE1 => 0,
            AHOLDCW::VALUE2 => 1,
            AHOLDCW::VALUE3 => 14,
            AHOLDCW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHOLDCW<'a> {
    w: &'a mut W,
}
impl<'a> _AHOLDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHOLDCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 clock cycle selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHOLDCW::VALUE1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHOLDCW::VALUE2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(AHOLDCW::VALUE3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(AHOLDCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDRC`"]
pub enum ADDRCW {
    #[doc = "1 clock cycle selected"]
    VALUE1,
    #[doc = "1 clock cycle selected"]
    VALUE2,
    #[doc = "14 clock cycles selected"]
    VALUE3,
    #[doc = "15 clock cycles selected"]
    VALUE4,
}
impl ADDRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADDRCW::VALUE1 => 0,
            ADDRCW::VALUE2 => 1,
            ADDRCW::VALUE3 => 14,
            ADDRCW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 clock cycle selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ADDRCW::VALUE1)
    }
    #[doc = "1 clock cycle selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ADDRCW::VALUE2)
    }
    #[doc = "14 clock cycles selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(ADDRCW::VALUE3)
    }
    #[doc = "15 clock cycles selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(ADDRCW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Recovery Cycles between Different Regions"]
    #[inline]
    pub fn wrdtacs(&self) -> WRDTACSR {
        WRDTACSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline]
    pub fn wrrecovc(&self) -> WRRECOVCR {
        WRRECOVCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline]
    pub fn waitwrc(&self) -> WAITWRCR {
        WAITWRCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
    #[inline]
    pub fn datac(&self) -> DATACR {
        DATACR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline]
    pub fn extclock(&self) -> EXTCLOCKR {
        EXTCLOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline]
    pub fn extdata(&self) -> EXTDATAR {
        EXTDATAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline]
    pub fn cmddelay(&self) -> CMDDELAYR {
        CMDDELAYR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline]
    pub fn aholdc(&self) -> AHOLDCR {
        AHOLDCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline]
    pub fn addrc(&self) -> ADDRCR {
        ADDRCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Recovery Cycles between Different Regions"]
    #[inline]
    pub fn wrdtacs(&mut self) -> _WRDTACSW {
        _WRDTACSW { w: self }
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Write Accesses"]
    #[inline]
    pub fn wrrecovc(&mut self) -> _WRRECOVCW {
        _WRRECOVCW { w: self }
    }
    #[doc = "Bits 7:11 - Programmed Wait States for write accesses"]
    #[inline]
    pub fn waitwrc(&mut self) -> _WAITWRCW {
        _WAITWRCW { w: self }
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Write Accesses"]
    #[inline]
    pub fn datac(&mut self) -> _DATACW {
        _DATACW { w: self }
    }
    #[doc = "Bits 16:17 - Frequency of external clock at pin BFCLKO"]
    #[inline]
    pub fn extclock(&mut self) -> _EXTCLOCKW {
        _EXTCLOCKW { w: self }
    }
    #[doc = "Bits 18:19 - Extended data"]
    #[inline]
    pub fn extdata(&mut self) -> _EXTDATAW {
        _EXTDATAW { w: self }
    }
    #[doc = "Bits 20:23 - Command Delay Cycles"]
    #[inline]
    pub fn cmddelay(&mut self) -> _CMDDELAYW {
        _CMDDELAYW { w: self }
    }
    #[doc = "Bits 24:27 - Address Hold Cycles"]
    #[inline]
    pub fn aholdc(&mut self) -> _AHOLDCW {
        _AHOLDCW { w: self }
    }
    #[doc = "Bits 28:31 - Address Cycles"]
    #[inline]
    pub fn addrc(&mut self) -> _ADDRCW {
        _ADDRCW { w: self }
    }
}
