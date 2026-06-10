#[doc = "Register `SRCADDR2` reader"]
pub type R = crate::R<Srcaddr2Spec>;
#[doc = "Register `SRCADDR2` writer"]
pub type W = crate::W<Srcaddr2Spec>;
#[doc = "Field `SRCADDR` reader - desc SRCADDR"]
pub type SrcaddrR = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - desc SRCADDR"]
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc SRCADDR"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc SRCADDR"]
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SrcaddrW<'_, Srcaddr2Spec> {
        SrcaddrW::new(self, 0)
    }
}
#[doc = "Channel2 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcaddr2Spec;
impl crate::RegisterSpec for Srcaddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcaddr2::R`](R) reader structure"]
impl crate::Readable for Srcaddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`srcaddr2::W`](W) writer structure"]
impl crate::Writable for Srcaddr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRCADDR2 to value 0"]
impl crate::Resettable for Srcaddr2Spec {}
