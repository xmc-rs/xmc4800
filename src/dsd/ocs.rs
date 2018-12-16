#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OCS {
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
#[doc = "Possible values of the field `SUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSR {
    #[doc = "Will not suspend"]
    VALUE1,
    #[doc = "Hard suspend: Clock is switched off immediately."]
    VALUE2,
    #[doc = "Soft suspend channel 0"]
    VALUE3,
    #[doc = "Soft suspend channel 1"]
    VALUE4,
    #[doc = "Soft suspend channel 3"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SUSR::VALUE1 => 0,
            SUSR::VALUE2 => 1,
            SUSR::VALUE3 => 2,
            SUSR::VALUE4 => 3,
            SUSR::VALUE5 => 5,
            SUSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SUSR {
        match value {
            0 => SUSR::VALUE1,
            1 => SUSR::VALUE2,
            2 => SUSR::VALUE3,
            3 => SUSR::VALUE4,
            5 => SUSR::VALUE5,
            i => SUSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SUSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SUSR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == SUSR::VALUE5
    }
}
#[doc = "Possible values of the field `SUSSTA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSSTAR {
    #[doc = "Module is not (yet) suspended"]
    VALUE1,
    #[doc = "Module is suspended"]
    VALUE2,
}
impl SUSSTAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SUSSTAR::VALUE1 => false,
            SUSSTAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSSTAR {
        match value {
            false => SUSSTAR::VALUE1,
            true => SUSSTAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SUSSTAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SUSSTAR::VALUE2
    }
}
#[doc = "Values that can be written to the field `SUS`"]
pub enum SUSW {
    #[doc = "Will not suspend"]
    VALUE1,
    #[doc = "Hard suspend: Clock is switched off immediately."]
    VALUE2,
    #[doc = "Soft suspend channel 0"]
    VALUE3,
    #[doc = "Soft suspend channel 1"]
    VALUE4,
    #[doc = "Soft suspend channel 3"]
    VALUE5,
}
impl SUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SUSW::VALUE1 => 0,
            SUSW::VALUE2 => 1,
            SUSW::VALUE3 => 2,
            SUSW::VALUE4 => 3,
            SUSW::VALUE5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Will not suspend"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSW::VALUE1)
    }
    #[doc = "Hard suspend: Clock is switched off immediately."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSW::VALUE2)
    }
    #[doc = "Soft suspend channel 0"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSW::VALUE3)
    }
    #[doc = "Soft suspend channel 1"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSW::VALUE4)
    }
    #[doc = "Soft suspend channel 3"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(SUSW::VALUE5)
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
#[doc = r" Proxy"]
pub struct _SUS_PW<'a> {
    w: &'a mut W,
}
impl<'a> _SUS_PW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline]
    pub fn sus(&self) -> SUSR {
        SUSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - Suspend State"]
    #[inline]
    pub fn sussta(&self) -> SUSSTAR {
        SUSSTAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 24:27 - OCDS Suspend Control"]
    #[inline]
    pub fn sus(&mut self) -> _SUSW {
        _SUSW { w: self }
    }
    #[doc = "Bit 28 - SUS Write Protection"]
    #[inline]
    pub fn sus_p(&mut self) -> _SUS_PW {
        _SUS_PW { w: self }
    }
}
