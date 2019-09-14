#[doc = "Reader of register MAC_ADDRESS2_LOW"]
pub type R = crate::R<u32, super::MAC_ADDRESS2_LOW>;
#[doc = "Writer for register MAC_ADDRESS2_LOW"]
pub type W = crate::W<u32, super::MAC_ADDRESS2_LOW>;
#[doc = "Register MAC_ADDRESS2_LOW `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MAC_ADDRESS2_LOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ADDRLO`"]
pub type ADDRLO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDRLO`"]
pub struct ADDRLO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MAC Address2 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address2 \\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&mut self) -> ADDRLO_W {
        ADDRLO_W { w: self }
    }
}
