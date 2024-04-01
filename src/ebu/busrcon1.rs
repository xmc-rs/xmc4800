#[doc = "Register `BUSRCON1` reader"]
pub type R = crate::R<Busrcon1Spec>;
#[doc = "Register `BUSRCON1` writer"]
pub type W = crate::W<Busrcon1Spec>;
#[doc = "Burst Length for Synchronous Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fetblen {
    #[doc = "0: 1 data access (default after reset)."]
    Value1 = 0,
    #[doc = "1: 2 data accesses."]
    Value2 = 1,
    #[doc = "2: 4 data accesses."]
    Value3 = 2,
    #[doc = "3: 8 data accesses."]
    Value4 = 3,
}
impl From<Fetblen> for u8 {
    #[inline(always)]
    fn from(variant: Fetblen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fetblen {
    type Ux = u8;
}
impl crate::IsEnum for Fetblen {}
#[doc = "Field `FETBLEN` reader - Burst Length for Synchronous Burst"]
pub type FetblenR = crate::FieldReader<Fetblen>;
impl FetblenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fetblen> {
        match self.bits {
            0 => Some(Fetblen::Value1),
            1 => Some(Fetblen::Value2),
            2 => Some(Fetblen::Value3),
            3 => Some(Fetblen::Value4),
            _ => None,
        }
    }
    #[doc = "1 data access (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fetblen::Value1
    }
    #[doc = "2 data accesses."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fetblen::Value2
    }
    #[doc = "4 data accesses."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Fetblen::Value3
    }
    #[doc = "8 data accesses."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Fetblen::Value4
    }
}
#[doc = "Field `FETBLEN` writer - Burst Length for Synchronous Burst"]
pub type FetblenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fetblen>;
impl<'a, REG> FetblenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data access (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fetblen::Value1)
    }
    #[doc = "2 data accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fetblen::Value2)
    }
    #[doc = "4 data accesses."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Fetblen::Value3)
    }
    #[doc = "8 data accesses."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Fetblen::Value4)
    }
}
#[doc = "Synchronous burst buffer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fbbmsel {
    #[doc = "0: Burst buffer length defined by value in FETBLEN (default after reset)."]
    Value1 = 0,
    #[doc = "1: Continuous mode. All data required for transaction is transferred in a single burst."]
    Value2 = 1,
}
impl From<Fbbmsel> for bool {
    #[inline(always)]
    fn from(variant: Fbbmsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBBMSEL` reader - Synchronous burst buffer mode select"]
pub type FbbmselR = crate::BitReader<Fbbmsel>;
impl FbbmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fbbmsel {
        match self.bits {
            false => Fbbmsel::Value1,
            true => Fbbmsel::Value2,
        }
    }
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fbbmsel::Value1
    }
    #[doc = "Continuous mode. All data required for transaction is transferred in a single burst."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fbbmsel::Value2
    }
}
#[doc = "Field `FBBMSEL` writer - Synchronous burst buffer mode select"]
pub type FbbmselW<'a, REG> = crate::BitWriter<'a, REG, Fbbmsel>;
impl<'a, REG> FbbmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fbbmsel::Value1)
    }
    #[doc = "Continuous mode. All data required for transaction is transferred in a single burst."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fbbmsel::Value2)
    }
}
#[doc = "Read Single Stage Synchronization:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfsss {
    #[doc = "0: Two stages of synchronization used. (maximum margin)"]
    Value1 = 0,
    #[doc = "1: One stage of synchronization used. (minimum latency)"]
    Value2 = 1,
}
impl From<Bfsss> for bool {
    #[inline(always)]
    fn from(variant: Bfsss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFSSS` reader - Read Single Stage Synchronization:"]
pub type BfsssR = crate::BitReader<Bfsss>;
impl BfsssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfsss {
        match self.bits {
            false => Bfsss::Value1,
            true => Bfsss::Value2,
        }
    }
    #[doc = "Two stages of synchronization used. (maximum margin)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfsss::Value1
    }
    #[doc = "One stage of synchronization used. (minimum latency)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfsss::Value2
    }
}
#[doc = "Field `BFSSS` writer - Read Single Stage Synchronization:"]
pub type BfsssW<'a, REG> = crate::BitWriter<'a, REG, Bfsss>;
impl<'a, REG> BfsssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Two stages of synchronization used. (maximum margin)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfsss::Value1)
    }
    #[doc = "One stage of synchronization used. (minimum latency)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfsss::Value2)
    }
}
#[doc = "Burst FLASH Clock Feedback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdbken {
    #[doc = "0: BFCLK feedback not used."]
    Value1 = 0,
    #[doc = "1: Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    Value2 = 1,
}
impl From<Fdbken> for bool {
    #[inline(always)]
    fn from(variant: Fdbken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDBKEN` reader - Burst FLASH Clock Feedback Enable"]
pub type FdbkenR = crate::BitReader<Fdbken>;
impl FdbkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdbken {
        match self.bits {
            false => Fdbken::Value1,
            true => Fdbken::Value2,
        }
    }
    #[doc = "BFCLK feedback not used."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fdbken::Value1
    }
    #[doc = "Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fdbken::Value2
    }
}
#[doc = "Field `FDBKEN` writer - Burst FLASH Clock Feedback Enable"]
pub type FdbkenW<'a, REG> = crate::BitWriter<'a, REG, Fdbken>;
impl<'a, REG> FdbkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BFCLK feedback not used."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdbken::Value1)
    }
    #[doc = "Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fdbken::Value2)
    }
}
#[doc = "Burst Flash Clock Mode Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfcmsel {
    #[doc = "0: Burst Flash Clock runs continuously with values selected by this register"]
    Value1 = 0,
    #[doc = "1: Burst Flash Clock is disabled between accesses"]
    Value2 = 1,
}
impl From<Bfcmsel> for bool {
    #[inline(always)]
    fn from(variant: Bfcmsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFCMSEL` reader - Burst Flash Clock Mode Select"]
pub type BfcmselR = crate::BitReader<Bfcmsel>;
impl BfcmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfcmsel {
        match self.bits {
            false => Bfcmsel::Value1,
            true => Bfcmsel::Value2,
        }
    }
    #[doc = "Burst Flash Clock runs continuously with values selected by this register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bfcmsel::Value1
    }
    #[doc = "Burst Flash Clock is disabled between accesses"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bfcmsel::Value2
    }
}
#[doc = "Field `BFCMSEL` writer - Burst Flash Clock Mode Select"]
pub type BfcmselW<'a, REG> = crate::BitWriter<'a, REG, Bfcmsel>;
impl<'a, REG> BfcmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst Flash Clock runs continuously with values selected by this register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfcmsel::Value1)
    }
    #[doc = "Burst Flash Clock is disabled between accesses"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bfcmsel::Value2)
    }
}
#[doc = "Field `NAA` reader - Enable flash non-array access workaround"]
pub type NaaR = crate::BitReader;
#[doc = "Field `NAA` writer - Enable flash non-array access workaround"]
pub type NaaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Early Chip Select for Synchronous Burst\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecse {
    #[doc = "0: CS is delayed."]
    Value1 = 0,
    #[doc = "1: CS is not delayed."]
    Value2 = 1,
}
impl From<Ecse> for bool {
    #[inline(always)]
    fn from(variant: Ecse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECSE` reader - Early Chip Select for Synchronous Burst"]
pub type EcseR = crate::BitReader<Ecse>;
impl EcseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecse {
        match self.bits {
            false => Ecse::Value1,
            true => Ecse::Value2,
        }
    }
    #[doc = "CS is delayed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ecse::Value1
    }
    #[doc = "CS is not delayed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ecse::Value2
    }
}
#[doc = "Field `ECSE` writer - Early Chip Select for Synchronous Burst"]
pub type EcseW<'a, REG> = crate::BitWriter<'a, REG, Ecse>;
impl<'a, REG> EcseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CS is delayed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ecse::Value1)
    }
    #[doc = "CS is not delayed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ecse::Value2)
    }
}
#[doc = "Early Burst Signal Enable for Synchronous Burst\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ebse {
    #[doc = "0: ADV is delayed."]
    Value1 = 0,
    #[doc = "1: ADV is not delayed."]
    Value2 = 1,
}
impl From<Ebse> for bool {
    #[inline(always)]
    fn from(variant: Ebse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EBSE` reader - Early Burst Signal Enable for Synchronous Burst"]
pub type EbseR = crate::BitReader<Ebse>;
impl EbseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ebse {
        match self.bits {
            false => Ebse::Value1,
            true => Ebse::Value2,
        }
    }
    #[doc = "ADV is delayed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ebse::Value1
    }
    #[doc = "ADV is not delayed."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ebse::Value2
    }
}
#[doc = "Field `EBSE` writer - Early Burst Signal Enable for Synchronous Burst"]
pub type EbseW<'a, REG> = crate::BitWriter<'a, REG, Ebse>;
impl<'a, REG> EbseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADV is delayed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ebse::Value1)
    }
    #[doc = "ADV is not delayed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ebse::Value2)
    }
}
#[doc = "Disable Burst Address Wrapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dba {
    #[doc = "0: Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    Value1 = 0,
    #[doc = "1: Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    Value2 = 1,
}
impl From<Dba> for bool {
    #[inline(always)]
    fn from(variant: Dba) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBA` reader - Disable Burst Address Wrapping"]
pub type DbaR = crate::BitReader<Dba>;
impl DbaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dba {
        match self.bits {
            false => Dba::Value1,
            true => Dba::Value2,
        }
    }
    #[doc = "Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dba::Value1
    }
    #[doc = "Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dba::Value2
    }
}
#[doc = "Field `DBA` writer - Disable Burst Address Wrapping"]
pub type DbaW<'a, REG> = crate::BitWriter<'a, REG, Dba>;
impl<'a, REG> DbaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dba::Value1)
    }
    #[doc = "Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dba::Value2)
    }
}
#[doc = "Reversed polarity at WAIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waitinv {
    #[doc = "0: input at WAIT pin is active low (default after reset)."]
    Value1 = 0,
    #[doc = "1: input at WAIT pin is active high."]
    Value2 = 1,
}
impl From<Waitinv> for bool {
    #[inline(always)]
    fn from(variant: Waitinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITINV` reader - Reversed polarity at WAIT"]
pub type WaitinvR = crate::BitReader<Waitinv>;
impl WaitinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Waitinv {
        match self.bits {
            false => Waitinv::Value1,
            true => Waitinv::Value2,
        }
    }
    #[doc = "input at WAIT pin is active low (default after reset)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Waitinv::Value1
    }
    #[doc = "input at WAIT pin is active high."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Waitinv::Value2
    }
}
#[doc = "Field `WAITINV` writer - Reversed polarity at WAIT"]
pub type WaitinvW<'a, REG> = crate::BitWriter<'a, REG, Waitinv>;
impl<'a, REG> WaitinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "input at WAIT pin is active low (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Waitinv::Value1)
    }
    #[doc = "input at WAIT pin is active high."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Waitinv::Value2)
    }
}
#[doc = "Byte Control Signal Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bcgen {
    #[doc = "0: Byte control signals follow chip select timing."]
    Value1 = 0,
    #[doc = "1: Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    Value2 = 1,
    #[doc = "2: Byte control signals follow write enable signal timing (RD/WR only)."]
    Value3 = 2,
}
impl From<Bcgen> for u8 {
    #[inline(always)]
    fn from(variant: Bcgen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bcgen {
    type Ux = u8;
}
impl crate::IsEnum for Bcgen {}
#[doc = "Field `BCGEN` reader - Byte Control Signal Control"]
pub type BcgenR = crate::FieldReader<Bcgen>;
impl BcgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bcgen> {
        match self.bits {
            0 => Some(Bcgen::Value1),
            1 => Some(Bcgen::Value2),
            2 => Some(Bcgen::Value3),
            _ => None,
        }
    }
    #[doc = "Byte control signals follow chip select timing."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bcgen::Value1
    }
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bcgen::Value2
    }
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bcgen::Value3
    }
}
#[doc = "Field `BCGEN` writer - Byte Control Signal Control"]
pub type BcgenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bcgen>;
impl<'a, REG> BcgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte control signals follow chip select timing."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcgen::Value1)
    }
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bcgen::Value2)
    }
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bcgen::Value3)
    }
}
#[doc = "Field `PORTW` reader - Device Addressing Mode"]
pub type PortwR = crate::FieldReader;
#[doc = "Field `PORTW` writer - Device Addressing Mode"]
pub type PortwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAIT` reader - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
pub type WaitR = crate::FieldReader;
#[doc = "Field `WAIT` writer - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
pub type WaitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Asynchronous Address phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aap {
    #[doc = "0: Clock is enabled at beginning of access."]
    Value1 = 0,
    #[doc = "1: Clock is enabled at after address phase."]
    Value2 = 1,
}
impl From<Aap> for bool {
    #[inline(always)]
    fn from(variant: Aap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AAP` reader - Asynchronous Address phase:"]
