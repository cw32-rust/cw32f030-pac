#[doc = "Register `DSTADDR1` reader"]
pub type R = crate::R<Dstaddr1Spec>;
#[doc = "Register `DSTADDR1` writer"]
pub type W = crate::W<Dstaddr1Spec>;
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
    pub fn dstaddr(&mut self) -> DstaddrW<'_, Dstaddr1Spec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel1 destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dstaddr1Spec;
impl crate::RegisterSpec for Dstaddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr1::R`](R) reader structure"]
impl crate::Readable for Dstaddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dstaddr1::W`](W) writer structure"]
impl crate::Writable for Dstaddr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSTADDR1 to value 0"]
impl crate::Resettable for Dstaddr1Spec {}
