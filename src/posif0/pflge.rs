#[doc = "Register `PFLGE` reader"]
pub type R = crate::R<PFLGE_SPEC>;
#[doc = "Register `PFLGE` writer"]
pub type W = crate::W<PFLGE_SPEC>;
#[doc = "Correct Hall Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECHE_A {
    #[doc = "0: Correct Hall Event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Correct Hall Event interrupt enabled"]
    VALUE2 = 1,
}
impl From<ECHE_A> for bool {
    #[inline(always)]
    fn from(variant: ECHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECHE` reader - Correct Hall Event Enable"]
pub type ECHE_R = crate::BitReader<ECHE_A>;
impl ECHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECHE_A {
        match self.bits {
            false => ECHE_A::VALUE1,
            true => ECHE_A::VALUE2,
        }
    }
    #[doc = "Correct Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECHE_A::VALUE1
    }
    #[doc = "Correct Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECHE_A::VALUE2
    }
}
#[doc = "Field `ECHE` writer - Correct Hall Event Enable"]
pub type ECHE_W<'a, REG> = crate::BitWriter<'a, REG, ECHE_A>;
impl<'a, REG> ECHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ECHE_A::VALUE1)
    }
    #[doc = "Correct Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ECHE_A::VALUE2)
    }
}
#[doc = "Wrong Hall Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWHE_A {
    #[doc = "0: Wrong Hall Event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Wrong Hall Event interrupt enabled"]
    VALUE2 = 1,
}
impl From<EWHE_A> for bool {
    #[inline(always)]
    fn from(variant: EWHE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWHE` reader - Wrong Hall Event Enable"]
pub type EWHE_R = crate::BitReader<EWHE_A>;
impl EWHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWHE_A {
        match self.bits {
            false => EWHE_A::VALUE1,
            true => EWHE_A::VALUE2,
        }
    }
    #[doc = "Wrong Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EWHE_A::VALUE1
    }
    #[doc = "Wrong Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWHE_A::VALUE2
    }
}
#[doc = "Field `EWHE` writer - Wrong Hall Event Enable"]
pub type EWHE_W<'a, REG> = crate::BitWriter<'a, REG, EWHE_A>;
impl<'a, REG> EWHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wrong Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EWHE_A::VALUE1)
    }
    #[doc = "Wrong Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EWHE_A::VALUE2)
    }
}
#[doc = "Hall Input Update Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EHIE_A {
    #[doc = "0: Update of the Hall Inputs interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Update of the Hall Inputs interrupt is enabled"]
    VALUE2 = 1,
}
impl From<EHIE_A> for bool {
    #[inline(always)]
    fn from(variant: EHIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EHIE` reader - Hall Input Update Enable"]
pub type EHIE_R = crate::BitReader<EHIE_A>;
impl EHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EHIE_A {
        match self.bits {
            false => EHIE_A::VALUE1,
            true => EHIE_A::VALUE2,
        }
    }
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHIE_A::VALUE1
    }
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHIE_A::VALUE2
    }
}
#[doc = "Field `EHIE` writer - Hall Input Update Enable"]
pub type EHIE_W<'a, REG> = crate::BitWriter<'a, REG, EHIE_A>;
impl<'a, REG> EHIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EHIE_A::VALUE1)
    }
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EHIE_A::VALUE2)
    }
}
#[doc = "Multi-Channel pattern shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMST_A {
    #[doc = "0: Shadow transfer event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer event interrupt enabled"]
    VALUE2 = 1,
}
impl From<EMST_A> for bool {
    #[inline(always)]
    fn from(variant: EMST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMST` reader - Multi-Channel pattern shadow transfer enable"]
pub type EMST_R = crate::BitReader<EMST_A>;
impl EMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMST_A {
        match self.bits {
            false => EMST_A::VALUE1,
            true => EMST_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMST_A::VALUE1
    }
    #[doc = "Shadow transfer event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMST_A::VALUE2
    }
}
#[doc = "Field `EMST` writer - Multi-Channel pattern shadow transfer enable"]
pub type EMST_W<'a, REG> = crate::BitWriter<'a, REG, EMST_A>;
impl<'a, REG> EMST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shadow transfer event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EMST_A::VALUE1)
    }
    #[doc = "Shadow transfer event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EMST_A::VALUE2)
    }
}
#[doc = "Quadrature Index Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINDX_A {
    #[doc = "0: Index event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Index event interrupt enabled"]
    VALUE2 = 1,
}
impl From<EINDX_A> for bool {
    #[inline(always)]
    fn from(variant: EINDX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINDX` reader - Quadrature Index Event Enable"]
pub type EINDX_R = crate::BitReader<EINDX_A>;
impl EINDX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EINDX_A {
        match self.bits {
            false => EINDX_A::VALUE1,
            true => EINDX_A::VALUE2,
        }
    }
    #[doc = "Index event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EINDX_A::VALUE1
    }
    #[doc = "Index event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EINDX_A::VALUE2
    }
}
#[doc = "Field `EINDX` writer - Quadrature Index Event Enable"]
pub type EINDX_W<'a, REG> = crate::BitWriter<'a, REG, EINDX_A>;
impl<'a, REG> EINDX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Index event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EINDX_A::VALUE1)
    }
    #[doc = "Index event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EINDX_A::VALUE2)
    }
}
#[doc = "Quadrature Phase Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EERR_A {
    #[doc = "0: Phase error event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Phase error event interrupt enabled"]
    VALUE2 = 1,
}
impl From<EERR_A> for bool {
    #[inline(always)]
    fn from(variant: EERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EERR` reader - Quadrature Phase Error Enable"]
pub type EERR_R = crate::BitReader<EERR_A>;
impl EERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EERR_A {
        match self.bits {
            false => EERR_A::VALUE1,
            true => EERR_A::VALUE2,
        }
    }
    #[doc = "Phase error event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EERR_A::VALUE1
    }
    #[doc = "Phase error event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EERR_A::VALUE2
    }
}
#[doc = "Field `EERR` writer - Quadrature Phase Error Enable"]
pub type EERR_W<'a, REG> = crate::BitWriter<'a, REG, EERR_A>;
impl<'a, REG> EERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase error event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EERR_A::VALUE1)
    }
    #[doc = "Phase error event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EERR_A::VALUE2)
    }
}
#[doc = "Quadrature CLK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECNT_A {
    #[doc = "0: Quadrature CLK event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Quadrature CLK event interrupt enabled"]
    VALUE2 = 1,
}
impl From<ECNT_A> for bool {
    #[inline(always)]
    fn from(variant: ECNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECNT` reader - Quadrature CLK interrupt Enable"]
pub type ECNT_R = crate::BitReader<ECNT_A>;
impl ECNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECNT_A {
        match self.bits {
            false => ECNT_A::VALUE1,
            true => ECNT_A::VALUE2,
        }
    }
    #[doc = "Quadrature CLK event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECNT_A::VALUE1
    }
    #[doc = "Quadrature CLK event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECNT_A::VALUE2
    }
}
#[doc = "Field `ECNT` writer - Quadrature CLK interrupt Enable"]
pub type ECNT_W<'a, REG> = crate::BitWriter<'a, REG, ECNT_A>;
impl<'a, REG> ECNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature CLK event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ECNT_A::VALUE1)
    }
    #[doc = "Quadrature CLK event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ECNT_A::VALUE2)
    }
}
#[doc = "Quadrature direction change interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDIR_A {
    #[doc = "0: Direction change event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Direction change event interrupt enabled"]
    VALUE2 = 1,
}
impl From<EDIR_A> for bool {
    #[inline(always)]
    fn from(variant: EDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDIR` reader - Quadrature direction change interrupt Enable"]
pub type EDIR_R = crate::BitReader<EDIR_A>;
impl EDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EDIR_A {
        match self.bits {
            false => EDIR_A::VALUE1,
            true => EDIR_A::VALUE2,
        }
    }
    #[doc = "Direction change event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EDIR_A::VALUE1
    }
    #[doc = "Direction change event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EDIR_A::VALUE2
    }
}
#[doc = "Field `EDIR` writer - Quadrature direction change interrupt Enable"]
pub type EDIR_W<'a, REG> = crate::BitWriter<'a, REG, EDIR_A>;
impl<'a, REG> EDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direction change event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EDIR_A::VALUE1)
    }
    #[doc = "Direction change event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EDIR_A::VALUE2)
    }
}
#[doc = "Quadrature Period CLK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPCLK_A {
    #[doc = "0: Quadrature Period CLK event interrupt disabled"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Period CLK event interrupt enabled"]
    VALUE2 = 1,
}
impl From<EPCLK_A> for bool {
    #[inline(always)]
    fn from(variant: EPCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPCLK` reader - Quadrature Period CLK interrupt Enable"]
pub type EPCLK_R = crate::BitReader<EPCLK_A>;
impl EPCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPCLK_A {
        match self.bits {
            false => EPCLK_A::VALUE1,
            true => EPCLK_A::VALUE2,
        }
    }
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPCLK_A::VALUE1
    }
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPCLK_A::VALUE2
    }
}
#[doc = "Field `EPCLK` writer - Quadrature Period CLK interrupt Enable"]
pub type EPCLK_W<'a, REG> = crate::BitWriter<'a, REG, EPCLK_A>;
impl<'a, REG> EPCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPCLK_A::VALUE1)
    }
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPCLK_A::VALUE2)
    }
}
#[doc = "Correct Hall Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHESEL_A {
    #[doc = "0: Correct Hall Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Correct Hall Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<CHESEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHESEL` reader - Correct Hall Event Service Request Selector"]
pub type CHESEL_R = crate::BitReader<CHESEL_A>;
impl CHESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHESEL_A {
        match self.bits {
            false => CHESEL_A::VALUE1,
            true => CHESEL_A::VALUE2,
        }
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHESEL_A::VALUE1
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHESEL_A::VALUE2
    }
}
#[doc = "Field `CHESEL` writer - Correct Hall Event Service Request Selector"]
pub type CHESEL_W<'a, REG> = crate::BitWriter<'a, REG, CHESEL_A>;
impl<'a, REG> CHESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CHESEL_A::VALUE1)
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CHESEL_A::VALUE2)
    }
}
#[doc = "Wrong Hall Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WHESEL_A {
    #[doc = "0: Wrong Hall Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Wrong Hall Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<WHESEL_A> for bool {
    #[inline(always)]
    fn from(variant: WHESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WHESEL` reader - Wrong Hall Event Service Request Selector"]
pub type WHESEL_R = crate::BitReader<WHESEL_A>;
impl WHESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WHESEL_A {
        match self.bits {
            false => WHESEL_A::VALUE1,
            true => WHESEL_A::VALUE2,
        }
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WHESEL_A::VALUE1
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WHESEL_A::VALUE2
    }
}
#[doc = "Field `WHESEL` writer - Wrong Hall Event Service Request Selector"]
pub type WHESEL_W<'a, REG> = crate::BitWriter<'a, REG, WHESEL_A>;
impl<'a, REG> WHESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WHESEL_A::VALUE1)
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WHESEL_A::VALUE2)
    }
}
#[doc = "Hall Inputs Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIESEL_A {
    #[doc = "0: Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<HIESEL_A> for bool {
    #[inline(always)]
    fn from(variant: HIESEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIESEL` reader - Hall Inputs Update Event Service Request Selector"]
pub type HIESEL_R = crate::BitReader<HIESEL_A>;
impl HIESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIESEL_A {
        match self.bits {
            false => HIESEL_A::VALUE1,
            true => HIESEL_A::VALUE2,
        }
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIESEL_A::VALUE1
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIESEL_A::VALUE2
    }
}
#[doc = "Field `HIESEL` writer - Hall Inputs Update Event Service Request Selector"]
pub type HIESEL_W<'a, REG> = crate::BitWriter<'a, REG, HIESEL_A>;
impl<'a, REG> HIESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIESEL_A::VALUE1)
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIESEL_A::VALUE2)
    }
}
#[doc = "Multi-Channel pattern Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTSEL_A {
    #[doc = "0: Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<MSTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTSEL` reader - Multi-Channel pattern Update Event Service Request Selector"]
