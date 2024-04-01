#[doc = "Register `CLKMXSTAT` reader"]
pub type R = crate::R<ClkmxstatSpec>;
#[doc = "Status of System Clock Multiplexing Upon Source Switching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sysclkmux {
    #[doc = "1: fOFI clock active"]
    Value1 = 1,
    #[doc = "2: fPLL clock active"]
    Value2 = 2,
}
impl From<Sysclkmux> for u8 {
    #[inline(always)]
    fn from(variant: Sysclkmux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysclkmux {
    type Ux = u8;
}
impl crate::IsEnum for Sysclkmux {}
#[doc = "Field `SYSCLKMUX` reader - Status of System Clock Multiplexing Upon Source Switching"]
pub type SysclkmuxR = crate::FieldReader<Sysclkmux>;
impl SysclkmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysclkmux> {
        match self.bits {
            1 => Some(Sysclkmux::Value1),
            2 => Some(Sysclkmux::Value2),
            _ => None,
        }
    }
    #[doc = "fOFI clock active"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sysclkmux::Value1
    }
    #[doc = "fPLL clock active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sysclkmux::Value2
    }
}
impl R {
    #[doc = "Bits 0:1 - Status of System Clock Multiplexing Upon Source Switching"]
    #[inline(always)]
    pub fn sysclkmux(&self) -> SysclkmuxR {
        SysclkmuxR::new((self.bits & 3) as u8)
    }
}
#[doc = "Clock Multiplexing Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkmxstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkmxstatSpec;
impl crate::RegisterSpec for ClkmxstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkmxstat::R`](R) reader structure"]
impl crate::Readable for ClkmxstatSpec {}
#[doc = "`reset()` method sets CLKMXSTAT to value 0"]
impl crate::Resettable for ClkmxstatSpec {
    const RESET_VALUE: u32 = 0;
}
