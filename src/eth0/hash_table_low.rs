#[doc = "Reader of register HASH_TABLE_LOW"]
pub type R = crate::R<u32, super::HASH_TABLE_LOW>;
#[doc = "Writer for register HASH_TABLE_LOW"]
pub type W = crate::W<u32, super::HASH_TABLE_LOW>;
#[doc = "Register HASH_TABLE_LOW `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_TABLE_LOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HTL`"]
pub type HTL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HTL`"]
pub struct HTL_W<'a> {
    w: &'a mut W,
}
impl<'a> HTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Table Low"]
    #[inline(always)]
    pub fn htl(&mut self) -> HTL_W {
        HTL_W { w: self }
    }
}