pub type MSTSEL_R = crate::BitReader<MSTSEL_A>;
impl MSTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSTSEL_A {
        match self.bits {
            false => MSTSEL_A::VALUE1,
            true => MSTSEL_A::VALUE2,
        }
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSTSEL_A::VALUE1
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSTSEL_A::VALUE2
    }
}
#[doc = "Field `MSTSEL` writer - Multi-Channel pattern Update Event Service Request Selector"]
pub type MSTSEL_W<'a, REG> = crate::BitWriter<'a, REG, MSTSEL_A>;
impl<'a, REG> MSTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTSEL_A::VALUE1)
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MSTSEL_A::VALUE2)
    }
}
#[doc = "Quadrature Index Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INDSEL_A {
    #[doc = "0: Quadrature Index Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Index Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<INDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: INDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INDSEL` reader - Quadrature Index Event Service Request Selector"]
pub type INDSEL_R = crate::BitReader<INDSEL_A>;
impl INDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INDSEL_A {
        match self.bits {
            false => INDSEL_A::VALUE1,
            true => INDSEL_A::VALUE2,
        }
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INDSEL_A::VALUE1
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INDSEL_A::VALUE2
    }
}
#[doc = "Field `INDSEL` writer - Quadrature Index Event Service Request Selector"]
pub type INDSEL_W<'a, REG> = crate::BitWriter<'a, REG, INDSEL_A>;
impl<'a, REG> INDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(INDSEL_A::VALUE1)
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(INDSEL_A::VALUE2)
    }
}
#[doc = "Quadrature Phase Error Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSEL_A {
    #[doc = "0: Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<ERRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ERRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRSEL` reader - Quadrature Phase Error Event Service Request Selector"]
