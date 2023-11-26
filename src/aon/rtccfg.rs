#[doc = "Register `rtccfg` reader"]
pub type R = crate::R<RTCCFG_SPEC>;
#[doc = "Register `rtccfg` writer"]
pub type W = crate::W<RTCCFG_SPEC>;
#[doc = "Field `rtcscale` reader - Counter scale value."]
pub type RTCSCALE_R = crate::FieldReader;
#[doc = "Field `rtcscale` writer - Counter scale value."]
pub type RTCSCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rtcenalways` reader - Enable Always - run continuously"]
pub type RTCENALWAYS_R = crate::BitReader;
#[doc = "Field `rtcenalways` writer - Enable Always - run continuously"]
pub type RTCENALWAYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtcip0` reader - Interrupt 0 Pending"]
pub type RTCIP0_R = crate::BitReader;
#[doc = "Field `rtcip0` writer - Interrupt 0 Pending"]
pub type RTCIP0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Counter scale value."]
    #[inline(always)]
    pub fn rtcscale(&self) -> RTCSCALE_R {
        RTCSCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Enable Always - run continuously"]
    #[inline(always)]
    pub fn rtcenalways(&self) -> RTCENALWAYS_R {
        RTCENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 0 Pending"]
    #[inline(always)]
    pub fn rtcip0(&self) -> RTCIP0_R {
        RTCIP0_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Counter scale value."]
    #[inline(always)]
    #[must_use]
    pub fn rtcscale(&mut self) -> RTCSCALE_W<RTCCFG_SPEC> {
        RTCSCALE_W::new(self, 0)
    }
    #[doc = "Bit 12 - Enable Always - run continuously"]
    #[inline(always)]
    #[must_use]
    pub fn rtcenalways(&mut self) -> RTCENALWAYS_W<RTCCFG_SPEC> {
        RTCENALWAYS_W::new(self, 12)
    }
    #[doc = "Bit 28 - Interrupt 0 Pending"]
    #[inline(always)]
    #[must_use]
    pub fn rtcip0(&mut self) -> RTCIP0_W<RTCCFG_SPEC> {
        RTCIP0_W::new(self, 28)
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
#[doc = "rtc Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCFG_SPEC;
impl crate::RegisterSpec for RTCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccfg::R`](R) reader structure"]
impl crate::Readable for RTCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccfg::W`](W) writer structure"]
impl crate::Writable for RTCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtccfg to value 0"]
impl crate::Resettable for RTCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
