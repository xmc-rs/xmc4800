#[doc = "Register `CLEARERR` writer"]
pub type W = crate::W<ClearerrSpec>;
#[doc = "Clear Interrupt Status and Raw Status for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Clear Interrupt Status and Raw Status for channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Clear Interrupt Status and Raw Status for channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Clear Interrupt Status and Raw Status for channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Clear Interrupt Status and Raw Status for channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` writer - Clear Interrupt Status and Raw Status for channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG, Ch4>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch5> for bool {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` writer - Clear Interrupt Status and Raw Status for channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG, Ch5>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch6> for bool {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` writer - Clear Interrupt Status and Raw Status for channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG, Ch6>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Value2)
    }
}
#[doc = "Clear Interrupt Status and Raw Status for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7 {
    #[doc = "0: no effect"]
    Value1 = 0,
    #[doc = "1: clear status"]
    Value2 = 1,
}
impl From<Ch7> for bool {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` writer - Clear Interrupt Status and Raw Status for channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG, Ch7>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Value1)
    }
    #[doc = "clear status"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt Status and Raw Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ClearerrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Interrupt Status and Raw Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ClearerrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Interrupt Status and Raw Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<ClearerrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Interrupt Status and Raw Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<ClearerrSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Interrupt Status and Raw Status for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<ClearerrSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Interrupt Status and Raw Status for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<ClearerrSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Interrupt Status and Raw Status for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<ClearerrSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Interrupt Status and Raw Status for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<ClearerrSpec> {
        Ch7W::new(self, 7)
    }
}
#[doc = "IntErr Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearerr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearerrSpec;
impl crate::RegisterSpec for ClearerrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clearerr::W`](W) writer structure"]
impl crate::Writable for ClearerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARERR to value 0"]
impl crate::Resettable for ClearerrSpec {
    const RESET_VALUE: u32 = 0;
}
