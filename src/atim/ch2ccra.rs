#[doc = "Register `CH2CCRA` reader"]
pub type R = crate::R<Ch2ccraSpec>;
#[doc = "Register `CH2CCRA` writer"]
pub type W = crate::W<Ch2ccraSpec>;
#[doc = "Field `CCR2A` reader - desc CCR2A"]
pub type Ccr2aR = crate::FieldReader<u16>;
#[doc = "Field `CCR2A` writer - desc CCR2A"]
pub type Ccr2aW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR2A"]
    #[inline(always)]
    pub fn ccr2a(&self) -> Ccr2aR {
        Ccr2aR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR2A"]
    #[inline(always)]
    pub fn ccr2a(&mut self) -> Ccr2aW<'_, Ch2ccraSpec> {
        Ccr2aW::new(self, 0)
    }
}
#[doc = "desc CH2CCRA\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ccra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ccra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2ccraSpec;
impl crate::RegisterSpec for Ch2ccraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2ccra::R`](R) reader structure"]
impl crate::Readable for Ch2ccraSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2ccra::W`](W) writer structure"]
impl crate::Writable for Ch2ccraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2CCRA to value 0"]
impl crate::Resettable for Ch2ccraSpec {}
