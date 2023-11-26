#[doc = "Register `wdogcfg` reader"]
pub type R = crate::R<WDOGCFG_SPEC>;
#[doc = "Register `wdogcfg` writer"]
pub type W = crate::W<WDOGCFG_SPEC>;
#[doc = "Field `wdogscale` reader - Counter scale value."]
pub type WDOGSCALE_R = crate::FieldReader;
#[doc = "Field `wdogscale` writer - Counter scale value."]
pub type WDOGSCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wdogrsten` reader - Controls whether the comparator output can set the wdogrst bit and hence cause a full reset."]
pub type WDOGRSTEN_R = crate::BitReader;
#[doc = "Field `wdogrsten` writer - Controls whether the comparator output can set the wdogrst bit and hence cause a full reset."]
pub type WDOGRSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdogzerocmp` reader - Reset counter to zero after match."]
pub type WDOGZEROCMP_R = crate::BitReader;
#[doc = "Field `wdogzerocmp` writer - Reset counter to zero after match."]
pub type WDOGZEROCMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdogenalways` reader - Enable Always - run continuously"]
pub type WDOGENALWAYS_R = crate::BitReader;
#[doc = "Field `wdogenalways` writer - Enable Always - run continuously"]
pub type WDOGENALWAYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdogcoreawake` reader - Increment the watchdog counter if the processor is not asleep"]
pub type WDOGCOREAWAKE_R = crate::BitReader;
#[doc = "Field `wdogcoreawake` writer - Increment the watchdog counter if the processor is not asleep"]
pub type WDOGCOREAWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wdogip0` reader - Interrupt 0 Pending"]
pub type WDOGIP0_R = crate::BitReader;
#[doc = "Field `wdogip0` writer - Interrupt 0 Pending"]
pub type WDOGIP0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Counter scale value."]
    #[inline(always)]
    pub fn wdogscale(&self) -> WDOGSCALE_R {
        WDOGSCALE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Controls whether the comparator output can set the wdogrst bit and hence cause a full reset."]
    #[inline(always)]
    pub fn wdogrsten(&self) -> WDOGRSTEN_R {
        WDOGRSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset counter to zero after match."]
    #[inline(always)]
    pub fn wdogzerocmp(&self) -> WDOGZEROCMP_R {
        WDOGZEROCMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Always - run continuously"]
    #[inline(always)]
    pub fn wdogenalways(&self) -> WDOGENALWAYS_R {
        WDOGENALWAYS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Increment the watchdog counter if the processor is not asleep"]
    #[inline(always)]
    pub fn wdogcoreawake(&self) -> WDOGCOREAWAKE_R {
        WDOGCOREAWAKE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 0 Pending"]
    #[inline(always)]
    pub fn wdogip0(&self) -> WDOGIP0_R {
        WDOGIP0_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Counter scale value."]
    #[inline(always)]
    #[must_use]
    pub fn wdogscale(&mut self) -> WDOGSCALE_W<WDOGCFG_SPEC> {
        WDOGSCALE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Controls whether the comparator output can set the wdogrst bit and hence cause a full reset."]
    #[inline(always)]
    #[must_use]
    pub fn wdogrsten(&mut self) -> WDOGRSTEN_W<WDOGCFG_SPEC> {
        WDOGRSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reset counter to zero after match."]
    #[inline(always)]
    #[must_use]
    pub fn wdogzerocmp(&mut self) -> WDOGZEROCMP_W<WDOGCFG_SPEC> {
        WDOGZEROCMP_W::new(self, 9)
    }
    #[doc = "Bit 12 - Enable Always - run continuously"]
    #[inline(always)]
    #[must_use]
    pub fn wdogenalways(&mut self) -> WDOGENALWAYS_W<WDOGCFG_SPEC> {
        WDOGENALWAYS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Increment the watchdog counter if the processor is not asleep"]
    #[inline(always)]
    #[must_use]
    pub fn wdogcoreawake(&mut self) -> WDOGCOREAWAKE_W<WDOGCFG_SPEC> {
        WDOGCOREAWAKE_W::new(self, 13)
    }
    #[doc = "Bit 28 - Interrupt 0 Pending"]
    #[inline(always)]
    #[must_use]
    pub fn wdogip0(&mut self) -> WDOGIP0_W<WDOGCFG_SPEC> {
        WDOGIP0_W::new(self, 28)
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
#[doc = "wdog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGCFG_SPEC;
impl crate::RegisterSpec for WDOGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogcfg::R`](R) reader structure"]
impl crate::Readable for WDOGCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdogcfg::W`](W) writer structure"]
impl crate::Writable for WDOGCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdogcfg to value 0"]
impl crate::Resettable for WDOGCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
