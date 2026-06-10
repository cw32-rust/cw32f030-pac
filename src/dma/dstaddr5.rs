#[doc = "Register `DSTADDR5` reader"]
pub type R = crate::R<Dstaddr5Spec>;
#[doc = "Register `DSTADDR5` writer"]
pub type W = crate::W<Dstaddr5Spec>;
#[doc = "Field `DSTADDR` reader - desc DSTADDR"]
pub type DstaddrR = crate::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - desc DSTADDR"]
pub type DstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc DSTADDR"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DstaddrR {
        DstaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc DSTADDR"]
    #[inline(always)]
    pub fn dstaddr(&mut self) -> DstaddrW<'_, Dstaddr5Spec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel5 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dstaddr5Spec;
impl crate::RegisterSpec for Dstaddr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr5::R`](R) reader structure"]
impl crate::Readable for Dstaddr5Spec {}
#[doc = "`write(|w| ..)` method takes [`dstaddr5::W`](W) writer structure"]
impl crate::Writable for Dstaddr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSTADDR5 to value 0"]
impl crate::Resettable for Dstaddr5Spec {}
