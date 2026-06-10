#[doc = "Register `CH1CCRA` reader"]
pub type R = crate::R<Ch1ccraSpec>;
#[doc = "Register `CH1CCRA` writer"]
pub type W = crate::W<Ch1ccraSpec>;
#[doc = "Field `CCR1A` reader - desc CCR1A"]
pub type Ccr1aR = crate::FieldReader<u16>;
#[doc = "Field `CCR1A` writer - desc CCR1A"]
pub type Ccr1aW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR1A"]
    #[inline(always)]
    pub fn ccr1a(&self) -> Ccr1aR {
        Ccr1aR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR1A"]
    #[inline(always)]
    pub fn ccr1a(&mut self) -> Ccr1aW<'_, Ch1ccraSpec> {
        Ccr1aW::new(self, 0)
    }
}
#[doc = "desc CH1CCRA\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ccra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ccra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ccraSpec;
impl crate::RegisterSpec for Ch1ccraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccra::R`](R) reader structure"]
impl crate::Readable for Ch1ccraSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ccra::W`](W) writer structure"]
impl crate::Writable for Ch1ccraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CCRA to value 0"]
impl crate::Resettable for Ch1ccraSpec {}
