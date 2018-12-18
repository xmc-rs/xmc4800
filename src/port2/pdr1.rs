#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDR1 {
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
#[doc = "Possible values of the field `PD8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD8R {
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
impl PD8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD8R::SD_SHE => 0,
            PD8R::SD_MEE => 1,
            PD8R::SD_SOE => 2,
            PD8R::MD => 4,
            PD8R::WD => 7,
            PD8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD8R {
        match value {
            0 => PD8R::SD_SHE,
            1 => PD8R::SD_MEE,
            2 => PD8R::SD_SOE,
            4 => PD8R::MD,
            7 => PD8R::WD,
            i => PD8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD8R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD8R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD8R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD8R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD8R::WD
    }
}
#[doc = "Possible values of the field `PD9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD9R {
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
impl PD9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD9R::SD_SHE => 0,
            PD9R::SD_MEE => 1,
            PD9R::SD_SOE => 2,
            PD9R::MD => 4,
            PD9R::WD => 7,
            PD9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD9R {
        match value {
            0 => PD9R::SD_SHE,
            1 => PD9R::SD_MEE,
            2 => PD9R::SD_SOE,
            4 => PD9R::MD,
            7 => PD9R::WD,
            i => PD9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD9R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD9R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD9R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD9R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD9R::WD
    }
}
#[doc = "Possible values of the field `PD10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD10R {
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
impl PD10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD10R::SD_SHE => 0,
            PD10R::SD_MEE => 1,
            PD10R::SD_SOE => 2,
            PD10R::MD => 4,
            PD10R::WD => 7,
            PD10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD10R {
        match value {
            0 => PD10R::SD_SHE,
            1 => PD10R::SD_MEE,
            2 => PD10R::SD_SOE,
            4 => PD10R::MD,
            7 => PD10R::WD,
            i => PD10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD10R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD10R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD10R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD10R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD10R::WD
    }
}
#[doc = "Possible values of the field `PD11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD11R {
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
impl PD11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD11R::SD_SHE => 0,
            PD11R::SD_MEE => 1,
            PD11R::SD_SOE => 2,
            PD11R::MD => 4,
            PD11R::WD => 7,
            PD11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD11R {
        match value {
            0 => PD11R::SD_SHE,
            1 => PD11R::SD_MEE,
            2 => PD11R::SD_SOE,
            4 => PD11R::MD,
            7 => PD11R::WD,
            i => PD11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD11R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD11R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD11R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD11R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD11R::WD
    }
}
#[doc = "Possible values of the field `PD12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD12R {
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
impl PD12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD12R::SD_SHE => 0,
            PD12R::SD_MEE => 1,
            PD12R::SD_SOE => 2,
            PD12R::MD => 4,
            PD12R::WD => 7,
            PD12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD12R {
        match value {
            0 => PD12R::SD_SHE,
            1 => PD12R::SD_MEE,
            2 => PD12R::SD_SOE,
            4 => PD12R::MD,
            7 => PD12R::WD,
            i => PD12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD12R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD12R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD12R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD12R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD12R::WD
    }
}
#[doc = "Possible values of the field `PD13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD13R {
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
impl PD13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD13R::SD_SHE => 0,
            PD13R::SD_MEE => 1,
            PD13R::SD_SOE => 2,
            PD13R::MD => 4,
            PD13R::WD => 7,
            PD13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD13R {
        match value {
            0 => PD13R::SD_SHE,
            1 => PD13R::SD_MEE,
            2 => PD13R::SD_SOE,
            4 => PD13R::MD,
            7 => PD13R::WD,
            i => PD13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD13R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD13R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD13R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD13R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD13R::WD
    }
}
#[doc = "Possible values of the field `PD14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD14R {
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
impl PD14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD14R::SD_SHE => 0,
            PD14R::SD_MEE => 1,
            PD14R::SD_SOE => 2,
            PD14R::MD => 4,
            PD14R::WD => 7,
            PD14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD14R {
        match value {
            0 => PD14R::SD_SHE,
            1 => PD14R::SD_MEE,
            2 => PD14R::SD_SOE,
            4 => PD14R::MD,
            7 => PD14R::WD,
            i => PD14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD14R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD14R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD14R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD14R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD14R::WD
    }
}
#[doc = "Possible values of the field `PD15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD15R {
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
impl PD15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PD15R::SD_SHE => 0,
            PD15R::SD_MEE => 1,
            PD15R::SD_SOE => 2,
            PD15R::MD => 4,
            PD15R::WD => 7,
            PD15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PD15R {
        match value {
            0 => PD15R::SD_SHE,
            1 => PD15R::SD_MEE,
            2 => PD15R::SD_SOE,
            4 => PD15R::MD,
            7 => PD15R::WD,
            i => PD15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SD_SHE`"]
    #[inline]
    pub fn is_sd_she(&self) -> bool {
        *self == PD15R::SD_SHE
    }
    #[doc = "Checks if the value of the field is `SD_MEE`"]
    #[inline]
    pub fn is_sd_mee(&self) -> bool {
        *self == PD15R::SD_MEE
    }
    #[doc = "Checks if the value of the field is `SD_SOE`"]
    #[inline]
    pub fn is_sd_soe(&self) -> bool {
        *self == PD15R::SD_SOE
    }
    #[doc = "Checks if the value of the field is `MD`"]
    #[inline]
    pub fn is_md(&self) -> bool {
        *self == PD15R::MD
    }
    #[doc = "Checks if the value of the field is `WD`"]
    #[inline]
    pub fn is_wd(&self) -> bool {
        *self == PD15R::WD
    }
}
#[doc = "Values that can be written to the field `PD8`"]
pub enum PD8W {
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
impl PD8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD8W::SD_SHE => 0,
            PD8W::SD_MEE => 1,
            PD8W::SD_SOE => 2,
            PD8W::MD => 4,
            PD8W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD8W<'a> {
    w: &'a mut W,
}
impl<'a> _PD8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD8W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD8W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD8W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD8W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD8W::WD)
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
#[doc = "Values that can be written to the field `PD9`"]
pub enum PD9W {
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
impl PD9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD9W::SD_SHE => 0,
            PD9W::SD_MEE => 1,
            PD9W::SD_SOE => 2,
            PD9W::MD => 4,
            PD9W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD9W<'a> {
    w: &'a mut W,
}
impl<'a> _PD9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD9W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD9W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD9W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD9W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD9W::WD)
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
#[doc = "Values that can be written to the field `PD10`"]
pub enum PD10W {
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
impl PD10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD10W::SD_SHE => 0,
            PD10W::SD_MEE => 1,
            PD10W::SD_SOE => 2,
            PD10W::MD => 4,
            PD10W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD10W<'a> {
    w: &'a mut W,
}
impl<'a> _PD10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD10W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD10W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD10W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD10W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD10W::WD)
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
#[doc = "Values that can be written to the field `PD11`"]
pub enum PD11W {
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
impl PD11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD11W::SD_SHE => 0,
            PD11W::SD_MEE => 1,
            PD11W::SD_SOE => 2,
            PD11W::MD => 4,
            PD11W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD11W<'a> {
    w: &'a mut W,
}
impl<'a> _PD11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD11W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD11W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD11W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD11W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD11W::WD)
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
#[doc = "Values that can be written to the field `PD12`"]
pub enum PD12W {
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
impl PD12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD12W::SD_SHE => 0,
            PD12W::SD_MEE => 1,
            PD12W::SD_SOE => 2,
            PD12W::MD => 4,
            PD12W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD12W<'a> {
    w: &'a mut W,
}
impl<'a> _PD12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD12W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD12W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD12W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD12W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD12W::WD)
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
#[doc = "Values that can be written to the field `PD13`"]
pub enum PD13W {
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
impl PD13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD13W::SD_SHE => 0,
            PD13W::SD_MEE => 1,
            PD13W::SD_SOE => 2,
            PD13W::MD => 4,
            PD13W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD13W<'a> {
    w: &'a mut W,
}
impl<'a> _PD13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD13W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD13W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD13W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD13W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD13W::WD)
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
#[doc = "Values that can be written to the field `PD14`"]
pub enum PD14W {
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
impl PD14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD14W::SD_SHE => 0,
            PD14W::SD_MEE => 1,
            PD14W::SD_SOE => 2,
            PD14W::MD => 4,
            PD14W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD14W<'a> {
    w: &'a mut W,
}
impl<'a> _PD14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD14W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD14W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD14W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD14W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD14W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PD15`"]
pub enum PD15W {
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
impl PD15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PD15W::SD_SHE => 0,
            PD15W::SD_MEE => 1,
            PD15W::SD_SOE => 2,
            PD15W::MD => 4,
            PD15W::WD => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PD15W<'a> {
    w: &'a mut W,
}
impl<'a> _PD15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PD15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "A2 strong driver, sharp edge"]
    #[inline]
    pub fn sd_she(self) -> &'a mut W {
        self.variant(PD15W::SD_SHE)
    }
    #[doc = "A2 strong driver, medium edge"]
    #[inline]
    pub fn sd_mee(self) -> &'a mut W {
        self.variant(PD15W::SD_MEE)
    }
    #[doc = "A2 strong driver, soft edge"]
    #[inline]
    pub fn sd_soe(self) -> &'a mut W {
        self.variant(PD15W::SD_SOE)
    }
    #[doc = "A2 medium driver"]
    #[inline]
    pub fn md(self) -> &'a mut W {
        self.variant(PD15W::MD)
    }
    #[doc = "A2 weak driver"]
    #[inline]
    pub fn wd(self) -> &'a mut W {
        self.variant(PD15W::WD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline]
    pub fn pd8(&self) -> PD8R {
        PD8R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline]
    pub fn pd9(&self) -> PD9R {
        PD9R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline]
    pub fn pd10(&self) -> PD10R {
        PD10R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline]
    pub fn pd11(&self) -> PD11R {
        PD11R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline]
    pub fn pd12(&self) -> PD12R {
        PD12R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline]
    pub fn pd13(&self) -> PD13R {
        PD13R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline]
    pub fn pd14(&self) -> PD14R {
        PD14R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline]
    pub fn pd15(&self) -> PD15R {
        PD15R::_from({
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
    #[doc = "Bits 0:2 - Pad Driver Mode for Pn.8"]
    #[inline]
    pub fn pd8(&mut self) -> _PD8W {
        _PD8W { w: self }
    }
    #[doc = "Bits 4:6 - Pad Driver Mode for Pn.9"]
    #[inline]
    pub fn pd9(&mut self) -> _PD9W {
        _PD9W { w: self }
    }
    #[doc = "Bits 8:10 - Pad Driver Mode for Pn.10"]
    #[inline]
    pub fn pd10(&mut self) -> _PD10W {
        _PD10W { w: self }
    }
    #[doc = "Bits 12:14 - Pad Driver Mode for Pn.11"]
    #[inline]
    pub fn pd11(&mut self) -> _PD11W {
        _PD11W { w: self }
    }
    #[doc = "Bits 16:18 - Pad Driver Mode for Pn.12"]
    #[inline]
    pub fn pd12(&mut self) -> _PD12W {
        _PD12W { w: self }
    }
    #[doc = "Bits 20:22 - Pad Driver Mode for Pn.13"]
    #[inline]
    pub fn pd13(&mut self) -> _PD13W {
        _PD13W { w: self }
    }
    #[doc = "Bits 24:26 - Pad Driver Mode for Pn.14"]
    #[inline]
    pub fn pd14(&mut self) -> _PD14W {
        _PD14W { w: self }
    }
    #[doc = "Bits 28:30 - Pad Driver Mode for Pn.15"]
    #[inline]
    pub fn pd15(&mut self) -> _PD15W {
        _PD15W { w: self }
    }
}
