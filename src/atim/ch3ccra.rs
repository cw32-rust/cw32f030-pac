#[doc = "Register `CH3CCRA` reader"]
pub type R = crate::R<Ch3ccraSpec>;
#[doc = "Register `CH3CCRA` writer"]
pub type W = crate::W<Ch3ccraSpec>;
#[doc = "Field `CCR3A` reader - desc CCR3A"]
pub type Ccr3aR = crate::FieldReader<u16>;
#[doc = "Field `CCR3A` writer - desc CCR3A"]
pub type Ccr3aW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR3A"]
    #[inline(always)]
    pub fn ccr3a(&self) -> Ccr3aR {
        Ccr3aR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR3A"]
    #[inline(always)]
    pub fn ccr3a(&mut self) -> Ccr3aW<'_, Ch3ccraSpec> {
        Ccr3aW::new(self, 0)
    }
}
#[doc = "desc CH3CCRA\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ccra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ccra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ccraSpec;
impl crate::RegisterSpec for Ch3ccraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccra::R`](R) reader structure"]
impl crate::Readable for Ch3ccraSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ccra::W`](W) writer structure"]
impl crate::Writable for Ch3ccraSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CCRA to value 0"]
impl crate::Resettable for Ch3ccraSpec {}
