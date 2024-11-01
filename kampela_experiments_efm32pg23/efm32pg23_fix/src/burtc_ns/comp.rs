#[doc = "Register `COMP` reader"]
pub type R = crate::R<CompSpec>;
#[doc = "Register `COMP` writer"]
pub type W = crate::W<CompSpec>;
#[doc = "Field `COMP` reader - Compare Value"]
pub type CompR = crate::FieldReader<u32>;
#[doc = "Field `COMP` writer - Compare Value"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> CompW<CompSpec> {
        CompW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`comp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompSpec;
impl crate::RegisterSpec for CompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp::R`](R) reader structure"]
impl crate::Readable for CompSpec {}
#[doc = "`write(|w| ..)` method takes [`comp::W`](W) writer structure"]
impl crate::Writable for CompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP to value 0"]
impl crate::Resettable for CompSpec {
    const RESET_VALUE: u32 = 0;
}
