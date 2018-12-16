#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUSRAP3 {
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
#[doc = "Possible values of the field `RDDTACS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDDTACSR {
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
impl RDDTACSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDDTACSR::VALUE1 => 0,
            RDDTACSR::VALUE2 => 1,
            RDDTACSR::VALUE3 => 14,
            RDDTACSR::VALUE4 => 15,
            RDDTACSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDDTACSR {
        match value {
            0 => RDDTACSR::VALUE1,
            1 => RDDTACSR::VALUE2,
            14 => RDDTACSR::VALUE3,
            15 => RDDTACSR::VALUE4,
            i => RDDTACSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDDTACSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDDTACSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RDDTACSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RDDTACSR::VALUE4
    }
}
#[doc = "Possible values of the field `RDRECOVC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRECOVCR {
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
impl RDRECOVCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDRECOVCR::VALUE1 => 0,
            RDRECOVCR::VALUE2 => 1,
            RDRECOVCR::VALUE3 => 6,
            RDRECOVCR::VALUE4 => 7,
            RDRECOVCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDRECOVCR {
        match value {
            0 => RDRECOVCR::VALUE1,
            1 => RDRECOVCR::VALUE2,
            6 => RDRECOVCR::VALUE3,
            7 => RDRECOVCR::VALUE4,
            i => RDRECOVCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RDRECOVCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RDRECOVCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RDRECOVCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RDRECOVCR::VALUE4
    }
}
#[doc = "Possible values of the field `WAITRDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITRDCR {
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
impl WAITRDCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAITRDCR::VALUE1 => 0,
            WAITRDCR::VALUE2 => 1,
            WAITRDCR::VALUE3 => 2,
            WAITRDCR::VALUE4 => 30,
            WAITRDCR::VALUE5 => 31,
            WAITRDCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAITRDCR {
        match value {
            0 => WAITRDCR::VALUE1,
            1 => WAITRDCR::VALUE2,
            2 => WAITRDCR::VALUE3,
            30 => WAITRDCR::VALUE4,
            31 => WAITRDCR::VALUE5,
            i => WAITRDCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WAITRDCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WAITRDCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WAITRDCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == WAITRDCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == WAITRDCR::VALUE5
    }
}
#[doc = r" Value of the field"]
pub struct DATACR {
    bits: u8,
}
impl DATACR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = "Values that can be written to the field `RDDTACS`"]
pub enum RDDTACSW {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "14 clock cycles selected."]
    VALUE3,
    #[doc = "15 clock cycles selected."]
    VALUE4,
}
impl RDDTACSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDDTACSW::VALUE1 => 0,
            RDDTACSW::VALUE2 => 1,
            RDDTACSW::VALUE3 => 14,
            RDDTACSW::VALUE4 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDDTACSW<'a> {
    w: &'a mut W,
}
impl<'a> _RDDTACSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDDTACSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDDTACSW::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDDTACSW::VALUE2)
    }
    #[doc = "14 clock cycles selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RDDTACSW::VALUE3)
    }
    #[doc = "15 clock cycles selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RDDTACSW::VALUE4)
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
#[doc = "Values that can be written to the field `RDRECOVC`"]
pub enum RDRECOVCW {
    #[doc = "No Recovery Phase clock cycles available."]
    VALUE1,
    #[doc = "1 clock cycle selected."]
    VALUE2,
    #[doc = "6 clock cycles selected."]
    VALUE3,
    #[doc = "7 clock cycles selected."]
    VALUE4,
}
impl RDRECOVCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDRECOVCW::VALUE1 => 0,
            RDRECOVCW::VALUE2 => 1,
            RDRECOVCW::VALUE3 => 6,
            RDRECOVCW::VALUE4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDRECOVCW<'a> {
    w: &'a mut W,
}
impl<'a> _RDRECOVCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDRECOVCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No Recovery Phase clock cycles available."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RDRECOVCW::VALUE1)
    }
    #[doc = "1 clock cycle selected."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RDRECOVCW::VALUE2)
    }
    #[doc = "6 clock cycles selected."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RDRECOVCW::VALUE3)
    }
    #[doc = "7 clock cycles selected."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RDRECOVCW::VALUE4)
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
#[doc = "Values that can be written to the field `WAITRDC`"]
pub enum WAITRDCW {
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
impl WAITRDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAITRDCW::VALUE1 => 0,
            WAITRDCW::VALUE2 => 1,
            WAITRDCW::VALUE3 => 2,
            WAITRDCW::VALUE4 => 30,
            WAITRDCW::VALUE5 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITRDCW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITRDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITRDCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 wait state."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITRDCW::VALUE1)
    }
    #[doc = "1 wait states."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITRDCW::VALUE2)
    }
    #[doc = "2 wait state."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WAITRDCW::VALUE3)
    }
    #[doc = "30 wait states."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(WAITRDCW::VALUE4)
    }
    #[doc = "31 wait states."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(WAITRDCW::VALUE5)
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
#[doc = r" Proxy"]
pub struct _DATACW<'a> {
    w: &'a mut W,
}
impl<'a> _DATACW<'a> {
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
    pub fn rddtacs(&self) -> RDDTACSR {
        RDDTACSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline]
    pub fn rdrecovc(&self) -> RDRECOVCR {
        RDRECOVCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline]
    pub fn waitrdc(&self) -> WAITRDCR {
        WAITRDCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
    #[inline]
    pub fn datac(&self) -> DATACR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATACR { bits }
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
    pub fn rddtacs(&mut self) -> _RDDTACSW {
        _RDDTACSW { w: self }
    }
    #[doc = "Bits 4:6 - Recovery Cycles after Read Accesses"]
    #[inline]
    pub fn rdrecovc(&mut self) -> _RDRECOVCW {
        _RDRECOVCW { w: self }
    }
    #[doc = "Bits 7:11 - Programmed Wait States for read accesses"]
    #[inline]
    pub fn waitrdc(&mut self) -> _WAITRDCW {
        _WAITRDCW { w: self }
    }
    #[doc = "Bits 12:15 - Data Hold Cycles for Read Accesses"]
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
