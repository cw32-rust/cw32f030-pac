#[doc = "Register `DR32` reader"]
pub type R = crate::R<Dr32Spec>;
#[doc = "Register `DR32` writer"]
pub type W = crate::W<Dr32Spec>;
#[doc = "Field `DR32` reader - desc DR32"]
pub type Dr32R = crate::FieldReader<u32>;
#[doc = "Field `DR32` writer - desc DR32"]
pub type Dr32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DR32"]
    #[inline(always)]
    pub fn dr32(&self) -> Dr32R {
        Dr32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DR32"]
    #[inline(always)]
    pub fn dr32(&mut self) -> Dr32W<'_, Dr32Spec> {
        Dr32W::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr32Spec;
impl crate::RegisterSpec for Dr32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr32::R`](R) reader structure"]
impl crate::Readable for Dr32Spec {}
#[doc = "`write(|w| ..)` method takes [`dr32::W`](W) writer structure"]
impl crate::Writable for Dr32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DR32 to value 0"]
impl crate::Resettable for Dr32Spec {}
