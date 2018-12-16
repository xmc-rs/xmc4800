#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PDBG {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct QCSVR {
    bits: u8,
}
impl QCSVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct QPSVR {
    bits: u8,
}
impl QPSVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IVALR {
    bits: bool,
}
impl IVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct HSPR {
    bits: u8,
}
impl HSPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPP0R {
    bits: u8,
}
impl LPP0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPP1R {
    bits: u8,
}
impl LPP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPP2R {
    bits: u8,
}
impl LPP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Quadrature Decoder Current state"]
    #[inline]
    pub fn qcsv(&self) -> QCSVR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        QCSVR { bits }
    }
    #[doc = "Bits 2:3 - Quadrature Decoder Previous state"]
    #[inline]
    pub fn qpsv(&self) -> QPSVR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        QPSVR { bits }
    }
    #[doc = "Bit 4 - Current Index Value"]
    #[inline]
    pub fn ival(&self) -> IVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IVALR { bits }
    }
    #[doc = "Bits 5:7 - Hall Current Sampled Pattern"]
    #[inline]
    pub fn hsp(&self) -> HSPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSPR { bits }
    }
    #[doc = "Bits 8:13 - Actual count of the Low Pass Filter for POSI0"]
    #[inline]
    pub fn lpp0(&self) -> LPP0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPP0R { bits }
    }
    #[doc = "Bits 16:21 - Actual count of the Low Pass Filter for POSI1"]
    #[inline]
    pub fn lpp1(&self) -> LPP1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPP1R { bits }
    }
    #[doc = "Bits 22:27 - Actual count of the Low Pass Filter for POSI2"]
    #[inline]
    pub fn lpp2(&self) -> LPP2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPP2R { bits }
    }
}
