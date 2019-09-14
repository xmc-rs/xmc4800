#[doc = "Reader of register TIMESTAMP_CONTROL"]
pub type R = crate::R<u32, super::TIMESTAMP_CONTROL>;
#[doc = "Writer for register TIMESTAMP_CONTROL"]
pub type W = crate::W<u32, super::TIMESTAMP_CONTROL>;
#[doc = "Register TIMESTAMP_CONTROL `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::TIMESTAMP_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Reader of field `TSENA`"]
pub type TSENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENA`"]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
#[doc = "Reader of field `TSCFUPDT`"]
pub type TSCFUPDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCFUPDT`"]
pub struct TSCFUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCFUPDT_W<'a> {
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
#[doc = "Reader of field `TSINIT`"]
pub type TSINIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSINIT`"]
pub struct TSINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TSUPDT`"]
pub type TSUPDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUPDT`"]
pub struct TSUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TSTRIG`"]
pub type TSTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTRIG`"]
pub struct TSTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTRIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TSADDREG`"]
pub type TSADDREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSADDREG`"]
pub struct TSADDREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TSADDREG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TSENALL`"]
pub type TSENALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENALL`"]
pub struct TSENALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TSCTRLSSR`"]
pub type TSCTRLSSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCTRLSSR`"]
pub struct TSCTRLSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRLSSR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TSVER2ENA`"]
pub type TSVER2ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSVER2ENA`"]
pub struct TSVER2ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVER2ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TSIPENA`"]
pub type TSIPENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSIPENA`"]
pub struct TSIPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TSIPV6ENA`"]
pub type TSIPV6ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSIPV6ENA`"]
pub struct TSIPV6ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV6ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TSIPV4ENA`"]
pub type TSIPV4ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSIPV4ENA`"]
pub struct TSIPV4ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV4ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TSEVNTENA`"]
pub type TSEVNTENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSEVNTENA`"]
pub struct TSEVNTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEVNTENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TSMSTRENA`"]
pub type TSMSTRENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSMSTRENA`"]
pub struct TSMSTRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSMSTRENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SNAPTYPSEL`"]
pub type SNAPTYPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SNAPTYPSEL`"]
pub struct SNAPTYPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAPTYPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `TSENMACADDR`"]
pub type TSENMACADDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSENMACADDR`"]
pub struct TSENMACADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENMACADDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W {
        TSCFUPDT_W { w: self }
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W {
        TSINIT_W { w: self }
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W {
        TSUPDT_W { w: self }
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&mut self) -> TSTRIG_W {
        TSTRIG_W { w: self }
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddreg(&mut self) -> TSADDREG_W {
        TSADDREG_W { w: self }
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W {
        TSENALL_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W {
        TSCTRLSSR_W { w: self }
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W {
        TSVER2ENA_W { w: self }
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W {
        TSIPENA_W { w: self }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent Over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W {
        TSIPV6ENA_W { w: self }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W {
        TSIPV4ENA_W { w: self }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W {
        TSEVNTENA_W { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W {
        TSMSTRENA_W { w: self }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W {
        SNAPTYPSEL_W { w: self }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W {
        TSENMACADDR_W { w: self }
    }
}
