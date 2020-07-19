#[doc = "Reader of register PFLGE"]
pub type R = crate::R<u32, super::PFLGE>;
#[doc = "Writer for register PFLGE"]
pub type W = crate::W<u32, super::PFLGE>;
#[doc = "Register PFLGE `reset()`'s with value 0"]
impl crate::ResetValue for super::PFLGE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Correct Hall Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ECHE`"]
pub type ECHE_R = crate::R<bool, ECHE_A>;
impl ECHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECHE_A {
        match self.bits {
            false => ECHE_A::VALUE1,
            true => ECHE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECHE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECHE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ECHE`"]
pub struct ECHE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECHE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECHE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Correct Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECHE_A::VALUE1)
    }
    #[doc = "Correct Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECHE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Wrong Hall Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EWHE`"]
pub type EWHE_R = crate::R<bool, EWHE_A>;
impl EWHE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWHE_A {
        match self.bits {
            false => EWHE_A::VALUE1,
            true => EWHE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EWHE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EWHE_A::VALUE2
    }
}
#[doc = "Write proxy for field `EWHE`"]
pub struct EWHE_W<'a> {
    w: &'a mut W,
}
impl<'a> EWHE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EWHE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wrong Hall Event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EWHE_A::VALUE1)
    }
    #[doc = "Wrong Hall Event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EWHE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Hall Input Update Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EHIE`"]
pub type EHIE_R = crate::R<bool, EHIE_A>;
impl EHIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EHIE_A {
        match self.bits {
            false => EHIE_A::VALUE1,
            true => EHIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EHIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EHIE_A::VALUE2
    }
}
#[doc = "Write proxy for field `EHIE`"]
pub struct EHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EHIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EHIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Update of the Hall Inputs interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EHIE_A::VALUE1)
    }
    #[doc = "Update of the Hall Inputs interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EHIE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Multi-Channel pattern shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EMST`"]
pub type EMST_R = crate::R<bool, EMST_A>;
impl EMST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMST_A {
        match self.bits {
            false => EMST_A::VALUE1,
            true => EMST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMST_A::VALUE2
    }
}
#[doc = "Write proxy for field `EMST`"]
pub struct EMST_W<'a> {
    w: &'a mut W,
}
impl<'a> EMST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shadow transfer event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EMST_A::VALUE1)
    }
    #[doc = "Shadow transfer event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EMST_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Quadrature Index Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EINDX`"]
pub type EINDX_R = crate::R<bool, EINDX_A>;
impl EINDX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINDX_A {
        match self.bits {
            false => EINDX_A::VALUE1,
            true => EINDX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EINDX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EINDX_A::VALUE2
    }
}
#[doc = "Write proxy for field `EINDX`"]
pub struct EINDX_W<'a> {
    w: &'a mut W,
}
impl<'a> EINDX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINDX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Index event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EINDX_A::VALUE1)
    }
    #[doc = "Index event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EINDX_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Quadrature Phase Error Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EERR`"]
pub type EERR_R = crate::R<bool, EERR_A>;
impl EERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EERR_A {
        match self.bits {
            false => EERR_A::VALUE1,
            true => EERR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EERR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EERR_A::VALUE2
    }
}
#[doc = "Write proxy for field `EERR`"]
pub struct EERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase error event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EERR_A::VALUE1)
    }
    #[doc = "Phase error event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EERR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Quadrature CLK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ECNT`"]
pub type ECNT_R = crate::R<bool, ECNT_A>;
impl ECNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECNT_A {
        match self.bits {
            false => ECNT_A::VALUE1,
            true => ECNT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECNT_A::VALUE2
    }
}
#[doc = "Write proxy for field `ECNT`"]
pub struct ECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ECNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature CLK event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECNT_A::VALUE1)
    }
    #[doc = "Quadrature CLK event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECNT_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Quadrature direction change interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EDIR`"]
pub type EDIR_R = crate::R<bool, EDIR_A>;
impl EDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDIR_A {
        match self.bits {
            false => EDIR_A::VALUE1,
            true => EDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EDIR_A::VALUE2
    }
}
#[doc = "Write proxy for field `EDIR`"]
pub struct EDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EDIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Direction change event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EDIR_A::VALUE1)
    }
    #[doc = "Direction change event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EDIR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Quadrature Period CLK interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `EPCLK`"]
