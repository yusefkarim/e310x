#[doc = "Register `aoncfg` reader"]
pub type R = crate::R<AONCFG_SPEC>;
#[doc = "Register `aoncfg` writer"]
pub type W = crate::W<AONCFG_SPEC>;
#[doc = "Field `has_bandgap` reader - Bandgap feature is present"]
pub type HAS_BANDGAP_R = crate::BitReader;
#[doc = "Field `has_bod` reader - Brownout detector feature is present"]
pub type HAS_BOD_R = crate::BitReader;
#[doc = "Field `has_lfrosc` reader - Low Frequency Ring Oscillator feature is present"]
pub type HAS_LFROSC_R = crate::BitReader;
#[doc = "Field `has_lfrcosc` reader - Low Frequency RC Oscillator feature is present"]
pub type HAS_LFRCOSC_R = crate::BitReader;
#[doc = "Field `has_lfxosc` reader - Low Frequency Crystal Oscillator feature is present"]
pub type HAS_LFXOSC_R = crate::BitReader;
#[doc = "Field `has_por` reader - Power-On-Reset feature is present"]
pub type HAS_POR_R = crate::BitReader;
#[doc = "Field `has_ldo` reader - Low Dropout Regulator feature is present"]
pub type HAS_LDO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bandgap feature is present"]
    #[inline(always)]
    pub fn has_bandgap(&self) -> HAS_BANDGAP_R {
        HAS_BANDGAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brownout detector feature is present"]
    #[inline(always)]
    pub fn has_bod(&self) -> HAS_BOD_R {
        HAS_BOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Frequency Ring Oscillator feature is present"]
    #[inline(always)]
    pub fn has_lfrosc(&self) -> HAS_LFROSC_R {
        HAS_LFROSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Frequency RC Oscillator feature is present"]
    #[inline(always)]
    pub fn has_lfrcosc(&self) -> HAS_LFRCOSC_R {
        HAS_LFRCOSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Frequency Crystal Oscillator feature is present"]
    #[inline(always)]
    pub fn has_lfxosc(&self) -> HAS_LFXOSC_R {
        HAS_LFXOSC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power-On-Reset feature is present"]
    #[inline(always)]
    pub fn has_por(&self) -> HAS_POR_R {
        HAS_POR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Dropout Regulator feature is present"]
    #[inline(always)]
    pub fn has_ldo(&self) -> HAS_LDO_R {
        HAS_LDO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
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
#[doc = "AON Block Configuration Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoncfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoncfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AONCFG_SPEC;
impl crate::RegisterSpec for AONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoncfg::R`](R) reader structure"]
impl crate::Readable for AONCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aoncfg::W`](W) writer structure"]
impl crate::Writable for AONCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aoncfg to value 0"]
impl crate::Resettable for AONCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
