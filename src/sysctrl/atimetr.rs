#[doc = "Register `ATIMETR` reader"]
pub type R = crate::R<AtimetrSpec>;
#[doc = "Register `ATIMETR` writer"]
pub type W = crate::W<AtimetrSpec>;
#[doc = "Field `ATIMETR` reader - desc ATIMETR"]
pub type AtimetrR = crate::FieldReader;
#[doc = "Field `ATIMETR` writer - desc ATIMETR"]
pub type AtimetrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc ATIMETR"]
    #[inline(always)]
    pub fn atimetr(&self) -> AtimetrR {
        AtimetrR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc ATIMETR"]
    #[inline(always)]
    pub fn atimetr(&mut self) -> AtimetrW<'_, AtimetrSpec> {
        AtimetrW::new(self, 0)
    }
}
#[doc = "ATIM ETR Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`atimetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atimetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtimetrSpec;
impl crate::RegisterSpec for AtimetrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atimetr::R`](R) reader structure"]
impl crate::Readable for AtimetrSpec {}
#[doc = "`write(|w| ..)` method takes [`atimetr::W`](W) writer structure"]
impl crate::Writable for AtimetrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ATIMETR to value 0"]
impl crate::Resettable for AtimetrSpec {}
