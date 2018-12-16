#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::RAM_SIZE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct RAM_SIZER {
    bits: u8,
}
impl RAM_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
    #[inline]
    pub fn ram_size(&self) -> RAM_SIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        RAM_SIZER { bits }
    }
}
