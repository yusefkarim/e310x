#[doc = "Register `lfrosccfg` reader"]
pub type R = crate::R<LFROSCCFG_SPEC>;
#[doc = "Register `lfrosccfg` writer"]
pub type W = crate::W<LFROSCCFG_SPEC>;
#[doc = "Field `lfroscdiv` reader - Ring Oscillator Divider Register"]
pub type LFROSCDIV_R = crate::FieldReader;
#[doc = "Field `lfroscdiv` writer - Ring Oscillator Divider Register"]
pub type LFROSCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `lfrosctrim` reader - Ring Oscillator Trim Register"]
pub type LFROSCTRIM_R = crate::FieldReader;
#[doc = "Field `lfrosctrim` writer - Ring Oscillator Trim Register"]
pub type LFROSCTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lfroscen` reader - Ring Oscillator Enable"]
pub type LFROSCEN_R = crate::BitReader;
#[doc = "Field `lfroscen` writer - Ring Oscillator Enable"]
pub type LFROSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lfroscrdy` reader - Ring Oscillator Ready"]
pub type LFROSCRDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Ring Oscillator Divider Register"]
    #[inline(always)]
    pub fn lfroscdiv(&self) -> LFROSCDIV_R {
        LFROSCDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Ring Oscillator Trim Register"]
    #[inline(always)]
    pub fn lfrosctrim(&self) -> LFROSCTRIM_R {
        LFROSCTRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Ring Oscillator Enable"]
    #[inline(always)]
    pub fn lfroscen(&self) -> LFROSCEN_R {
        LFROSCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Ring Oscillator Ready"]
    #[inline(always)]
    pub fn lfroscrdy(&self) -> LFROSCRDY_R {
        LFROSCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Ring Oscillator Divider Register"]
    #[inline(always)]
    #[must_use]
    pub fn lfroscdiv(&mut self) -> LFROSCDIV_W<LFROSCCFG_SPEC> {
        LFROSCDIV_W::new(self, 0)
    }
    #[doc = "Bits 16:20 - Ring Oscillator Trim Register"]
    #[inline(always)]
    #[must_use]
    pub fn lfrosctrim(&mut self) -> LFROSCTRIM_W<LFROSCCFG_SPEC> {
        LFROSCTRIM_W::new(self, 16)
    }
    #[doc = "Bit 30 - Ring Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfroscen(&mut self) -> LFROSCEN_W<LFROSCCFG_SPEC> {
        LFROSCEN_W::new(self, 30)
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
#[doc = "Ring Oscillator Configuration and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfrosccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfrosccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFROSCCFG_SPEC;
impl crate::RegisterSpec for LFROSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrosccfg::R`](R) reader structure"]
impl crate::Readable for LFROSCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lfrosccfg::W`](W) writer structure"]
impl crate::Writable for LFROSCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lfrosccfg to value 0"]
impl crate::Resettable for LFROSCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
