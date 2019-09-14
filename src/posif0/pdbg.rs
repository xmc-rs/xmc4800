#[doc = "Reader of register PDBG"]
pub type R = crate::R<u32, super::PDBG>;
#[doc = "Reader of field `QCSV`"]
pub type QCSV_R = crate::R<u8, u8>;
#[doc = "Reader of field `QPSV`"]
pub type QPSV_R = crate::R<u8, u8>;
#[doc = "Reader of field `IVAL`"]
pub type IVAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSP`"]
pub type HSP_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPP0`"]
pub type LPP0_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPP1`"]
pub type LPP1_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPP2`"]
pub type LPP2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Quadrature Decoder Current state"]
    #[inline(always)]
    pub fn qcsv(&self) -> QCSV_R {
        QCSV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Quadrature Decoder Previous state"]
    #[inline(always)]
    pub fn qpsv(&self) -> QPSV_R {
        QPSV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Current Index Value"]
    #[inline(always)]
    pub fn ival(&self) -> IVAL_R {
        IVAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Hall Current Sampled Pattern"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:13 - Actual count of the Low Pass Filter for POSI0"]
    #[inline(always)]
    pub fn lpp0(&self) -> LPP0_R {
        LPP0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Actual count of the Low Pass Filter for POSI1"]
    #[inline(always)]
    pub fn lpp1(&self) -> LPP1_R {
        LPP1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - Actual count of the Low Pass Filter for POSI2"]
    #[inline(always)]
    pub fn lpp2(&self) -> LPP2_R {
        LPP2_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
