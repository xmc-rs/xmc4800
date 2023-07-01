#[doc = "Register `DOEPTSIZ_ISO` reader"]
pub struct R(crate::R<DOEPTSIZ_ISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ_ISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ_ISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ_ISO` writer"]
pub struct W(crate::W<DOEPTSIZ_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DOEPTSIZ_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ_ISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XferSize` reader - Transfer Size"]
pub type XFER_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XferSize` writer - Transfer Size"]
pub type XFER_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPTSIZ_ISO_SPEC, 19, O, u32>;
#[doc = "Field `PktCnt` reader - Packet Count"]
pub type PKT_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `PktCnt` writer - Packet Count"]
pub type PKT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPTSIZ_ISO_SPEC, 10, O, u16>;
#[doc = "Field `RxDPID` reader - Received Data PID"]
pub type RX_DPID_R = crate::FieldReader<RX_DPID_A>;
#[doc = "Received Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_DPID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA2"]
    VALUE2 = 1,
    #[doc = "2: DATA1"]
    VALUE3 = 2,
    #[doc = "3: MDATA"]
    VALUE4 = 3,
}
impl From<RX_DPID_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_DPID_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_DPID_A {
    type Ux = u8;
}
impl RX_DPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DPID_A {
        match self.bits {
            0 => RX_DPID_A::VALUE1,
            1 => RX_DPID_A::VALUE2,
            2 => RX_DPID_A::VALUE3,
            3 => RX_DPID_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_DPID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_DPID_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_DPID_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_DPID_A::VALUE4
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfer_size(&self) -> XFER_SIZE_R {
        XFER_SIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pkt_cnt(&self) -> PKT_CNT_R {
        PKT_CNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Received Data PID"]
    #[inline(always)]
    pub fn rx_dpid(&self) -> RX_DPID_R {
        RX_DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_size(&mut self) -> XFER_SIZE_W<0> {
        XFER_SIZE_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pkt_cnt(&mut self) -> PKT_CNT_W<19> {
        PKT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Transfer Size Register \\[ISO\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz_iso](index.html) module"]
pub struct DOEPTSIZ_ISO_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz_iso::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ_ISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz_iso::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ_ISO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ_ISO to value 0"]
impl crate::Resettable for DOEPTSIZ_ISO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