pub type EPCLK_R = crate::R<bool, EPCLK_A>;
impl EPCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPCLK_A {
        match self.bits {
            false => EPCLK_A::VALUE1,
            true => EPCLK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPCLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPCLK_A::VALUE2
    }
}
#[doc = "Write proxy for field `EPCLK`"]
pub struct EPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Period CLK event interrupt disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPCLK_A::VALUE1)
    }
    #[doc = "Quadrature Period CLK event interrupt enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPCLK_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Correct Hall Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CHESEL`"]
pub type CHESEL_R = crate::R<bool, CHESEL_A>;
impl CHESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHESEL_A {
        match self.bits {
            false => CHESEL_A::VALUE1,
            true => CHESEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CHESEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CHESEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `CHESEL`"]
pub struct CHESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CHESEL_A::VALUE1)
    }
    #[doc = "Correct Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CHESEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Wrong Hall Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `WHESEL`"]
pub type WHESEL_R = crate::R<bool, WHESEL_A>;
impl WHESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHESEL_A {
        match self.bits {
            false => WHESEL_A::VALUE1,
            true => WHESEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WHESEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WHESEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `WHESEL`"]
pub struct WHESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WHESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WHESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WHESEL_A::VALUE1)
    }
    #[doc = "Wrong Hall Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WHESEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Hall Inputs Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HIESEL`"]
pub type HIESEL_R = crate::R<bool, HIESEL_A>;
impl HIESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIESEL_A {
        match self.bits {
            false => HIESEL_A::VALUE1,
            true => HIESEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIESEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIESEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `HIESEL`"]
pub struct HIESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HIESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIESEL_A::VALUE1)
    }
    #[doc = "Hall Inputs Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIESEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Multi-Channel pattern Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `MSTSEL`"]
pub type MSTSEL_R = crate::R<bool, MSTSEL_A>;
impl MSTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSEL_A {
        match self.bits {
            false => MSTSEL_A::VALUE1,
            true => MSTSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MSTSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MSTSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `MSTSEL`"]
pub struct MSTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MSTSEL_A::VALUE1)
    }
    #[doc = "Multi-Channel pattern Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MSTSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Quadrature Index Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `INDSEL`"]
