#[doc = "Register `SDRMCON` reader"]
pub type R = crate::R<SDRMCON_SPEC>;
#[doc = "Register `SDRMCON` writer"]
pub type W = crate::W<SDRMCON_SPEC>;
#[doc = "Field `CRAS` reader - Row to precharge delay counter"]
pub type CRAS_R = crate::FieldReader;
#[doc = "Field `CRAS` writer - Row to precharge delay counter"]
pub type CRAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRFSH` reader - Initialization refresh commands counter"]
pub type CRFSH_R = crate::FieldReader;
#[doc = "Field `CRFSH` writer - Initialization refresh commands counter"]
pub type CRFSH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRSC` reader - Mode register set-up time"]
pub type CRSC_R = crate::FieldReader;
#[doc = "Field `CRSC` writer - Mode register set-up time"]
pub type CRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRP` reader - Row precharge time counter"]
pub type CRP_R = crate::FieldReader;
#[doc = "Field `CRP` writer - Row precharge time counter"]
pub type CRP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Width of column address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWIDTH_A {
    #[doc = "0: do not use"]
    VALUE1 = 0,
    #[doc = "1: Address(8:0)"]
    VALUE2 = 1,
    #[doc = "2: Address(9:0)"]
    VALUE3 = 2,
    #[doc = "3: Address(10:0)"]
    VALUE4 = 3,
}
impl From<AWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: AWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWIDTH_A {
    type Ux = u8;
}
impl crate::IsEnum for AWIDTH_A {}
#[doc = "Field `AWIDTH` reader - Width of column address"]
pub type AWIDTH_R = crate::FieldReader<AWIDTH_A>;
impl AWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWIDTH_A {
        match self.bits {
            0 => AWIDTH_A::VALUE1,
            1 => AWIDTH_A::VALUE2,
            2 => AWIDTH_A::VALUE3,
            3 => AWIDTH_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "do not use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AWIDTH_A::VALUE1
    }
    #[doc = "Address(8:0)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AWIDTH_A::VALUE2
    }
    #[doc = "Address(9:0)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == AWIDTH_A::VALUE3
    }
    #[doc = "Address(10:0)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == AWIDTH_A::VALUE4
    }
}
#[doc = "Field `AWIDTH` writer - Width of column address"]
pub type AWIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, AWIDTH_A, crate::Safe>;
impl<'a, REG> AWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "do not use"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AWIDTH_A::VALUE1)
    }
    #[doc = "Address(8:0)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AWIDTH_A::VALUE2)
    }
    #[doc = "Address(9:0)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(AWIDTH_A::VALUE3)
    }
    #[doc = "Address(10:0)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(AWIDTH_A::VALUE4)
    }
}
#[doc = "Field `CRCD` reader - Row to column delay counter"]
pub type CRCD_R = crate::FieldReader;
#[doc = "Field `CRCD` writer - Row to column delay counter"]
pub type CRCD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRC` reader - Row cycle time counter"]
pub type CRC_R = crate::FieldReader;
#[doc = "Field `CRC` writer - Row cycle time counter"]
pub type CRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Mask for row tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ROWM_A {
    #[doc = "1: Address bit 26 to 9"]
    VALUE2 = 1,
    #[doc = "2: Address bit 26 to 10"]
    VALUE3 = 2,
    #[doc = "3: Address bit 26 to 11"]
    VALUE4 = 3,
    #[doc = "4: Address bit 26 to 12"]
    VALUE5 = 4,
    #[doc = "5: Address bit 26 to 13"]
    VALUE6 = 5,
}
impl From<ROWM_A> for u8 {
    #[inline(always)]
    fn from(variant: ROWM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ROWM_A {
    type Ux = u8;
}
impl crate::IsEnum for ROWM_A {}
#[doc = "Field `ROWM` reader - Mask for row tag"]
pub type ROWM_R = crate::FieldReader<ROWM_A>;
impl ROWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ROWM_A> {
        match self.bits {
            1 => Some(ROWM_A::VALUE2),
            2 => Some(ROWM_A::VALUE3),
            3 => Some(ROWM_A::VALUE4),
            4 => Some(ROWM_A::VALUE5),
            5 => Some(ROWM_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Address bit 26 to 9"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ROWM_A::VALUE2
    }
    #[doc = "Address bit 26 to 10"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ROWM_A::VALUE3
    }
    #[doc = "Address bit 26 to 11"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ROWM_A::VALUE4
    }
    #[doc = "Address bit 26 to 12"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ROWM_A::VALUE5
    }
    #[doc = "Address bit 26 to 13"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ROWM_A::VALUE6
    }
}
#[doc = "Field `ROWM` writer - Mask for row tag"]
pub type ROWM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ROWM_A>;
impl<'a, REG> ROWM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address bit 26 to 9"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ROWM_A::VALUE2)
    }
    #[doc = "Address bit 26 to 10"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ROWM_A::VALUE3)
    }
    #[doc = "Address bit 26 to 11"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ROWM_A::VALUE4)
    }
    #[doc = "Address bit 26 to 12"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(ROWM_A::VALUE5)
    }
    #[doc = "Address bit 26 to 13"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(ROWM_A::VALUE6)
    }
}
#[doc = "Mask for bank tag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BANKM_A {
    #[doc = "1: Address bit 21 to 20"]
    VALUE2 = 1,
    #[doc = "2: Address bit 22 to 21"]
    VALUE3 = 2,
    #[doc = "3: Address bit 23 to 22"]
    VALUE4 = 3,
    #[doc = "4: Address bit 24 to 23"]
    VALUE5 = 4,
    #[doc = "5: Address bit 25 to 24"]
    VALUE6 = 5,
    #[doc = "6: Address bit 26 to 25"]
    VALUE7 = 6,
    #[doc = "7: Address bit 26"]
    VALUE8 = 7,
}
impl From<BANKM_A> for u8 {
    #[inline(always)]
    fn from(variant: BANKM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BANKM_A {
    type Ux = u8;
}
impl crate::IsEnum for BANKM_A {}
#[doc = "Field `BANKM` reader - Mask for bank tag"]
pub type BANKM_R = crate::FieldReader<BANKM_A>;
impl BANKM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BANKM_A> {
        match self.bits {
            1 => Some(BANKM_A::VALUE2),
            2 => Some(BANKM_A::VALUE3),
            3 => Some(BANKM_A::VALUE4),
            4 => Some(BANKM_A::VALUE5),
            5 => Some(BANKM_A::VALUE6),
            6 => Some(BANKM_A::VALUE7),
            7 => Some(BANKM_A::VALUE8),
            _ => None,
        }
    }
    #[doc = "Address bit 21 to 20"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BANKM_A::VALUE2
    }
    #[doc = "Address bit 22 to 21"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BANKM_A::VALUE3
    }
    #[doc = "Address bit 23 to 22"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == BANKM_A::VALUE4
    }
    #[doc = "Address bit 24 to 23"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == BANKM_A::VALUE5
    }
    #[doc = "Address bit 25 to 24"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == BANKM_A::VALUE6
    }
    #[doc = "Address bit 26 to 25"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == BANKM_A::VALUE7
    }
    #[doc = "Address bit 26"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == BANKM_A::VALUE8
    }
}
#[doc = "Field `BANKM` writer - Mask for bank tag"]
pub type BANKM_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BANKM_A>;
impl<'a, REG> BANKM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address bit 21 to 20"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE2)
    }
    #[doc = "Address bit 22 to 21"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE3)
    }
    #[doc = "Address bit 23 to 22"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE4)
    }
    #[doc = "Address bit 24 to 23"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE5)
    }
    #[doc = "Address bit 25 to 24"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE6)
    }
    #[doc = "Address bit 26 to 25"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE7)
    }
    #[doc = "Address bit 26"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(BANKM_A::VALUE8)
    }
}
#[doc = "Field `CRCE` reader - Row cycle time counter extension"]
pub type CRCE_R = crate::FieldReader;
#[doc = "Field `CRCE` writer - Row cycle time counter extension"]
pub type CRCE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Disable SDRAM clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKDIS_A {
    #[doc = "0: clock enabled"]
    VALUE1 = 0,
    #[doc = "1: clock disabled"]
    VALUE2 = 1,
}
impl From<CLKDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKDIS` reader - Disable SDRAM clock output"]
pub type CLKDIS_R = crate::BitReader<CLKDIS_A>;
impl CLKDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKDIS_A {
        match self.bits {
            false => CLKDIS_A::VALUE1,
            true => CLKDIS_A::VALUE2,
        }
    }
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKDIS_A::VALUE1
    }
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKDIS_A::VALUE2
    }
}
#[doc = "Field `CLKDIS` writer - Disable SDRAM clock output"]
pub type CLKDIS_W<'a, REG> = crate::BitWriter<'a, REG, CLKDIS_A>;
impl<'a, REG> CLKDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clock enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIS_A::VALUE1)
    }
    #[doc = "clock disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIS_A::VALUE2)
    }
}
#[doc = "Power Save Mode used for gated clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWR_MODE_A {
    #[doc = "0: precharge before clock stop (default after reset)"]
    VALUE1 = 0,
    #[doc = "1: auto-precharge before clock stop"]
    VALUE2 = 1,
    #[doc = "2: active power down (stop clock without precharge)"]
    VALUE3 = 2,
    #[doc = "3: clock stop power down"]
    VALUE4 = 3,
}
impl From<PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWR_MODE_A {
    type Ux = u8;
}
impl crate::IsEnum for PWR_MODE_A {}
#[doc = "Field `PWR_MODE` reader - Power Save Mode used for gated clock mode"]
pub type PWR_MODE_R = crate::FieldReader<PWR_MODE_A>;
impl PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWR_MODE_A {
        match self.bits {
            0 => PWR_MODE_A::VALUE1,
            1 => PWR_MODE_A::VALUE2,
            2 => PWR_MODE_A::VALUE3,
            3 => PWR_MODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "precharge before clock stop (default after reset)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PWR_MODE_A::VALUE1
    }
    #[doc = "auto-precharge before clock stop"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PWR_MODE_A::VALUE2
    }
    #[doc = "active power down (stop clock without precharge)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PWR_MODE_A::VALUE3
    }
    #[doc = "clock stop power down"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PWR_MODE_A::VALUE4
    }
}
#[doc = "Field `PWR_MODE` writer - Power Save Mode used for gated clock mode"]
pub type PWR_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWR_MODE_A, crate::Safe>;
impl<'a, REG> PWR_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "precharge before clock stop (default after reset)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::VALUE1)
    }
    #[doc = "auto-precharge before clock stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::VALUE2)
    }
    #[doc = "active power down (stop clock without precharge)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::VALUE3)
    }
    #[doc = "clock stop power down"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(PWR_MODE_A::VALUE4)
    }
}
#[doc = "SDRAM clock mode select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDCMSEL_A {
    #[doc = "1: clock disabled between accesses"]
    VALUE1 = 1,
    #[doc = "0: clock continuously runs"]
    VALUE2 = 0,
}
impl From<SDCMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDCMSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDCMSEL` reader - SDRAM clock mode select"]
pub type SDCMSEL_R = crate::BitReader<SDCMSEL_A>;
impl SDCMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDCMSEL_A {
        match self.bits {
            true => SDCMSEL_A::VALUE1,
            false => SDCMSEL_A::VALUE2,
        }
    }
    #[doc = "clock disabled between accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDCMSEL_A::VALUE1
    }
    #[doc = "clock continuously runs"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDCMSEL_A::VALUE2
    }
}
#[doc = "Field `SDCMSEL` writer - SDRAM clock mode select"]
pub type SDCMSEL_W<'a, REG> = crate::BitWriter<'a, REG, SDCMSEL_A>;
impl<'a, REG> SDCMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clock disabled between accesses"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SDCMSEL_A::VALUE1)
    }
    #[doc = "clock continuously runs"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SDCMSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    pub fn cras(&self) -> CRAS_R {
        CRAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    pub fn crfsh(&self) -> CRFSH_R {
        CRFSH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    pub fn crsc(&self) -> CRSC_R {
        CRSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    pub fn crp(&self) -> CRP_R {
        CRP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    pub fn awidth(&self) -> AWIDTH_R {
        AWIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    pub fn crcd(&self) -> CRCD_R {
        CRCD_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    pub fn rowm(&self) -> ROWM_R {
        ROWM_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    pub fn bankm(&self) -> BANKM_R {
        BANKM_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    pub fn clkdis(&self) -> CLKDIS_R {
        CLKDIS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PWR_MODE_R {
        PWR_MODE_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    pub fn sdcmsel(&self) -> SDCMSEL_R {
        SDCMSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Row to precharge delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn cras(&mut self) -> CRAS_W<SDRMCON_SPEC> {
        CRAS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Initialization refresh commands counter"]
    #[inline(always)]
    #[must_use]
    pub fn crfsh(&mut self) -> CRFSH_W<SDRMCON_SPEC> {
        CRFSH_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Mode register set-up time"]
    #[inline(always)]
    #[must_use]
    pub fn crsc(&mut self) -> CRSC_W<SDRMCON_SPEC> {
        CRSC_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Row precharge time counter"]
    #[inline(always)]
    #[must_use]
    pub fn crp(&mut self) -> CRP_W<SDRMCON_SPEC> {
        CRP_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Width of column address"]
    #[inline(always)]
    #[must_use]
    pub fn awidth(&mut self) -> AWIDTH_W<SDRMCON_SPEC> {
        AWIDTH_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Row to column delay counter"]
    #[inline(always)]
    #[must_use]
    pub fn crcd(&mut self) -> CRCD_W<SDRMCON_SPEC> {
        CRCD_W::new(self, 14)
    }
    #[doc = "Bits 16:18 - Row cycle time counter"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<SDRMCON_SPEC> {
        CRC_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - Mask for row tag"]
    #[inline(always)]
    #[must_use]
    pub fn rowm(&mut self) -> ROWM_W<SDRMCON_SPEC> {
        ROWM_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - Mask for bank tag"]
    #[inline(always)]
    #[must_use]
    pub fn bankm(&mut self) -> BANKM_W<SDRMCON_SPEC> {
        BANKM_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - Row cycle time counter extension"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<SDRMCON_SPEC> {
        CRCE_W::new(self, 25)
    }
    #[doc = "Bit 28 - Disable SDRAM clock output"]
    #[inline(always)]
    #[must_use]
    pub fn clkdis(&mut self) -> CLKDIS_W<SDRMCON_SPEC> {
        CLKDIS_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - Power Save Mode used for gated clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PWR_MODE_W<SDRMCON_SPEC> {
        PWR_MODE_W::new(self, 29)
    }
    #[doc = "Bit 31 - SDRAM clock mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sdcmsel(&mut self) -> SDCMSEL_W<SDRMCON_SPEC> {
        SDCMSEL_W::new(self, 31)
    }
}
#[doc = "EBU SDRAM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrmcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrmcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDRMCON_SPEC;
impl crate::RegisterSpec for SDRMCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdrmcon::R`](R) reader structure"]
impl crate::Readable for SDRMCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdrmcon::W`](W) writer structure"]
impl crate::Writable for SDRMCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDRMCON to value 0x8000_0000"]
impl crate::Resettable for SDRMCON_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
