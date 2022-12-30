#[doc = "Register `RBCTR` reader"]
pub struct R(crate::R<RBCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBCTR` writer"]
pub struct W(crate::W<RBCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RBCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPTR` writer - Data Pointer"]
pub type DPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBCTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `LIMIT` reader - Limit For Interrupt Generation"]
pub type LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIMIT` writer - Limit For Interrupt Generation"]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBCTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `SRBTM` reader - Standard Receive Buffer Trigger Mode"]
pub type SRBTM_R = crate::BitReader<SRBTM_A>;
#[doc = "Standard Receive Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRBTM_A {
    #[doc = "0: Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    VALUE1 = 0,
    #[doc = "1: Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    VALUE2 = 1,
}
impl From<SRBTM_A> for bool {
    #[inline(always)]
    fn from(variant: SRBTM_A) -> Self {
        variant as u8 != 0
    }
}
impl SRBTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBTM_A {
        match self.bits {
            false => SRBTM_A::VALUE1,
            true => SRBTM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBTM_A::VALUE2
    }
}
#[doc = "Field `SRBTM` writer - Standard Receive Buffer Trigger Mode"]
pub type SRBTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, SRBTM_A, O>;
impl<'a, const O: u8> SRBTM_W<'a, O> {
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTM_A::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTM_A::VALUE2)
    }
}
#[doc = "Field `SRBTEN` reader - Standard Receive Buffer Trigger Enable"]
pub type SRBTEN_R = crate::BitReader<SRBTEN_A>;
#[doc = "Standard Receive Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRBTEN_A {
    #[doc = "0: The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    VALUE2 = 1,
}
impl From<SRBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRBTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRBTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBTEN_A {
        match self.bits {
            false => SRBTEN_A::VALUE1,
            true => SRBTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBTEN_A::VALUE2
    }
}
#[doc = "Field `SRBTEN` writer - Standard Receive Buffer Trigger Enable"]
pub type SRBTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, SRBTEN_A, O>;
impl<'a, const O: u8> SRBTEN_W<'a, O> {
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTEN_A::VALUE1)
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTEN_A::VALUE2)
    }
}
#[doc = "Field `SRBINP` reader - Standard Receive Buffer Interrupt Node Pointer"]
pub type SRBINP_R = crate::FieldReader<u8, SRBINP_A>;
#[doc = "Standard Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRBINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6 = 5,
}
impl From<SRBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRBINP_A) -> Self {
        variant as _
    }
}
impl SRBINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRBINP_A> {
        match self.bits {
            0 => Some(SRBINP_A::VALUE1),
            1 => Some(SRBINP_A::VALUE2),
            2 => Some(SRBINP_A::VALUE3),
            3 => Some(SRBINP_A::VALUE4),
            4 => Some(SRBINP_A::VALUE5),
            5 => Some(SRBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SRBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SRBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SRBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SRBINP_A::VALUE6
    }
}
#[doc = "Field `SRBINP` writer - Standard Receive Buffer Interrupt Node Pointer"]
pub type SRBINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBCTR_SPEC, u8, SRBINP_A, 3, O>;
impl<'a, const O: u8> SRBINP_W<'a, O> {
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE6)
    }
}
#[doc = "Field `ARBINP` reader - Alternative Receive Buffer Interrupt Node Pointer"]
pub type ARBINP_R = crate::FieldReader<u8, ARBINP_A>;
#[doc = "Alternative Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6 = 5,
}
impl From<ARBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBINP_A) -> Self {
        variant as _
    }
}
impl ARBINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARBINP_A> {
        match self.bits {
            0 => Some(ARBINP_A::VALUE1),
            1 => Some(ARBINP_A::VALUE2),
            2 => Some(ARBINP_A::VALUE3),
            3 => Some(ARBINP_A::VALUE4),
            4 => Some(ARBINP_A::VALUE5),
            5 => Some(ARBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ARBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ARBINP_A::VALUE6
    }
}
#[doc = "Field `ARBINP` writer - Alternative Receive Buffer Interrupt Node Pointer"]
pub type ARBINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBCTR_SPEC, u8, ARBINP_A, 3, O>;
impl<'a, const O: u8> ARBINP_W<'a, O> {
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE6)
    }
}
#[doc = "Field `RCIM` reader - Receiver Control Information Mode"]
pub type RCIM_R = crate::FieldReader<u8, RCIM_A>;
#[doc = "Receiver Control Information Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCIM_A {
    #[doc = "0: RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    VALUE1 = 0,
    #[doc = "1: RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    VALUE2 = 1,
    #[doc = "2: RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    VALUE3 = 2,
    #[doc = "3: RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    VALUE4 = 3,
}
impl From<RCIM_A> for u8 {
    #[inline(always)]
    fn from(variant: RCIM_A) -> Self {
        variant as _
    }
}
impl RCIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIM_A {
        match self.bits {
            0 => RCIM_A::VALUE1,
            1 => RCIM_A::VALUE2,
            2 => RCIM_A::VALUE3,
            3 => RCIM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RCIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RCIM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RCIM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RCIM_A::VALUE4
    }
}
#[doc = "Field `RCIM` writer - Receiver Control Information Mode"]
pub type RCIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RBCTR_SPEC, u8, RCIM_A, 2, O>;
impl<'a, const O: u8> RCIM_W<'a, O> {
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE1)
    }
    #[doc = "RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE2)
    }
    #[doc = "RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE3)
    }
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE4)
    }
}
#[doc = "Field `SIZE` reader - Buffer Size"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    VALUE1 = 0,
    #[doc = "1: The FIFO buffer contains 2 entries."]
    VALUE2 = 1,
    #[doc = "2: The FIFO buffer contains 4 entries."]
    VALUE3 = 2,
    #[doc = "3: The FIFO buffer contains 8 entries."]
    VALUE4 = 3,
    #[doc = "4: The FIFO buffer contains 16 entries."]
    VALUE5 = 4,
    #[doc = "5: The FIFO buffer contains 32 entries."]
    VALUE6 = 5,
    #[doc = "6: The FIFO buffer contains 64 entries."]
    VALUE7 = 6,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::VALUE1),
            1 => Some(SIZE_A::VALUE2),
            2 => Some(SIZE_A::VALUE3),
            3 => Some(SIZE_A::VALUE4),
            4 => Some(SIZE_A::VALUE5),
            5 => Some(SIZE_A::VALUE6),
            6 => Some(SIZE_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIZE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIZE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SIZE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SIZE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SIZE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SIZE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SIZE_A::VALUE7
    }
}
#[doc = "Field `SIZE` writer - Buffer Size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBCTR_SPEC, u8, SIZE_A, 3, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE1)
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE2)
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE3)
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE4)
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE5)
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE6)
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE7)
    }
}
#[doc = "Field `RNM` reader - Receiver Notification Mode"]
pub type RNM_R = crate::BitReader<RNM_A>;
#[doc = "Receiver Notification Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNM_A {
    #[doc = "0: Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    VALUE1 = 0,
    #[doc = "1: RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    VALUE2 = 1,
}
impl From<RNM_A> for bool {
    #[inline(always)]
    fn from(variant: RNM_A) -> Self {
        variant as u8 != 0
    }
}
impl RNM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNM_A {
        match self.bits {
            false => RNM_A::VALUE1,
            true => RNM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RNM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RNM_A::VALUE2
    }
}
#[doc = "Field `RNM` writer - Receiver Notification Mode"]
pub type RNM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, RNM_A, O>;
impl<'a, const O: u8> RNM_W<'a, O> {
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNM_A::VALUE1)
    }
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNM_A::VALUE2)
    }
}
#[doc = "Field `LOF` reader - Buffer Event on Limit Overflow"]
pub type LOF_R = crate::BitReader<LOF_A>;
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOF_A {
    #[doc = "0: A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    VALUE1 = 0,
    #[doc = "1: A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    VALUE2 = 1,
}
impl From<LOF_A> for bool {
    #[inline(always)]
    fn from(variant: LOF_A) -> Self {
        variant as u8 != 0
    }
}
impl LOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOF_A {
        match self.bits {
            false => LOF_A::VALUE1,
            true => LOF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOF_A::VALUE2
    }
}
#[doc = "Field `LOF` writer - Buffer Event on Limit Overflow"]
pub type LOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, LOF_A, O>;
impl<'a, const O: u8> LOF_W<'a, O> {
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOF_A::VALUE1)
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOF_A::VALUE2)
    }
}
#[doc = "Field `ARBIEN` reader - Alternative Receive Buffer Interrupt Enable"]
pub type ARBIEN_R = crate::BitReader<ARBIEN_A>;
#[doc = "Alternative Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBIEN_A {
    #[doc = "0: The alternative receive buffer interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The alternative receive buffer interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<ARBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARBIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBIEN_A {
        match self.bits {
            false => ARBIEN_A::VALUE1,
            true => ARBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBIEN_A::VALUE2
    }
}
#[doc = "Field `ARBIEN` writer - Alternative Receive Buffer Interrupt Enable"]
pub type ARBIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, ARBIEN_A, O>;
impl<'a, const O: u8> ARBIEN_W<'a, O> {
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBIEN_A::VALUE1)
    }
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBIEN_A::VALUE2)
    }
}
#[doc = "Field `SRBIEN` reader - Standard Receive Buffer Interrupt Enable"]
pub type SRBIEN_R = crate::BitReader<SRBIEN_A>;
#[doc = "Standard Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRBIEN_A {
    #[doc = "0: The standard receive buffer interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard receive buffer interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<SRBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRBIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SRBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBIEN_A {
        match self.bits {
            false => SRBIEN_A::VALUE1,
            true => SRBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBIEN_A::VALUE2
    }
}
#[doc = "Field `SRBIEN` writer - Standard Receive Buffer Interrupt Enable"]
pub type SRBIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, SRBIEN_A, O>;
impl<'a, const O: u8> SRBIEN_W<'a, O> {
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBIEN_A::VALUE1)
    }
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBIEN_A::VALUE2)
    }
}
#[doc = "Field `RBERIEN` reader - Receive Buffer Error Interrupt Enable"]
pub type RBERIEN_R = crate::BitReader<RBERIEN_A>;
#[doc = "Receive Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBERIEN_A {
    #[doc = "0: The receive buffer error interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The receive buffer error interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<RBERIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RBERIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RBERIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBERIEN_A {
        match self.bits {
            false => RBERIEN_A::VALUE1,
            true => RBERIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBERIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBERIEN_A::VALUE2
    }
}
#[doc = "Field `RBERIEN` writer - Receive Buffer Error Interrupt Enable"]
pub type RBERIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBCTR_SPEC, RBERIEN_A, O>;
impl<'a, const O: u8> RBERIEN_W<'a, O> {
    #[doc = "The receive buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RBERIEN_A::VALUE1)
    }
    #[doc = "The receive buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RBERIEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    pub fn srbtm(&self) -> SRBTM_R {
        SRBTM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    pub fn srbten(&self) -> SRBTEN_R {
        SRBTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn srbinp(&self) -> SRBINP_R {
        SRBINP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn arbinp(&self) -> ARBINP_R {
        ARBINP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    pub fn rcim(&self) -> RCIM_R {
        RCIM_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    pub fn rnm(&self) -> RNM_R {
        RNM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LOF_R {
        LOF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn arbien(&self) -> ARBIEN_R {
        ARBIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn srbien(&self) -> SRBIEN_R {
        SRBIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn rberien(&self) -> RBERIEN_R {
        RBERIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn dptr(&mut self) -> DPTR_W<0> {
        DPTR_W::new(self)
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<8> {
        LIMIT_W::new(self)
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn srbtm(&mut self) -> SRBTM_W<14> {
        SRBTM_W::new(self)
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srbten(&mut self) -> SRBTEN_W<15> {
        SRBTEN_W::new(self)
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn srbinp(&mut self) -> SRBINP_W<16> {
        SRBINP_W::new(self)
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn arbinp(&mut self) -> ARBINP_W<19> {
        ARBINP_W::new(self)
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rcim(&mut self) -> RCIM_W<22> {
        RCIM_W::new(self)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<24> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnm(&mut self) -> RNM_W<27> {
        RNM_W::new(self)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn lof(&mut self) -> LOF_W<28> {
        LOF_W::new(self)
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arbien(&mut self) -> ARBIEN_W<29> {
        ARBIEN_W::new(self)
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srbien(&mut self) -> SRBIEN_W<30> {
        SRBIEN_W::new(self)
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rberien(&mut self) -> RBERIEN_W<31> {
        RBERIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbctr](index.html) module"]
pub struct RBCTR_SPEC;
impl crate::RegisterSpec for RBCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbctr::R](R) reader structure"]
impl crate::Readable for RBCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbctr::W](W) writer structure"]
impl crate::Writable for RBCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBCTR to value 0"]
impl crate::Resettable for RBCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
