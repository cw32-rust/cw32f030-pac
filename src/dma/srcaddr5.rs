#[doc = "Register `SRCADDR5` reader"]
pub type R = crate::R<Srcaddr5Spec>;
#[doc = "Register `SRCADDR5` writer"]
pub type W = crate::W<Srcaddr5Spec>;
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
    pub fn srcaddr(&mut self) -> SrcaddrW<'_, Srcaddr5Spec> {
        SrcaddrW::new(self, 0)
    }
}
#[doc = "Channel5 source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcaddr5Spec;
impl crate::RegisterSpec for Srcaddr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcaddr5::R`](R) reader structure"]
impl crate::Readable for Srcaddr5Spec {}
#[doc = "`write(|w| ..)` method takes [`srcaddr5::W`](W) writer structure"]
impl crate::Writable for Srcaddr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRCADDR5 to value 0"]
impl crate::Resettable for Srcaddr5Spec {}
