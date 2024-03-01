#[doc = "Register `USBPLLCON` reader"]
pub type R = crate::R<UsbpllconSpec>;
#[doc = "Register `USBPLLCON` writer"]
pub type W = crate::W<UsbpllconSpec>;
#[doc = "VCO Bypass\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcobyp {
    #[doc = "0: Normal operation, VCO is not bypassed"]
    Value1 = 0,
    #[doc = "1: Prescaler Mode, VCO is bypassed"]
    Value2 = 1,
}
impl From<Vcobyp> for bool {
    #[inline(always)]
    fn from(variant: Vcobyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOBYP` reader - VCO Bypass"]
pub type VcobypR = crate::BitReader<Vcobyp>;
impl VcobypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcobyp {
        match self.bits {
            false => Vcobyp::Value1,
            true => Vcobyp::Value2,
        }
    }
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcobyp::Value1
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcobyp::Value2
    }
}
#[doc = "Field `VCOBYP` writer - VCO Bypass"]
pub type VcobypW<'a, REG> = crate::BitWriter<'a, REG, Vcobyp>;
impl<'a, REG> VcobypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, VCO is not bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcobyp::Value1)
    }
    #[doc = "Prescaler Mode, VCO is bypassed"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vcobyp::Value2)
    }
}
#[doc = "VCO Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcopwd {
    #[doc = "0: Normal behavior"]
    Value1 = 0,
    #[doc = "1: The VCO is put into a Power Saving Mode"]
    Value2 = 1,
}
impl From<Vcopwd> for bool {
    #[inline(always)]
    fn from(variant: Vcopwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPWD` reader - VCO Power Saving Mode"]
pub type VcopwdR = crate::BitReader<Vcopwd>;
impl VcopwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcopwd {
        match self.bits {
            false => Vcopwd::Value1,
            true => Vcopwd::Value2,
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcopwd::Value1
    }
    #[doc = "The VCO is put into a Power Saving Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcopwd::Value2
    }
}
#[doc = "Field `VCOPWD` writer - VCO Power Saving Mode"]
pub type VcopwdW<'a, REG> = crate::BitWriter<'a, REG, Vcopwd>;
impl<'a, REG> VcopwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopwd::Value1)
    }
    #[doc = "The VCO is put into a Power Saving Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vcopwd::Value2)
    }
}
#[doc = "VCO Trim Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcotr {
    #[doc = "0: VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    Value1 = 0,
    #[doc = "1: VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    Value2 = 1,
}
impl From<Vcotr> for bool {
    #[inline(always)]
    fn from(variant: Vcotr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOTR` reader - VCO Trim Control"]
pub type VcotrR = crate::BitReader<Vcotr>;
impl VcotrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vcotr {
        match self.bits {
            false => Vcotr::Value1,
            true => Vcotr::Value2,
        }
    }
    #[doc = "VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vcotr::Value1
    }
    #[doc = "VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vcotr::Value2
    }
}
#[doc = "Field `VCOTR` writer - VCO Trim Control"]
pub type VcotrW<'a, REG> = crate::BitWriter<'a, REG, Vcotr>;
impl<'a, REG> VcotrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCO bandwidth is operating in the normal range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcotr::Value1)
    }
    #[doc = "VCO bandwidth is operating in the test range. VCO output frequency is between 260 and 520 MHz for a input frequency between 8 and 16 MHz."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vcotr::Value2)
    }
}
#[doc = "Disconnect Oscillator from VCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Findis {
    #[doc = "0: Connect oscillator to the VCO part"]
    Value1 = 0,
    #[doc = "1: Disconnect oscillator from the VCO part."]
    Value2 = 1,
}
impl From<Findis> for bool {
    #[inline(always)]
    fn from(variant: Findis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FINDIS` reader - Disconnect Oscillator from VCO"]
pub type FindisR = crate::BitReader<Findis>;
impl FindisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Findis {
        match self.bits {
            false => Findis::Value1,
            true => Findis::Value2,
        }
    }
    #[doc = "Connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Findis::Value1
    }
    #[doc = "Disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Findis::Value2
    }
}
#[doc = "Field `FINDIS` writer - Disconnect Oscillator from VCO"]
pub type FindisW<'a, REG> = crate::BitWriter<'a, REG, Findis>;
impl<'a, REG> FindisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect oscillator to the VCO part"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Findis::Value1)
    }
    #[doc = "Disconnect oscillator from the VCO part."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Findis::Value2)
    }
}
#[doc = "Oscillator Disconnect Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscdiscdis {
    #[doc = "0: In case of a PLL loss-of-lock bit FINDIS is set"]
    Value1 = 0,
    #[doc = "1: In case of a PLL loss-of-lock bit FINDIS is cleared"]
    Value2 = 1,
}
impl From<Oscdiscdis> for bool {
    #[inline(always)]
    fn from(variant: Oscdiscdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCDISCDIS` reader - Oscillator Disconnect Disable"]
pub type OscdiscdisR = crate::BitReader<Oscdiscdis>;
impl OscdiscdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscdiscdis {
        match self.bits {
            false => Oscdiscdis::Value1,
            true => Oscdiscdis::Value2,
        }
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Oscdiscdis::Value1
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Oscdiscdis::Value2
    }
}
#[doc = "Field `OSCDISCDIS` writer - Oscillator Disconnect Disable"]
pub type OscdiscdisW<'a, REG> = crate::BitWriter<'a, REG, Oscdiscdis>;
impl<'a, REG> OscdiscdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is set"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscdiscdis::Value1)
    }
    #[doc = "In case of a PLL loss-of-lock bit FINDIS is cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Oscdiscdis::Value2)
    }
}
#[doc = "Field `NDIV` reader - N-Divider Value"]
pub type NdivR = crate::FieldReader;
#[doc = "Field `NDIV` writer - N-Divider Value"]
pub type NdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "PLL Power Saving Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllpwd {
    #[doc = "0: Normal behavior"]
    Value1 = 0,
    #[doc = "1: The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected."]
    Value2 = 1,
}
impl From<Pllpwd> for bool {
    #[inline(always)]
    fn from(variant: Pllpwd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPWD` reader - PLL Power Saving Mode"]
pub type PllpwdR = crate::BitReader<Pllpwd>;
impl PllpwdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllpwd {
        match self.bits {
            false => Pllpwd::Value1,
            true => Pllpwd::Value2,
        }
    }
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pllpwd::Value1
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pllpwd::Value2
    }
}
#[doc = "Field `PLLPWD` writer - PLL Power Saving Mode"]
pub type PllpwdW<'a, REG> = crate::BitWriter<'a, REG, Pllpwd>;
impl<'a, REG> PllpwdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal behavior"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpwd::Value1)
    }
    #[doc = "The complete PLL block is put into a Power Saving Mode. Only the Bypass Mode is active if previously selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pllpwd::Value2)
    }
}
#[doc = "Field `RESLD` writer - Restart VCO Lock Detection"]
pub type ResldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIV` reader - P-Divider Value"]
pub type PdivR = crate::FieldReader;
#[doc = "Field `PDIV` writer - P-Divider Value"]
pub type PdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    pub fn vcobyp(&self) -> VcobypR {
        VcobypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    pub fn vcopwd(&self) -> VcopwdR {
        VcopwdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    pub fn vcotr(&self) -> VcotrR {
        VcotrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    pub fn findis(&self) -> FindisR {
        FindisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    pub fn oscdiscdis(&self) -> OscdiscdisR {
        OscdiscdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    pub fn ndiv(&self) -> NdivR {
        NdivR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    pub fn pllpwd(&self) -> PllpwdR {
        PllpwdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    pub fn pdiv(&self) -> PdivR {
        PdivR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VCO Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn vcobyp(&mut self) -> VcobypW<UsbpllconSpec> {
        VcobypW::new(self, 0)
    }
    #[doc = "Bit 1 - VCO Power Saving Mode"]
    #[inline(always)]
    #[must_use]
    pub fn vcopwd(&mut self) -> VcopwdW<UsbpllconSpec> {
        VcopwdW::new(self, 1)
    }
    #[doc = "Bit 2 - VCO Trim Control"]
    #[inline(always)]
    #[must_use]
    pub fn vcotr(&mut self) -> VcotrW<UsbpllconSpec> {
        VcotrW::new(self, 2)
    }
    #[doc = "Bit 4 - Disconnect Oscillator from VCO"]
    #[inline(always)]
    #[must_use]
    pub fn findis(&mut self) -> FindisW<UsbpllconSpec> {
        FindisW::new(self, 4)
    }
    #[doc = "Bit 6 - Oscillator Disconnect Disable"]
    #[inline(always)]
    #[must_use]
    pub fn oscdiscdis(&mut self) -> OscdiscdisW<UsbpllconSpec> {
        OscdiscdisW::new(self, 6)
    }
    #[doc = "Bits 8:14 - N-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn ndiv(&mut self) -> NdivW<UsbpllconSpec> {
        NdivW::new(self, 8)
    }
    #[doc = "Bit 16 - PLL Power Saving Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwd(&mut self) -> PllpwdW<UsbpllconSpec> {
        PllpwdW::new(self, 16)
    }
    #[doc = "Bit 18 - Restart VCO Lock Detection"]
    #[inline(always)]
    #[must_use]
    pub fn resld(&mut self) -> ResldW<UsbpllconSpec> {
        ResldW::new(self, 18)
    }
    #[doc = "Bits 24:27 - P-Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn pdiv(&mut self) -> PdivW<UsbpllconSpec> {
        PdivW::new(self, 24)
    }
}
#[doc = "USB PLL Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbpllcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbpllcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbpllconSpec;
impl crate::RegisterSpec for UsbpllconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbpllcon::R`](R) reader structure"]
impl crate::Readable for UsbpllconSpec {}
#[doc = "`write(|w| ..)` method takes [`usbpllcon::W`](W) writer structure"]
impl crate::Writable for UsbpllconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPLLCON to value 0x0001_0003"]
impl crate::Resettable for UsbpllconSpec {
    const RESET_VALUE: u32 = 0x0001_0003;
}
