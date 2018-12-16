#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MII_PDI_ACS_STATE {
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
#[doc = "Possible values of the field `ACS_MII_BY_PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACS_MII_BY_PDIR {
    #[doc = "ECAT has access to MII managment"]
    VALUE1,
    #[doc = "PDI has access to MII managment"]
    VALUE2,
}
impl ACS_MII_BY_PDIR {
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
            ACS_MII_BY_PDIR::VALUE1 => false,
            ACS_MII_BY_PDIR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACS_MII_BY_PDIR {
        match value {
            false => ACS_MII_BY_PDIR::VALUE1,
            true => ACS_MII_BY_PDIR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ACS_MII_BY_PDIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ACS_MII_BY_PDIR::VALUE2
    }
}
#[doc = "Possible values of the field `FORCE_PDI_ACS_S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_PDI_ACS_SR {
    #[doc = "no change"]
    VALUE1,
    #[doc = "Reset Bit ACS_MII_BY_PDI"]
    VALUE2,
}
impl FORCE_PDI_ACS_SR {
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
            FORCE_PDI_ACS_SR::VALUE1 => false,
            FORCE_PDI_ACS_SR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCE_PDI_ACS_SR {
        match value {
            false => FORCE_PDI_ACS_SR::VALUE1,
            true => FORCE_PDI_ACS_SR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_PDI_ACS_SR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_PDI_ACS_SR::VALUE2
    }
}
#[doc = "Values that can be written to the field `ACS_MII_BY_PDI`"]
pub enum ACS_MII_BY_PDIW {
    #[doc = "ECAT has access to MII managment"]
    VALUE1,
    #[doc = "PDI has access to MII managment"]
    VALUE2,
}
impl ACS_MII_BY_PDIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACS_MII_BY_PDIW::VALUE1 => false,
            ACS_MII_BY_PDIW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACS_MII_BY_PDIW<'a> {
    w: &'a mut W,
}
impl<'a> _ACS_MII_BY_PDIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACS_MII_BY_PDIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ECAT has access to MII managment"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ACS_MII_BY_PDIW::VALUE1)
    }
    #[doc = "PDI has access to MII managment"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ACS_MII_BY_PDIW::VALUE2)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Access to MII management"]
    #[inline]
    pub fn acs_mii_by_pdi(&self) -> ACS_MII_BY_PDIR {
        ACS_MII_BY_PDIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Force PDI Access State by ECAT master"]
    #[inline]
    pub fn force_pdi_acs_s(&self) -> FORCE_PDI_ACS_SR {
        FORCE_PDI_ACS_SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Access to MII management"]
    #[inline]
    pub fn acs_mii_by_pdi(&mut self) -> _ACS_MII_BY_PDIW {
        _ACS_MII_BY_PDIW { w: self }
    }
}
