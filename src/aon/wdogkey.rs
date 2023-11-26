#[doc = "Register `wdogkey` reader"]
pub type R = crate::R<WDOGKEY_SPEC>;
#[doc = "Register `wdogkey` writer"]
pub type W = crate::W<WDOGKEY_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WDOGKEY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
#[doc = "Key Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogkey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogkey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGKEY_SPEC;
impl crate::RegisterSpec for WDOGKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogkey::R`](R) reader structure"]
impl crate::Readable for WDOGKEY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdogkey::W`](W) writer structure"]
impl crate::Writable for WDOGKEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdogkey to value 0"]
impl crate::Resettable for WDOGKEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
