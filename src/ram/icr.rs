#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `PARITY` reader - desc PARITY"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - desc PARITY"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, IcrSpec> {
        ParityW::new(self, 0)
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
