#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `UD` reader - desc UD"]
pub type UdR = crate::BitReader;
#[doc = "Field `UD` writer - desc UD"]
pub type UdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - desc UD"]
    #[inline(always)]
    pub fn ud(&mut self) -> UdW<'_, IcrSpec> {
        UdW::new(self, 3)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
