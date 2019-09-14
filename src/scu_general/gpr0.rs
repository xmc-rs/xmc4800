#[doc = "Reader of register GPR0"]
pub type R = crate::R<u32, super::GPR0>;
#[doc = "Writer for register GPR0"]
pub type W = crate::W<u32, super::GPR0>;
#[doc = "Register GPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT`"]
pub type DAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DAT`"]
pub struct DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - User Data"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W { w: self }
    }
}
