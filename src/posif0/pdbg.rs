#[doc = "Register `PDBG` reader"]
pub type R = crate::R<PdbgSpec>;
#[doc = "Field `QCSV` reader - Quadrature Decoder Current state"]
pub type QcsvR = crate::FieldReader;
#[doc = "Field `QPSV` reader - Quadrature Decoder Previous state"]
pub type QpsvR = crate::FieldReader;
#[doc = "Field `IVAL` reader - Current Index Value"]
pub type IvalR = crate::BitReader;
#[doc = "Field `HSP` reader - Hall Current Sampled Pattern"]
pub type HspR = crate::FieldReader;
#[doc = "Field `LPP0` reader - Actual count of the Low Pass Filter for POSI0"]
pub type Lpp0R = crate::FieldReader;
#[doc = "Field `LPP1` reader - Actual count of the Low Pass Filter for POSI1"]
pub type Lpp1R = crate::FieldReader;
#[doc = "Field `LPP2` reader - Actual count of the Low Pass Filter for POSI2"]
pub type Lpp2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Quadrature Decoder Current state"]
    #[inline(always)]
    pub fn qcsv(&self) -> QcsvR {
        QcsvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Quadrature Decoder Previous state"]
    #[inline(always)]
    pub fn qpsv(&self) -> QpsvR {
        QpsvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Current Index Value"]
    #[inline(always)]
    pub fn ival(&self) -> IvalR {
        IvalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Hall Current Sampled Pattern"]
    #[inline(always)]
    pub fn hsp(&self) -> HspR {
        HspR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:13 - Actual count of the Low Pass Filter for POSI0"]
    #[inline(always)]
    pub fn lpp0(&self) -> Lpp0R {
        Lpp0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Actual count of the Low Pass Filter for POSI1"]
    #[inline(always)]
    pub fn lpp1(&self) -> Lpp1R {
        Lpp1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - Actual count of the Low Pass Filter for POSI2"]
    #[inline(always)]
    pub fn lpp2(&self) -> Lpp2R {
        Lpp2R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[doc = "POSIF Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdbg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbgSpec;
impl crate::RegisterSpec for PdbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdbg::R`](R) reader structure"]
impl crate::Readable for PdbgSpec {}
#[doc = "`reset()` method sets PDBG to value 0"]
impl crate::Resettable for PdbgSpec {
    const RESET_VALUE: u32 = 0;
}
