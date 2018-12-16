#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STCON {
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
#[doc = "Possible values of the field `HWCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWCONR {
    #[doc = "Normal mode, JTAG"]
    VALUE1,
    #[doc = "ASC BSL enabled"]
    VALUE2,
    #[doc = "BMI customized boot enabled"]
    VALUE3,
    #[doc = "CAN BSL enabled"]
    VALUE4,
}
impl HWCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HWCONR::VALUE1 => 0,
            HWCONR::VALUE2 => 1,
            HWCONR::VALUE3 => 2,
            HWCONR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HWCONR {
        match value {
            0 => HWCONR::VALUE1,
            1 => HWCONR::VALUE2,
            2 => HWCONR::VALUE3,
            3 => HWCONR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HWCONR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HWCONR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HWCONR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HWCONR::VALUE4
    }
}
#[doc = "Possible values of the field `SWCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCONR {
    #[doc = "Normal mode, boot from Boot ROM"]
    VALUE1,
    #[doc = "ASC BSL enabled"]
    VALUE2,
    #[doc = "BMI customized boot enabled"]
    VALUE3,
    #[doc = "CAN BSL enabled"]
    VALUE4,
    #[doc = "Boot from Code SRAM"]
    VALUE5,
    #[doc = "Boot from alternate Flash Address 0"]
    VALUE6,
    #[doc = "Boot from alternate Flash Address 1"]
    VALUE7,
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    VALUE8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWCONR::VALUE1 => 0,
            SWCONR::VALUE2 => 1,
            SWCONR::VALUE3 => 2,
            SWCONR::VALUE4 => 3,
            SWCONR::VALUE5 => 4,
            SWCONR::VALUE6 => 8,
            SWCONR::VALUE7 => 12,
            SWCONR::VALUE8 => 14,
            SWCONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWCONR {
        match value {
            0 => SWCONR::VALUE1,
            1 => SWCONR::VALUE2,
            2 => SWCONR::VALUE3,
            3 => SWCONR::VALUE4,
            4 => SWCONR::VALUE5,
            8 => SWCONR::VALUE6,
            12 => SWCONR::VALUE7,
            14 => SWCONR::VALUE8,
            i => SWCONR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SWCONR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SWCONR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SWCONR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SWCONR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SWCONR::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline]
    pub fn is_value6(&self) -> bool {
        *self == SWCONR::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline]
    pub fn is_value7(&self) -> bool {
        *self == SWCONR::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline]
    pub fn is_value8(&self) -> bool {
        *self == SWCONR::VALUE8
    }
}
#[doc = "Values that can be written to the field `SWCON`"]
pub enum SWCONW {
    #[doc = "Normal mode, boot from Boot ROM"]
    VALUE1,
    #[doc = "ASC BSL enabled"]
    VALUE2,
    #[doc = "BMI customized boot enabled"]
    VALUE3,
    #[doc = "CAN BSL enabled"]
    VALUE4,
    #[doc = "Boot from Code SRAM"]
    VALUE5,
    #[doc = "Boot from alternate Flash Address 0"]
    VALUE6,
    #[doc = "Boot from alternate Flash Address 1"]
    VALUE7,
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    VALUE8,
}
impl SWCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWCONW::VALUE1 => 0,
            SWCONW::VALUE2 => 1,
            SWCONW::VALUE3 => 2,
            SWCONW::VALUE4 => 3,
            SWCONW::VALUE5 => 4,
            SWCONW::VALUE6 => 8,
            SWCONW::VALUE7 => 12,
            SWCONW::VALUE8 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWCONW<'a> {
    w: &'a mut W,
}
impl<'a> _SWCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWCONW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode, boot from Boot ROM"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SWCONW::VALUE1)
    }
    #[doc = "ASC BSL enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SWCONW::VALUE2)
    }
    #[doc = "BMI customized boot enabled"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SWCONW::VALUE3)
    }
    #[doc = "CAN BSL enabled"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SWCONW::VALUE4)
    }
    #[doc = "Boot from Code SRAM"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SWCONW::VALUE5)
    }
    #[doc = "Boot from alternate Flash Address 0"]
    #[inline]
    pub fn value6(self) -> &'a mut W {
        self.variant(SWCONW::VALUE6)
    }
    #[doc = "Boot from alternate Flash Address 1"]
    #[inline]
    pub fn value7(self) -> &'a mut W {
        self.variant(SWCONW::VALUE7)
    }
    #[doc = "Enable fallback Alternate Boot Mode (ABM)"]
    #[inline]
    pub fn value8(self) -> &'a mut W {
        self.variant(SWCONW::VALUE8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - HW Configuration"]
    #[inline]
    pub fn hwcon(&self) -> HWCONR {
        HWCONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline]
    pub fn swcon(&self) -> SWCONR {
        SWCONR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 8:11 - SW Configuration"]
    #[inline]
    pub fn swcon(&mut self) -> _SWCONW {
        _SWCONW { w: self }
    }
}
