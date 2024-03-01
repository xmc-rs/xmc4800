#[doc = "Register `OVRCLR` writer"]
pub type W = crate::W<OvrclrSpec>;
#[doc = "Field `LN0` writer - Line 0 Overrun Status Clear"]
pub type Ln0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN1` writer - Line 1 Overrun Status Clear"]
pub type Ln1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN2` writer - Line 2 Overrun Status Clear"]
pub type Ln2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN3` writer - Line 3 Overrun Status Clear"]
pub type Ln3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN4` writer - Line 4 Overrun Status Clear"]
pub type Ln4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN5` writer - Line 5 Overrun Status Clear"]
pub type Ln5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN6` writer - Line 6 Overrun Status Clear"]
pub type Ln6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN7` writer - Line 7 Overrun Status Clear"]
pub type Ln7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN8` writer - Line 8 Overrun Status Clear"]
pub type Ln8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN9` writer - Line 9 Overrun Status Clear"]
pub type Ln9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN10` writer - Line 10 Overrun Status Clear"]
pub type Ln10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LN11` writer - Line 11 Overrun Status Clear"]
pub type Ln11W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Line 0 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln0(&mut self) -> Ln0W<OvrclrSpec> {
        Ln0W::new(self, 0)
    }
    #[doc = "Bit 1 - Line 1 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln1(&mut self) -> Ln1W<OvrclrSpec> {
        Ln1W::new(self, 1)
    }
    #[doc = "Bit 2 - Line 2 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln2(&mut self) -> Ln2W<OvrclrSpec> {
        Ln2W::new(self, 2)
    }
    #[doc = "Bit 3 - Line 3 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln3(&mut self) -> Ln3W<OvrclrSpec> {
        Ln3W::new(self, 3)
    }
    #[doc = "Bit 4 - Line 4 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln4(&mut self) -> Ln4W<OvrclrSpec> {
        Ln4W::new(self, 4)
    }
    #[doc = "Bit 5 - Line 5 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln5(&mut self) -> Ln5W<OvrclrSpec> {
        Ln5W::new(self, 5)
    }
    #[doc = "Bit 6 - Line 6 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln6(&mut self) -> Ln6W<OvrclrSpec> {
        Ln6W::new(self, 6)
    }
    #[doc = "Bit 7 - Line 7 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln7(&mut self) -> Ln7W<OvrclrSpec> {
        Ln7W::new(self, 7)
    }
    #[doc = "Bit 8 - Line 8 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln8(&mut self) -> Ln8W<OvrclrSpec> {
        Ln8W::new(self, 8)
    }
    #[doc = "Bit 9 - Line 9 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln9(&mut self) -> Ln9W<OvrclrSpec> {
        Ln9W::new(self, 9)
    }
    #[doc = "Bit 10 - Line 10 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln10(&mut self) -> Ln10W<OvrclrSpec> {
        Ln10W::new(self, 10)
    }
    #[doc = "Bit 11 - Line 11 Overrun Status Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ln11(&mut self) -> Ln11W<OvrclrSpec> {
        Ln11W::new(self, 11)
    }
}
#[doc = "Overrun Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvrclrSpec;
impl crate::RegisterSpec for OvrclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ovrclr::W`](W) writer structure"]
impl crate::Writable for OvrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVRCLR to value 0"]
impl crate::Resettable for OvrclrSpec {
    const RESET_VALUE: u32 = 0;
}
