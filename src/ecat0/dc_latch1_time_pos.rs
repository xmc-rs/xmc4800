#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC_LATCH1_TIME_POS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DC_LATCH1_TIME_POSR {
    bits: u32,
}
impl DC_LATCH1_TIME_POSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Captures System time at the positive edge of the Latch1 signal"]
    #[inline]
    pub fn dc_latch1_time_pos(&self) -> DC_LATCH1_TIME_POSR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        DC_LATCH1_TIME_POSR { bits }
    }
}