pub type ERRSEL_R = crate::BitReader<ERRSEL_A>;
impl ERRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRSEL_A {
        match self.bits {
            false => ERRSEL_A::VALUE1,
            true => ERRSEL_A::VALUE2,
        }
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERRSEL_A::VALUE1
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRSEL_A::VALUE2
    }
}
#[doc = "Field `ERRSEL` writer - Quadrature Phase Error Event Service Request Selector"]
pub type ERRSEL_W<'a, REG> = crate::BitWriter<'a, REG, ERRSEL_A>;
impl<'a, REG> ERRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRSEL_A::VALUE1)
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ERRSEL_A::VALUE2)
    }
}
#[doc = "Quadrature Clock Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTSEL_A {
    #[doc = "0: Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<CNTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CNTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTSEL` reader - Quadrature Clock Event Service Request Selector"]
pub type CNTSEL_R = crate::BitReader<CNTSEL_A>;
impl CNTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTSEL_A {
        match self.bits {
            false => CNTSEL_A::VALUE1,
            true => CNTSEL_A::VALUE2,
        }
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CNTSEL_A::VALUE1
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CNTSEL_A::VALUE2
    }
}
#[doc = "Field `CNTSEL` writer - Quadrature Clock Event Service Request Selector"]
pub type CNTSEL_W<'a, REG> = crate::BitWriter<'a, REG, CNTSEL_A>;
impl<'a, REG> CNTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTSEL_A::VALUE1)
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CNTSEL_A::VALUE2)
    }
}
#[doc = "Quadrature Direction Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRSEL_A {
    #[doc = "0: Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<DIRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DIRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRSEL` reader - Quadrature Direction Update Event Service Request Selector"]
