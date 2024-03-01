#[doc = "Register `CLC` reader"]
pub type R = crate::R<ClcSpec>;
#[doc = "Register `CLC` writer"]
pub type W = crate::W<ClcSpec>;
#[doc = "EBU Disable Request Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disr {
    #[doc = "0: EBU disable is not requested"]
    Value1 = 0,
    #[doc = "1: EBU disable is requested"]
    Value2 = 1,
}
impl From<Disr> for bool {
    #[inline(always)]
    fn from(variant: Disr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISR` reader - EBU Disable Request Bit"]
pub type DisrR = crate::BitReader<Disr>;
impl DisrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disr {
        match self.bits {
            false => Disr::Value1,
            true => Disr::Value2,
        }
    }
    #[doc = "EBU disable is not requested"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Disr::Value1
    }
    #[doc = "EBU disable is requested"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Disr::Value2
    }
}
#[doc = "Field `DISR` writer - EBU Disable Request Bit"]
pub type DisrW<'a, REG> = crate::BitWriter<'a, REG, Disr>;
impl<'a, REG> DisrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EBU disable is not requested"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Disr::Value1)
    }
    #[doc = "EBU disable is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Disr::Value2)
    }
}
#[doc = "EBU Disable Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diss {
    #[doc = "0: EBU is enabled (default after reset)"]
    Value1 = 0,
    #[doc = "1: EBU is disabled"]
    Value2 = 1,
}
impl From<Diss> for bool {
    #[inline(always)]
    fn from(variant: Diss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISS` reader - EBU Disable Status Bit"]
pub type DissR = crate::BitReader<Diss>;
impl DissR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diss {
        match self.bits {
            false => Diss::Value1,
            true => Diss::Value2,
        }
    }
    #[doc = "EBU is enabled (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Diss::Value1
    }
    #[doc = "EBU is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Diss::Value2
    }
}
#[doc = "EBU Clocking Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    Value1 = 0,
    #[doc = "1: request EBU to run synchronously to ARM processor (default after reset)"]
    Value2 = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - EBU Clocking Mode"]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Value1,
            true => Sync::Value2,
        }
    }
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sync::Value1
    }
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sync::Value2
    }
}
#[doc = "Field `SYNC` writer - EBU Clocking Mode"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, Sync>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "request EBU to run asynchronously to AHB bus clock and use separate clock source"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Value1)
    }
    #[doc = "request EBU to run synchronously to ARM processor (default after reset)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Value2)
    }
}
#[doc = "DIV2 Clocking Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Div2 {
    #[doc = "0: standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    Value1 = 0,
    #[doc = "1: request EBU to run off AHB bus clock divided by 2."]
    Value2 = 1,
}
impl From<Div2> for bool {
    #[inline(always)]
    fn from(variant: Div2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV2` reader - DIV2 Clocking Mode"]
pub type Div2R = crate::BitReader<Div2>;
impl Div2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div2 {
        match self.bits {
            false => Div2::Value1,
            true => Div2::Value2,
        }
    }
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Div2::Value1
    }
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Div2::Value2
    }
}
#[doc = "Field `DIV2` writer - DIV2 Clocking Mode"]
pub type Div2W<'a, REG> = crate::BitWriter<'a, REG, Div2>;
impl<'a, REG> Div2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Div2::Value1)
    }
    #[doc = "request EBU to run off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Div2::Value2)
    }
}
#[doc = "EBU Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ebudiv {
    #[doc = "0: request EBU to run off input clock (default after reset)"]
    Value1 = 0,
    #[doc = "1: request EBU to run off input clock divided by 2"]
    Value2 = 1,
    #[doc = "2: request EBU to run off input clock divided by 3"]
    Value3 = 2,
    #[doc = "3: request EBU to run off input clock divided by 4"]
    Value4 = 3,
}
impl From<Ebudiv> for u8 {
    #[inline(always)]
    fn from(variant: Ebudiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ebudiv {
    type Ux = u8;
}
#[doc = "Field `EBUDIV` reader - EBU Clock Divide Ratio"]
pub type EbudivR = crate::FieldReader<Ebudiv>;
impl EbudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ebudiv {
        match self.bits {
            0 => Ebudiv::Value1,
            1 => Ebudiv::Value2,
            2 => Ebudiv::Value3,
            3 => Ebudiv::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "request EBU to run off input clock (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ebudiv::Value1
    }
    #[doc = "request EBU to run off input clock divided by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ebudiv::Value2
    }
    #[doc = "request EBU to run off input clock divided by 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ebudiv::Value3
    }
    #[doc = "request EBU to run off input clock divided by 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ebudiv::Value4
    }
}
#[doc = "Field `EBUDIV` writer - EBU Clock Divide Ratio"]
pub type EbudivW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ebudiv>;
impl<'a, REG> EbudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "request EBU to run off input clock (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ebudiv::Value1)
    }
    #[doc = "request EBU to run off input clock divided by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ebudiv::Value2)
    }
    #[doc = "request EBU to run off input clock divided by 3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ebudiv::Value3)
    }
    #[doc = "request EBU to run off input clock divided by 4"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ebudiv::Value4)
    }
}
#[doc = "EBU Clocking Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syncack {
    #[doc = "0: the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    Value1 = 0,
    #[doc = "1: EBU is synchronous to the AHB bus clock (default after reset)"]
    Value2 = 1,
}
impl From<Syncack> for bool {
    #[inline(always)]
    fn from(variant: Syncack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCACK` reader - EBU Clocking Mode Status"]
pub type SyncackR = crate::BitReader<Syncack>;
impl SyncackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncack {
        match self.bits {
            false => Syncack::Value1,
            true => Syncack::Value2,
        }
    }
    #[doc = "the EBU is asynchronous to the AHB bus clock and is using a separate clock source"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Syncack::Value1
    }
    #[doc = "EBU is synchronous to the AHB bus clock (default after reset)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Syncack::Value2
    }
}
#[doc = "DIV2 Clocking Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Div2ack {
    #[doc = "0: EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    Value1 = 0,
    #[doc = "1: EBU is running off AHB bus clock divided by 2."]
    Value2 = 1,
}
impl From<Div2ack> for bool {
    #[inline(always)]
    fn from(variant: Div2ack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIV2ACK` reader - DIV2 Clocking Mode Status"]
pub type Div2ackR = crate::BitReader<Div2ack>;
impl Div2ackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Div2ack {
        match self.bits {
            false => Div2ack::Value1,
            true => Div2ack::Value2,
        }
    }
    #[doc = "EBU is using standard clocking mode. clock input selected by SYNC bitfield (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Div2ack::Value1
    }
    #[doc = "EBU is running off AHB bus clock divided by 2."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Div2ack::Value2
    }
}
#[doc = "EBU Clock Divide Ratio Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ebudivack {
    #[doc = "0: EBU is running off input clock (default after reset)"]
    Value1 = 0,
    #[doc = "1: EBU is running off input clock divided by 2"]
    Value2 = 1,
    #[doc = "2: EBU is running off input clock divided by 3"]
    Value3 = 2,
    #[doc = "3: EBU is running off input clock divided by 4"]
    Value4 = 3,
}
impl From<Ebudivack> for u8 {
    #[inline(always)]
    fn from(variant: Ebudivack) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ebudivack {
    type Ux = u8;
}
#[doc = "Field `EBUDIVACK` reader - EBU Clock Divide Ratio Status"]
pub type EbudivackR = crate::FieldReader<Ebudivack>;
impl EbudivackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ebudivack {
        match self.bits {
            0 => Ebudivack::Value1,
            1 => Ebudivack::Value2,
            2 => Ebudivack::Value3,
            3 => Ebudivack::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "EBU is running off input clock (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ebudivack::Value1
    }
    #[doc = "EBU is running off input clock divided by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ebudivack::Value2
    }
    #[doc = "EBU is running off input clock divided by 3"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ebudivack::Value3
    }
    #[doc = "EBU is running off input clock divided by 4"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ebudivack::Value4
    }
}
impl R {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DisrR {
        DisrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBU Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DissR {
        DissR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    pub fn div2(&self) -> Div2R {
        Div2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EbudivR {
        EbudivR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - EBU Clocking Mode Status"]
    #[inline(always)]
    pub fn syncack(&self) -> SyncackR {
        SyncackR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIV2 Clocking Mode Status"]
    #[inline(always)]
    pub fn div2ack(&self) -> Div2ackR {
        Div2ackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - EBU Clock Divide Ratio Status"]
    #[inline(always)]
    pub fn ebudivack(&self) -> EbudivackR {
        EbudivackR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EBU Disable Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn disr(&mut self) -> DisrW<ClcSpec> {
        DisrW::new(self, 0)
    }
    #[doc = "Bit 16 - EBU Clocking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<ClcSpec> {
        SyncW::new(self, 16)
    }
    #[doc = "Bit 17 - DIV2 Clocking Mode"]
    #[inline(always)]
    #[must_use]
    pub fn div2(&mut self) -> Div2W<ClcSpec> {
        Div2W::new(self, 17)
    }
    #[doc = "Bits 18:19 - EBU Clock Divide Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ebudiv(&mut self) -> EbudivW<ClcSpec> {
        EbudivW::new(self, 18)
    }
}
#[doc = "EBU Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClcSpec;
impl crate::RegisterSpec for ClcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clc::R`](R) reader structure"]
impl crate::Readable for ClcSpec {}
#[doc = "`write(|w| ..)` method takes [`clc::W`](W) writer structure"]
impl crate::Writable for ClcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLC to value 0x0011_0000"]
impl crate::Resettable for ClcSpec {
    const RESET_VALUE: u32 = 0x0011_0000;
}
