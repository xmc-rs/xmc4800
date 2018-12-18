#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDR0 {
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
#[doc = "Possible values of the field `PD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD0R {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD0R::SD_SHE => 0,
            PD0R::SD_MEE => 1,
            PD0R::SD_SOE => 2,
            PD0R::MD => 4,
            PD0R::WD => 7,
            PD0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD0R {
        match value {
            0 => PD0R::SD_SHE,
            1 => PD0R::SD_MEE,
            2 => PD0R::SD_SOE,
            4 => PD0R::MD,
            7 => PD0R::WD,
            i => PD0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD0R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD0R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD0R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD0R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD0R::WD
    }
}
#[doc = "Possible values of the field `PD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD1R {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD1R::SD_SHE => 0,
            PD1R::SD_MEE => 1,
            PD1R::SD_SOE => 2,
            PD1R::MD => 4,
            PD1R::WD => 7,
            PD1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD1R {
        match value {
            0 => PD1R::SD_SHE,
            1 => PD1R::SD_MEE,
            2 => PD1R::SD_SOE,
            4 => PD1R::MD,
            7 => PD1R::WD,
            i => PD1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD1R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD1R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD1R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD1R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD1R::WD
    }
}
#[doc = "Possible values of the field `PD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD2R {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD2R::SD_SHE => 0,
            PD2R::SD_MEE => 1,
            PD2R::SD_SOE => 2,
            PD2R::MD => 4,
            PD2R::WD => 7,
            PD2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD2R {
        match value {
            0 => PD2R::SD_SHE,
            1 => PD2R::SD_MEE,
            2 => PD2R::SD_SOE,
            4 => PD2R::MD,
            7 => PD2R::WD,
            i => PD2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD2R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD2R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD2R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD2R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD2R::WD
    }
}
#[doc = "Possible values of the field `PD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD3R {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD3R::SD_SHE => 0,
            PD3R::SD_MEE => 1,
            PD3R::SD_SOE => 2,
            PD3R::MD => 4,
            PD3R::WD => 7,
            PD3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD3R {
        match value {
            0 => PD3R::SD_SHE,
            1 => PD3R::SD_MEE,
            2 => PD3R::SD_SOE,
            4 => PD3R::MD,
            7 => PD3R::WD,
            i => PD3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD3R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD3R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD3R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD3R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD3R::WD
    }
}
#[doc = "Possible values of the field `PD4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD4R {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD4R::SD_SHE => 0,
            PD4R::SD_MEE => 1,
            PD4R::SD_SOE => 2,
            PD4R::MD => 4,
            PD4R::WD => 7,
            PD4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD4R {
        match value {
            0 => PD4R::SD_SHE,
            1 => PD4R::SD_MEE,
            2 => PD4R::SD_SOE,
            4 => PD4R::MD,
            7 => PD4R::WD,
            i => PD4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD4R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD4R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD4R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD4R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD4R::WD
    }
}
#[doc = "Possible values of the field `PD5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD5R {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD5R::SD_SHE => 0,
            PD5R::SD_MEE => 1,
            PD5R::SD_SOE => 2,
            PD5R::MD => 4,
            PD5R::WD => 7,
            PD5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD5R {
        match value {
            0 => PD5R::SD_SHE,
            1 => PD5R::SD_MEE,
            2 => PD5R::SD_SOE,
            4 => PD5R::MD,
            7 => PD5R::WD,
            i => PD5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD5R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD5R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD5R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD5R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD5R::WD
    }
}
#[doc = "Possible values of the field `PD6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD6R {
    #[doc = "A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "A1+ medium driver"]
    MD,
    #[doc = "A1+ weak driver"]
    WD,
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl PD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD6R::SD_SOE => 2,
            PD6R::SD_SLE => 3,
            PD6R::MD => 4,
            PD6R::WD => 7,
            PD6R::SD_SOE_ALT => 0,
            PD6R::SD_SLE_ALT => 1,
            PD6R::MD_ALT => 6,
            PD6R::WD_ALT => 5,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD6R {
        match value {
            2 => PD6R::SD_SOE,
            3 => PD6R::SD_SLE,
            4 => PD6R::MD,
            7 => PD6R::WD,
            0 => PD6R::SD_SOE_ALT,
            1 => PD6R::SD_SLE_ALT,
            6 => PD6R::MD_ALT,
            5 => PD6R::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD6R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD6R::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD6R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD6R::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD6R::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD6R::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline]
    pub fn is_md_alt(&self) -> bool {
        *self == PD6R::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD6R::WD_ALT
    }
}
#[doc = "Possible values of the field `PD7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD7R {
    #[doc = "A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "A1+ medium driver"]
    MD,
    #[doc = "A1+ weak driver"]
    WD,
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl PD7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD7R::SD_SOE => 2,
            PD7R::SD_SLE => 3,
            PD7R::MD => 4,
            PD7R::WD => 7,
            PD7R::SD_SOE_ALT => 0,
            PD7R::SD_SLE_ALT => 1,
            PD7R::MD_ALT => 6,
            PD7R::WD_ALT => 5,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD7R {
        match value {
            2 => PD7R::SD_SOE,
            3 => PD7R::SD_SLE,
            4 => PD7R::MD,
            7 => PD7R::WD,
            0 => PD7R::SD_SOE_ALT,
            1 => PD7R::SD_SLE_ALT,
            6 => PD7R::MD_ALT,
            5 => PD7R::WD_ALT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD7R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `SD_SLE`"]
    #[inline]
    pub fn is_sd_sle(&self) -> bool {
        *self == PD7R::SD_SLE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD7R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD7R::WD
    }
    #[doc = "Checks if the value of the field is `SD_SOE_ALT`"]
    #[inline]
    pub fn is_sd_soe_alt(&self) -> bool {
        *self == PD7R::SD_SOE_ALT
    }
    #[doc = "Checks if the value of the field is `SD_SLE_ALT`"]
    #[inline]
    pub fn is_sd_sle_alt(&self) -> bool {
        *self == PD7R::SD_SLE_ALT
    }
    #[doc = "Checks if the value of the field is `MD_ALT`"]
    #[inline]
    pub fn is_md_alt(&self) -> bool {
        *self == PD7R::MD_ALT
    }
    #[doc = "Checks if the value of the field is `WD_ALT`"]
    #[inline]
    pub fn is_wd_alt(&self) -> bool {
        *self == PD7R::WD_ALT
    }
}
#[doc = "Values that can be written to the field `PD0`"]
pub enum PD0W {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
}
impl PD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD0W::SD_SHE => 0,
            PD0W::SD_MEE => 1,
            PD0W::SD_SOE => 2,
            PD0W::MD => 4,
            PD0W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD0W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD0W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD0W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD0W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD0W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD1`"]
pub enum PD1W {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
}
impl PD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD1W::SD_SHE => 0,
            PD1W::SD_MEE => 1,
            PD1W::SD_SOE => 2,
            PD1W::MD => 4,
            PD1W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD1W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD1W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD1W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD1W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD1W::WD)
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
#[doc = "Values that can be written to the field `PD2`"]
pub enum PD2W {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
}
impl PD2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD2W::SD_SHE => 0,
            PD2W::SD_MEE => 1,
            PD2W::SD_SOE => 2,
            PD2W::MD => 4,
            PD2W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD2W<'a> {
    w: &'a mut W,
}
impl<'a> _PD2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD2W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD2W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD2W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD2W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD2W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD3`"]
pub enum PD3W {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
}
impl PD3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD3W::SD_SHE => 0,
            PD3W::SD_MEE => 1,
            PD3W::SD_SOE => 2,
            PD3W::MD => 4,
            PD3W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD3W<'a> {
    w: &'a mut W,
}
impl<'a> _PD3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD3W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD3W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD3W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD3W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD3W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD4`"]
pub enum PD4W {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
}
impl PD4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD4W::SD_SHE => 0,
            PD4W::SD_MEE => 1,
            PD4W::SD_SOE => 2,
            PD4W::MD => 4,
            PD4W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD4W<'a> {
    w: &'a mut W,
}
impl<'a> _PD4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD4W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD4W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD4W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD4W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD4W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD5`"]
pub enum PD5W {
    #[doc = "A2 strong driver, sharp edge"]
    SD_SHE,
    #[doc = "A2 strong driver, medium edge"]
    SD_MEE,
    #[doc = "A2 strong driver, soft edge"]
    SD_SOE,
    #[doc = "A2 medium driver"]
    MD,
    #[doc = "A2 weak driver"]
    WD,
}
impl PD5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD5W::SD_SHE => 0,
            PD5W::SD_MEE => 1,
            PD5W::SD_SOE => 2,
            PD5W::MD => 4,
            PD5W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD5W<'a> {
    w: &'a mut W,
}
impl<'a> _PD5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD5W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD5W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD5W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD5W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD5W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD6`"]
pub enum PD6W {
    #[doc = "A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "A1+ medium driver"]
    MD,
    #[doc = "A1+ weak driver"]
    WD,
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl PD6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD6W::SD_SOE => 2,
            PD6W::SD_SLE => 3,
            PD6W::MD => 4,
            PD6W::WD => 7,
            PD6W::SD_SOE_ALT => 0,
            PD6W::SD_SLE_ALT => 1,
            PD6W::MD_ALT => 6,
            PD6W::WD_ALT => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD6W<'a> {
    w: &'a mut W,
}
impl<'a> _PD6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD6W::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD6W::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD6W::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD6W::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD6W::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD6W::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD6W::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD6W::WD_ALT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD7`"]
pub enum PD7W {
    #[doc = "A1+ strong driver, soft edge"]
    SD_SOE,
    #[doc = "A1+ strong driver, slow edge"]
    SD_SLE,
    #[doc = "A1+ medium driver"]
    MD,
    #[doc = "A1+ weak driver"]
    WD,
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    SD_SOE_ALT,
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    SD_SLE_ALT,
    #[doc = "A1+ medium driver (alternate value)"]
    MD_ALT,
    #[doc = "A1+ weak driver (alternate value)"]
    WD_ALT,
}
impl PD7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD7W::SD_SOE => 2,
            PD7W::SD_SLE => 3,
            PD7W::MD => 4,
            PD7W::WD => 7,
            PD7W::SD_SOE_ALT => 0,
            PD7W::SD_SLE_ALT => 1,
            PD7W::MD_ALT => 6,
            PD7W::WD_ALT => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD7W<'a> {
    w: &'a mut W,
}
impl<'a> _PD7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "A1+ strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD7W::SD_SOE)
    }
    #[doc = "A1+ strong driver, slow edge"]
    #[inline]
    pub fn sd_sle(self) -> &'a mut W {
        self.variant(PD7W::SD_SLE)
    }
    #[doc = "A1+ medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD7W::MD)
    }
    #[doc = "A1+ weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD7W::WD)
    }
    #[doc = "A1+ strong driver, soft edge (alternate value)"]
    #[inline]
    pub fn sd_soe_alt(self) -> &'a mut W {
        self.variant(PD7W::SD_SOE_ALT)
    }
    #[doc = "A1+ strong driver, slow edge (alternate value)"]
    #[inline]
    pub fn sd_sle_alt(self) -> &'a mut W {
        self.variant(PD7W::SD_SLE_ALT)
    }
    #[doc = "A1+ medium driver (alternate value)"]
    #[inline]
    pub fn md_alt(self) -> &'a mut W {
        self.variant(PD7W::MD_ALT)
    }
    #[doc = "A1+ weak driver (alternate value)"]
    #[inline]
    pub fn wd_alt(self) -> &'a mut W {
        self.variant(PD7W::WD_ALT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline]
    pub fn pd0(&self) -> PD0R {
        PD0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline]
    pub fn pd1(&self) -> PD1R {
        PD1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline]
    pub fn pd2(&self) -> PD2R {
        PD2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline]
    pub fn pd3(&self) -> PD3R {
        PD3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline]
    pub fn pd4(&self) -> PD4R {
        PD4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline]
    pub fn pd5(&self) -> PD5R {
        PD5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline]
    pub fn pd6(&self) -> PD6R {
        PD6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline]
    pub fn pd7(&self) -> PD7R {
        PD7R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 572662306 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.0"]
    #[inline]
    pub fn pd0(&mut self) -> _PD0W {
        _PD0W { w: self }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.1"]
    #[inline]
    pub fn pd1(&mut self) -> _PD1W {
        _PD1W { w: self }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.2"]
    #[inline]
    pub fn pd2(&mut self) -> _PD2W {
        _PD2W { w: self }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.3"]
    #[inline]
    pub fn pd3(&mut self) -> _PD3W {
        _PD3W { w: self }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.4"]
    #[inline]
    pub fn pd4(&mut self) -> _PD4W {
        _PD4W { w: self }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.5"]
    #[inline]
    pub fn pd5(&mut self) -> _PD5W {
        _PD5W { w: self }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.6"]
    #[inline]
    pub fn pd6(&mut self) -> _PD6W {
        _PD6W { w: self }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.7"]
    #[inline]
    pub fn pd7(&mut self) -> _PD7W {
        _PD7W { w: self }
    }
}