pub type INDSEL_R = crate::R<bool, INDSEL_A>;
impl INDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INDSEL_A {
        match self.bits {
            false => INDSEL_A::VALUE1,
            true => INDSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INDSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `INDSEL`"]
pub struct INDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INDSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(INDSEL_A::VALUE1)
    }
    #[doc = "Quadrature Index Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(INDSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Quadrature Phase Error Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ERRSEL`"]
pub type ERRSEL_R = crate::R<bool, ERRSEL_A>;
impl ERRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRSEL_A {
        match self.bits {
            false => ERRSEL_A::VALUE1,
            true => ERRSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERRSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERRSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `ERRSEL`"]
pub struct ERRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ERRSEL_A::VALUE1)
    }
    #[doc = "Quadrature Phase error Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ERRSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Quadrature Clock Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CNTSEL`"]
pub type CNTSEL_R = crate::R<bool, CNTSEL_A>;
impl CNTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTSEL_A {
        match self.bits {
            false => CNTSEL_A::VALUE1,
            true => CNTSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CNTSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CNTSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `CNTSEL`"]
pub struct CNTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CNTSEL_A::VALUE1)
    }
    #[doc = "Quadrature Clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CNTSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Quadrature Direction Update Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `DIRSEL`"]
pub type DIRSEL_R = crate::R<bool, DIRSEL_A>;
impl DIRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRSEL_A {
        match self.bits {
            false => DIRSEL_A::VALUE1,
            true => DIRSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DIRSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DIRSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `DIRSEL`"]
pub struct DIRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIRSEL_A::VALUE1)
    }
    #[doc = "Quadrature Direction Update Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIRSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Quadrature Period clock Event Service Request Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `PCLSEL`"]
pub type PCLSEL_R = crate::R<bool, PCLSEL_A>;
impl PCLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLSEL_A {
        match self.bits {
            false => PCLSEL_A::VALUE1,
            true => PCLSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCLSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCLSEL_A::VALUE2
    }
}
#[doc = "Write proxy for field `PCLSEL`"]
pub struct PCLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PCLSEL_A::VALUE1)
    }
    #[doc = "Quadrature Period clock Event interrupt forward to POSIFx.SR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PCLSEL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline(always)]
    pub fn eche(&self) -> ECHE_R {
        ECHE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline(always)]
    pub fn ewhe(&self) -> EWHE_R {
        EWHE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline(always)]
    pub fn ehie(&self) -> EHIE_R {
        EHIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline(always)]
    pub fn emst(&self) -> EMST_R {
        EMST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline(always)]
    pub fn eindx(&self) -> EINDX_R {
        EINDX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline(always)]
    pub fn ecnt(&self) -> ECNT_R {
        ECNT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline(always)]
    pub fn edir(&self) -> EDIR_R {
        EDIR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline(always)]
    pub fn epclk(&self) -> EPCLK_R {
        EPCLK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn chesel(&self) -> CHESEL_R {
        CHESEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn whesel(&self) -> WHESEL_R {
        WHESEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline(always)]
    pub fn hiesel(&self) -> HIESEL_R {
        HIESEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline(always)]
    pub fn indsel(&self) -> INDSEL_R {
        INDSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline(always)]
    pub fn errsel(&self) -> ERRSEL_R {
        ERRSEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline(always)]
    pub fn cntsel(&self) -> CNTSEL_R {
        CNTSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline(always)]
    pub fn dirsel(&self) -> DIRSEL_R {
        DIRSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline(always)]
    pub fn pclsel(&self) -> PCLSEL_R {
        PCLSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Correct Hall Event Enable"]
    #[inline(always)]
    pub fn eche(&mut self) -> ECHE_W {
        ECHE_W { w: self }
    }
    #[doc = "Bit 1 - Wrong Hall Event Enable"]
    #[inline(always)]
    pub fn ewhe(&mut self) -> EWHE_W {
        EWHE_W { w: self }
    }
    #[doc = "Bit 2 - Hall Input Update Enable"]
    #[inline(always)]
    pub fn ehie(&mut self) -> EHIE_W {
        EHIE_W { w: self }
    }
    #[doc = "Bit 4 - Multi-Channel pattern shadow transfer enable"]
    #[inline(always)]
    pub fn emst(&mut self) -> EMST_W {
        EMST_W { w: self }
    }
    #[doc = "Bit 8 - Quadrature Index Event Enable"]
    #[inline(always)]
    pub fn eindx(&mut self) -> EINDX_W {
        EINDX_W { w: self }
    }
    #[doc = "Bit 9 - Quadrature Phase Error Enable"]
    #[inline(always)]
    pub fn eerr(&mut self) -> EERR_W {
        EERR_W { w: self }
    }
    #[doc = "Bit 10 - Quadrature CLK interrupt Enable"]
    #[inline(always)]
    pub fn ecnt(&mut self) -> ECNT_W {
        ECNT_W { w: self }
    }
    #[doc = "Bit 11 - Quadrature direction change interrupt Enable"]
    #[inline(always)]
    pub fn edir(&mut self) -> EDIR_W {
        EDIR_W { w: self }
    }
    #[doc = "Bit 12 - Quadrature Period CLK interrupt Enable"]
    #[inline(always)]
    pub fn epclk(&mut self) -> EPCLK_W {
        EPCLK_W { w: self }
    }
    #[doc = "Bit 16 - Correct Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn chesel(&mut self) -> CHESEL_W {
        CHESEL_W { w: self }
    }
    #[doc = "Bit 17 - Wrong Hall Event Service Request Selector"]
    #[inline(always)]
    pub fn whesel(&mut self) -> WHESEL_W {
        WHESEL_W { w: self }
    }
    #[doc = "Bit 18 - Hall Inputs Update Event Service Request Selector"]
    #[inline(always)]
    pub fn hiesel(&mut self) -> HIESEL_W {
        HIESEL_W { w: self }
    }
    #[doc = "Bit 20 - Multi-Channel pattern Update Event Service Request Selector"]
    #[inline(always)]
    pub fn mstsel(&mut self) -> MSTSEL_W {
        MSTSEL_W { w: self }
    }
    #[doc = "Bit 24 - Quadrature Index Event Service Request Selector"]
    #[inline(always)]
    pub fn indsel(&mut self) -> INDSEL_W {
        INDSEL_W { w: self }
    }
    #[doc = "Bit 25 - Quadrature Phase Error Event Service Request Selector"]
    #[inline(always)]
    pub fn errsel(&mut self) -> ERRSEL_W {
        ERRSEL_W { w: self }
    }
    #[doc = "Bit 26 - Quadrature Clock Event Service Request Selector"]
    #[inline(always)]
    pub fn cntsel(&mut self) -> CNTSEL_W {
        CNTSEL_W { w: self }
    }
    #[doc = "Bit 27 - Quadrature Direction Update Event Service Request Selector"]
    #[inline(always)]
    pub fn dirsel(&mut self) -> DIRSEL_W {
        DIRSEL_W { w: self }
    }
    #[doc = "Bit 28 - Quadrature Period clock Event Service Request Selector"]
    #[inline(always)]
    pub fn pclsel(&mut self) -> PCLSEL_W {
        PCLSEL_W { w: self }
    }
}
