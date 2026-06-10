#[doc = "Register `DSTADDR4` reader"]
pub type R = crate::R<Dstaddr4Spec>;
#[doc = "Register `DSTADDR4` writer"]
pub type W = crate::W<Dstaddr4Spec>;
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
    pub fn dstaddr(&mut self) -> DstaddrW<'_, Dstaddr4Spec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel4 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dstaddr4Spec;
impl crate::RegisterSpec for Dstaddr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr4::R`](R) reader structure"]
impl crate::Readable for Dstaddr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dstaddr4::W`](W) writer structure"]
impl crate::Writable for Dstaddr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSTADDR4 to value 0"]
impl crate::Resettable for Dstaddr4Spec {}
