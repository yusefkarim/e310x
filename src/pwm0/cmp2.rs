#[doc = "Register `cmp2` reader"]
pub type R = crate::R<CMP2_SPEC>;
#[doc = "Register `cmp2` writer"]
pub type W = crate::W<CMP2_SPEC>;
#[doc = "Field `value` reader - "]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `value` writer - "]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<CMP2_SPEC> {
        VALUE_W::new(self, 0)
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
#[doc = "Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP2_SPEC;
impl crate::RegisterSpec for CMP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2::R`](R) reader structure"]
impl crate::Readable for CMP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp2::W`](W) writer structure"]
impl crate::Writable for CMP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cmp2 to value 0"]
impl crate::Resettable for CMP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