pub type DIRSEL_R = crate::BitReader<DIRSEL_A>;
impl DIRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRSEL_A {
        match self.bits {
            false => DIRSEL_A::VALUE1,
            true => DIRSEL_A::VALUE2,
        }
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIRSEL_A::VALUE1
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIRSEL_A::VALUE2
    }
}
#[doc = "Field `DIRSEL` writer - Quadrature Direction Update Event Service Request Selector"]
pub type DIRSEL_W<'a, REG> = crate::BitWriter<'a, REG, DIRSEL_A>;
impl<'a, REG> DIRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRSEL_A::VALUE1)
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DIRSEL_A::VALUE2)
    }
}
#[doc = "Quadrature Period clock Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCLSEL_A {
    #[doc = "0: Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    VALUE1 = 0,
    #[doc = "1: Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    VALUE2 = 1,
}
impl From<PCLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PCLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLSEL` reader - Quadrature Period clock Event Service Request Selector"]
pub type PCLSEL_R = crate::BitReader<PCLSEL_A>;
impl PCLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLSEL_A {
        match self.bits {
            false => PCLSEL_A::VALUE1,
            true => PCLSEL_A::VALUE2,
        }
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCLSEL_A::VALUE1
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCLSEL_A::VALUE2
    }
}
#[doc = "Field `PCLSEL` writer - Quadrature Period clock Event Service Request Selector"]
pub type PCLSEL_W<'a, REG> = crate::BitWriter<'a, REG, PCLSEL_A>;
impl<'a, REG> PCLSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PCLSEL_A::VALUE1)
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline(always)]
    pub fn eche(&self) -> ECHE_R {
        ECHE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline(always)]
    pub fn ewhe(&self) -> EWHE_R {
        EWHE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline(always)]
    pub fn ehie(&self) -> EHIE_R {
        EHIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline(always)]
    pub fn emst(&self) -> EMST_R {
        EMST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline(always)]
    pub fn eindx(&self) -> EINDX_R {
        EINDX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline(always)]
    pub fn ecnt(&self) -> ECNT_R {
        ECNT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline(always)]
    pub fn edir(&self) -> EDIR_R {
        EDIR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline(always)]
    pub fn epclk(&self) -> EPCLK_R {
        EPCLK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn chesel(&self) -> CHESEL_R {
        CHESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn whesel(&self) -> WHESEL_R {
        WHESEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline(always)]
    pub fn hiesel(&self) -> HIESEL_R {
        HIESEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline(always)]
    pub fn indsel(&self) -> INDSEL_R {
        INDSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline(always)]
    pub fn errsel(&self) -> ERRSEL_R {
        ERRSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline(always)]
    pub fn cntsel(&self) -> CNTSEL_R {
        CNTSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline(always)]
    pub fn dirsel(&self) -> DIRSEL_R {
        DIRSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline(always)]
    pub fn pclsel(&self) -> PCLSEL_R {
        PCLSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eche(&mut self) -> ECHE_W<PFLGE_SPEC> {
        ECHE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewhe(&mut self) -> EWHE_W<PFLGE_SPEC> {
        EWHE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ehie(&mut self) -> EHIE_W<PFLGE_SPEC> {
        EHIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn emst(&mut self) -> EMST_W<PFLGE_SPEC> {
        EMST_W::new(self, 4)
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eindx(&mut self) -> EINDX_W<PFLGE_SPEC> {
        EINDX_W::new(self, 8)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eerr(&mut self) -> EERR_W<PFLGE_SPEC> {
        EERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecnt(&mut self) -> ECNT_W<PFLGE_SPEC> {
        ECNT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn edir(&mut self) -> EDIR_W<PFLGE_SPEC> {
        EDIR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epclk(&mut self) -> EPCLK_W<PFLGE_SPEC> {
        EPCLK_W::new(self, 12)
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn chesel(&mut self) -> CHESEL_W<PFLGE_SPEC> {
        CHESEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn whesel(&mut self) -> WHESEL_W<PFLGE_SPEC> {
        WHESEL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn hiesel(&mut self) -> HIESEL_W<PFLGE_SPEC> {
        HIESEL_W::new(self, 18)
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mstsel(&mut self) -> MSTSEL_W<PFLGE_SPEC> {
        MSTSEL_W::new(self, 20)
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn indsel(&mut self) -> INDSEL_W<PFLGE_SPEC> {
        INDSEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn errsel(&mut self) -> ERRSEL_W<PFLGE_SPEC> {
        ERRSEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn cntsel(&mut self) -> CNTSEL_W<PFLGE_SPEC> {
        CNTSEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn dirsel(&mut self) -> DIRSEL_W<PFLGE_SPEC> {
        DIRSEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline(always)]
    #[must_use]
    pub fn pclsel(&mut self) -> PCLSEL_W<PFLGE_SPEC> {
        PCLSEL_W::new(self, 28)
    }
}
#[doc = "POSIF Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pflge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pflge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFLGE_SPEC;
impl crate::RegisterSpec for PFLGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pflge::R`](R) reader structure"]
impl crate::Readable for PFLGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pflge::W`](W) writer structure"]
impl crate::Writable for PFLGE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFLGE to value 0"]
impl crate::Resettable for PFLGE_SPEC {
    const RESET_VALUE: u32 = 0;
}
