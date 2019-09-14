#[doc = "Reader of register DTSSTAT"]
pub type R = crate::R<u32, super::DTSSTAT>;
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Sensor Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDY_A {
    #[doc = "0: The DTS is not ready"]
    VALUE1,
    #[doc = "1: The DTS is ready"]
    VALUE2,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        match variant {
            RDY_A::VALUE1 => false,
            RDY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, RDY_A>;
impl RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::VALUE1,
            true => RDY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RDY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RDY_A::VALUE2
    }
}
#[doc = "Sensor Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: not busy"]
    VALUE1,
    #[doc = "1: busy"]
    VALUE2,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::VALUE1 => false,
            BUSY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:9 - Result of the DTS Measurement"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Sensor Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Sensor Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
