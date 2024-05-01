#[doc = "Register `RXIPV6_NO_PAYLOAD_OCTETS` reader"]
pub type R = crate::R<Rxipv6NoPayloadOctetsSpec>;
#[doc = "Field `RXIPV6NOPAYOCT` reader - This field indicates the number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 headers Length field is used to update this counter."]
pub type Rxipv6nopayoctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv6nopayoct(&self) -> Rxipv6nopayoctR {
        Rxipv6nopayoctR::new(self.bits)
    }
}
#[doc = "Receive IPV6 No Payload Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_no_payload_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv6NoPayloadOctetsSpec;
impl crate::RegisterSpec for Rxipv6NoPayloadOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_no_payload_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv6NoPayloadOctetsSpec {}
#[doc = "`reset()` method sets RXIPV6_NO_PAYLOAD_OCTETS to value 0"]
impl crate::Resettable for Rxipv6NoPayloadOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
