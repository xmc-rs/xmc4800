#[doc = "Register `MODCFG` reader"]
pub type R = crate::R<ModcfgSpec>;
#[doc = "Register `MODCFG` writer"]
pub type W = crate::W<ModcfgSpec>;
#[doc = "Divider Factor for Modulator Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divm {
    #[doc = "0: fMOD = fCLK / 2"]
    Value1 = 0,
    #[doc = "1: fMOD = fCLK / 4"]
    Value2 = 1,
    #[doc = "2: fMOD = fCLK / 6"]
    Value3 = 2,
    #[doc = "15: fMOD = fCLK / 32"]
    Value4 = 15,
}
impl From<Divm> for u8 {
    #[inline(always)]
    fn from(variant: Divm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divm {
    type Ux = u8;
}
#[doc = "Field `DIVM` reader - Divider Factor for Modulator Clock"]
pub type DivmR = crate::FieldReader<Divm>;
impl DivmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divm> {
        match self.bits {
            0 => Some(Divm::Value1),
            1 => Some(Divm::Value2),
            2 => Some(Divm::Value3),
            15 => Some(Divm::Value4),
            _ => None,
        }
    }
    #[doc = "fMOD = fCLK / 2"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Divm::Value1
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Divm::Value2
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Divm::Value3
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Divm::Value4
    }
}
#[doc = "Field `DIVM` writer - Divider Factor for Modulator Clock"]
pub type DivmW<'a, REG> = crate::FieldWriter<'a, REG, 4, Divm>;
impl<'a, REG> DivmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fMOD = fCLK / 2"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Value1)
    }
    #[doc = "fMOD = fCLK / 4"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Value2)
    }
    #[doc = "fMOD = fCLK / 6"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Value3)
    }
    #[doc = "fMOD = fCLK / 32"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Divm::Value4)
    }
}
#[doc = "Write Control for Divider Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dwc {
    #[doc = "0: No write access to divider factor"]
    Value1 = 0,
    #[doc = "1: Bitfield DIVM can be written"]
    Value2 = 1,
}
impl From<Dwc> for bool {
    #[inline(always)]
    fn from(variant: Dwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DWC` writer - Write Control for Divider Factor"]
pub type DwcW<'a, REG> = crate::BitWriter<'a, REG, Dwc>;
impl<'a, REG> DwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to divider factor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dwc::Value1)
    }
    #[doc = "Bitfield DIVM can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dwc::Value2)
    }
}
impl R {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    pub fn divm(&self) -> DivmR {
        DivmR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Divider Factor for Modulator Clock"]
    #[inline(always)]
    #[must_use]
    pub fn divm(&mut self) -> DivmW<ModcfgSpec> {
        DivmW::new(self, 16)
    }
    #[doc = "Bit 23 - Write Control for Divider Factor"]
    #[inline(always)]
    #[must_use]
    pub fn dwc(&mut self) -> DwcW<ModcfgSpec> {
        DwcW::new(self, 23)
    }
}
#[doc = "Modulator Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModcfgSpec;
impl crate::RegisterSpec for ModcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modcfg::R`](R) reader structure"]
impl crate::Readable for ModcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`modcfg::W`](W) writer structure"]
impl crate::Writable for ModcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODCFG to value 0"]
impl crate::Resettable for ModcfgSpec {
    const RESET_VALUE: u32 = 0;
}