pub type AapR = crate::BitReader<Aap>;
impl AapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aap {
        match self.bits {
            false => Aap::Value1,
            true => Aap::Value2,
        }
    }
    #[doc = "Clock is enabled at beginning of access."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Aap::Value1
    }
    #[doc = "Clock is enabled at after address phase."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Aap::Value2
    }
}
#[doc = "Field `AAP` writer - Asynchronous Address phase:"]
pub type AapW<'a, REG> = crate::BitWriter<'a, REG, Aap>;
impl<'a, REG> AapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock is enabled at beginning of access."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Aap::Value1)
    }
    #[doc = "Clock is enabled at after address phase."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Aap::Value2)
    }
}
#[doc = "Field `AGEN` reader - Device Type for Region"]
pub type AgenR = crate::FieldReader;
#[doc = "Field `AGEN` writer - Device Type for Region"]
pub type AgenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline(always)]
    pub fn fetblen(&self) -> FetblenR {
        FetblenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline(always)]
    pub fn fbbmsel(&self) -> FbbmselR {
        FbbmselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Single Stage Synchronization:"]
    #[inline(always)]
    pub fn bfsss(&self) -> BfsssR {
        BfsssR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Burst FLASH Clock Feedback Enable"]
    #[inline(always)]
    pub fn fdbken(&self) -> FdbkenR {
        FdbkenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Burst Flash Clock Mode Select"]
    #[inline(always)]
    pub fn bfcmsel(&self) -> BfcmselR {
        BfcmselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline(always)]
    pub fn naa(&self) -> NaaR {
        NaaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline(always)]
    pub fn ecse(&self) -> EcseR {
        EcseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline(always)]
    pub fn ebse(&self) -> EbseR {
        EbseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Burst Address Wrapping"]
    #[inline(always)]
    pub fn dba(&self) -> DbaR {
        DbaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline(always)]
    pub fn waitinv(&self) -> WaitinvR {
        WaitinvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline(always)]
    pub fn bcgen(&self) -> BcgenR {
        BcgenR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline(always)]
    pub fn portw(&self) -> PortwR {
        PortwR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline(always)]
    pub fn wait(&self) -> WaitR {
        WaitR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline(always)]
    pub fn aap(&self) -> AapR {
        AapR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline(always)]
    pub fn agen(&self) -> AgenR {
        AgenR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline(always)]
    #[must_use]
    pub fn fetblen(&mut self) -> FetblenW<Busrcon1Spec> {
        FetblenW::new(self, 0)
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fbbmsel(&mut self) -> FbbmselW<Busrcon1Spec> {
        FbbmselW::new(self, 3)
    }
    #[doc = "Bit 4 - Read Single Stage Synchronization:"]
    #[inline(always)]
    #[must_use]
    pub fn bfsss(&mut self) -> BfsssW<Busrcon1Spec> {
        BfsssW::new(self, 4)
    }
    #[doc = "Bit 5 - Burst FLASH Clock Feedback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdbken(&mut self) -> FdbkenW<Busrcon1Spec> {
        FdbkenW::new(self, 5)
    }
    #[doc = "Bit 6 - Burst Flash Clock Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfcmsel(&mut self) -> BfcmselW<Busrcon1Spec> {
        BfcmselW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline(always)]
    #[must_use]
    pub fn naa(&mut self) -> NaaW<Busrcon1Spec> {
        NaaW::new(self, 7)
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline(always)]
    #[must_use]
    pub fn ecse(&mut self) -> EcseW<Busrcon1Spec> {
        EcseW::new(self, 16)
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline(always)]
    #[must_use]
    pub fn ebse(&mut self) -> EbseW<Busrcon1Spec> {
        EbseW::new(self, 17)
    }
    #[doc = "Bit 18 - Disable Burst Address Wrapping"]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DbaW<Busrcon1Spec> {
        DbaW::new(self, 18)
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline(always)]
    #[must_use]
    pub fn waitinv(&mut self) -> WaitinvW<Busrcon1Spec> {
        WaitinvW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline(always)]
    #[must_use]
    pub fn bcgen(&mut self) -> BcgenW<Busrcon1Spec> {
        BcgenW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn portw(&mut self) -> PortwW<Busrcon1Spec> {
        PortwW::new(self, 22)
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WaitW<Busrcon1Spec> {
        WaitW::new(self, 24)
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline(always)]
    #[must_use]
    pub fn aap(&mut self) -> AapW<Busrcon1Spec> {
        AapW::new(self, 26)
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline(always)]
    #[must_use]
    pub fn agen(&mut self) -> AgenW<Busrcon1Spec> {
        AgenW::new(self, 28)
    }
}
#[doc = "EBU Bus Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busrcon1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busrcon1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Busrcon1Spec;
impl crate::RegisterSpec for Busrcon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busrcon1::R`](R) reader structure"]
impl crate::Readable for Busrcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`busrcon1::W`](W) writer structure"]
impl crate::Writable for Busrcon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSRCON1 to value 0x00d3_0040"]
impl crate::Resettable for Busrcon1Spec {
    const RESET_VALUE: u32 = 0x00d3_0040;
}
