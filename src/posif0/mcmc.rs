#[doc = "Writer for register MCMC"]
pub type W = crate::W<u32, super::MCMC>;
#[doc = "Register MCMC `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MNPC`"]
pub struct MNPC_W<'a> {
    w: &'a mut W,
}
impl<'a> MNPC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `MPC`"]
pub struct MPC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Multi-Channel Pattern Update Enable Clear"]
    #[inline(always)]
    pub fn mnpc(&mut self) -> MNPC_W {
        MNPC_W { w: self }
    }
    #[doc = "Bit 1 - Multi-Channel Pattern clear"]
    #[inline(always)]
    pub fn mpc(&mut self) -> MPC_W {
        MPC_W { w: self }
    }
}
