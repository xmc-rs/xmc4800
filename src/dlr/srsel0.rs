#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRSEL0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct RS0R {
    bits: u8,
}
impl RS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS1R {
    bits: u8,
}
impl RS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS2R {
    bits: u8,
}
impl RS2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS3R {
    bits: u8,
}
impl RS3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS4R {
    bits: u8,
}
impl RS4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS5R {
    bits: u8,
}
impl RS5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS6R {
    bits: u8,
}
impl RS6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RS7R {
    bits: u8,
}
impl RS7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RS0W<'a> {
    w: &'a mut W,
}
impl<'a> _RS0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS1W<'a> {
    w: &'a mut W,
}
impl<'a> _RS1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS2W<'a> {
    w: &'a mut W,
}
impl<'a> _RS2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS3W<'a> {
    w: &'a mut W,
}
impl<'a> _RS3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS4W<'a> {
    w: &'a mut W,
}
impl<'a> _RS4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS5W<'a> {
    w: &'a mut W,
}
impl<'a> _RS5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS6W<'a> {
    w: &'a mut W,
}
impl<'a> _RS6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RS7W<'a> {
    w: &'a mut W,
}
impl<'a> _RS7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline]
    pub fn rs0(&self) -> RS0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS0R { bits }
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline]
    pub fn rs1(&self) -> RS1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS1R { bits }
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline]
    pub fn rs2(&self) -> RS2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS2R { bits }
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline]
    pub fn rs3(&self) -> RS3R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS3R { bits }
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline]
    pub fn rs4(&self) -> RS4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS4R { bits }
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline]
    pub fn rs5(&self) -> RS5R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS5R { bits }
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline]
    pub fn rs6(&self) -> RS6R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS6R { bits }
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline]
    pub fn rs7(&self) -> RS7R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RS7R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Request Source for Line 0"]
    #[inline]
    pub fn rs0(&mut self) -> _RS0W {
        _RS0W { w: self }
    }
    #[doc = "Bits 4:7 - Request Source for Line 1"]
    #[inline]
    pub fn rs1(&mut self) -> _RS1W {
        _RS1W { w: self }
    }
    #[doc = "Bits 8:11 - Request Source for Line 2"]
    #[inline]
    pub fn rs2(&mut self) -> _RS2W {
        _RS2W { w: self }
    }
    #[doc = "Bits 12:15 - Request Source for Line 3"]
    #[inline]
    pub fn rs3(&mut self) -> _RS3W {
        _RS3W { w: self }
    }
    #[doc = "Bits 16:19 - Request Source for Line 4"]
    #[inline]
    pub fn rs4(&mut self) -> _RS4W {
        _RS4W { w: self }
    }
    #[doc = "Bits 20:23 - Request Source for Line 5"]
    #[inline]
    pub fn rs5(&mut self) -> _RS5W {
        _RS5W { w: self }
    }
    #[doc = "Bits 24:27 - Request Source for Line 6"]
    #[inline]
    pub fn rs6(&mut self) -> _RS6W {
        _RS6W { w: self }
    }
    #[doc = "Bits 28:31 - Request Source for Line 7"]
    #[inline]
    pub fn rs7(&mut self) -> _RS7W {
        _RS7W { w: self }
    }
}
