#[doc = "Register `lfclkmux` reader"]
pub type R = crate::R<LFCLKMUX_SPEC>;
#[doc = "Register `lfclkmux` writer"]
pub type W = crate::W<LFCLKMUX_SPEC>;
#[doc = "Field `lfextclk_sel` reader - Low Frequency Clock Source Selector"]
pub type LFEXTCLK_SEL_R = crate::BitReader<LFEXTCLK_SEL_A>;
#[doc = "Low Frequency Clock Source Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFEXTCLK_SEL_A {
    #[doc = "0: Use internal LF clock source"]
    INTERNAL = 0,
    #[doc = "1: Use external LF clock source"]
    EXTERNAL = 1,
}
impl From<LFEXTCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LFEXTCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LFEXTCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFEXTCLK_SEL_A {
        match self.bits {
            false => LFEXTCLK_SEL_A::INTERNAL,
            true => LFEXTCLK_SEL_A::EXTERNAL,
        }
    }
    #[doc = "Use internal LF clock source"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == LFEXTCLK_SEL_A::INTERNAL
    }
    #[doc = "Use external LF clock source"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == LFEXTCLK_SEL_A::EXTERNAL
    }
}
#[doc = "Field `lfextclk_sel` writer - Low Frequency Clock Source Selector"]
pub type LFEXTCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG, LFEXTCLK_SEL_A>;
impl<'a, REG> LFEXTCLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use internal LF clock source"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(LFEXTCLK_SEL_A::INTERNAL)
    }
    #[doc = "Use external LF clock source"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(LFEXTCLK_SEL_A::EXTERNAL)
    }
}
#[doc = "Field `lfextclk_mux_status` reader - Setting of the aon_lfclksel pin"]
pub type LFEXTCLK_MUX_STATUS_R = crate::BitReader<LFEXTCLK_MUX_STATUS_A>;
#[doc = "Setting of the aon_lfclksel pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFEXTCLK_MUX_STATUS_A {
    #[doc = "0: Use external LF clock source"]
    EXTERNAL = 0,
    #[doc = "1: Use clock source selected by lfextclk_sel"]
    SW = 1,
}
impl From<LFEXTCLK_MUX_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: LFEXTCLK_MUX_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl LFEXTCLK_MUX_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LFEXTCLK_MUX_STATUS_A {
        match self.bits {
            false => LFEXTCLK_MUX_STATUS_A::EXTERNAL,
            true => LFEXTCLK_MUX_STATUS_A::SW,
        }
    }
    #[doc = "Use external LF clock source"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == LFEXTCLK_MUX_STATUS_A::EXTERNAL
    }
    #[doc = "Use clock source selected by lfextclk_sel"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == LFEXTCLK_MUX_STATUS_A::SW
    }
}
impl R {
    #[doc = "Bit 0 - Low Frequency Clock Source Selector"]
    #[inline(always)]
    pub fn lfextclk_sel(&self) -> LFEXTCLK_SEL_R {
        LFEXTCLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Setting of the aon_lfclksel pin"]
    #[inline(always)]
    pub fn lfextclk_mux_status(&self) -> LFEXTCLK_MUX_STATUS_R {
        LFEXTCLK_MUX_STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Frequency Clock Source Selector"]
    #[inline(always)]
    #[must_use]
    pub fn lfextclk_sel(&mut self) -> LFEXTCLK_SEL_W<LFCLKMUX_SPEC> {
        LFEXTCLK_SEL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Low-Frequency Clock Mux Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkmux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclkmux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFCLKMUX_SPEC;
impl crate::RegisterSpec for LFCLKMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclkmux::R`](R) reader structure"]
impl crate::Readable for LFCLKMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfclkmux::W`](W) writer structure"]
impl crate::Writable for LFCLKMUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lfclkmux to value 0"]
impl crate::Resettable for LFCLKMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
