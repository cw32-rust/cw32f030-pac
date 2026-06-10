#[doc = "Register `DSTADDR` reader"]
pub type R = crate::R<DstaddrSpec>;
#[doc = "Register `DSTADDR` writer"]
pub type W = crate::W<DstaddrSpec>;
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
    pub fn dstaddr(&mut self) -> DstaddrW<'_, DstaddrSpec> {
        DstaddrW::new(self, 0)
    }
}
#[doc = "Channel.y destination address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dstaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstaddrSpec;
impl crate::RegisterSpec for DstaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstaddr::R`](R) reader structure"]
impl crate::Readable for DstaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dstaddr::W`](W) writer structure"]
impl crate::Writable for DstaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSTADDR to value 0"]
impl crate::Resettable for DstaddrSpec {}
