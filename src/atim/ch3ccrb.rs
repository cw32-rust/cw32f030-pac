#[doc = "Register `CH3CCRB` reader"]
pub type R = crate::R<Ch3ccrbSpec>;
#[doc = "Register `CH3CCRB` writer"]
pub type W = crate::W<Ch3ccrbSpec>;
#[doc = "Field `CCR3B` reader - desc CCR3B"]
pub type Ccr3bR = crate::FieldReader<u16>;
#[doc = "Field `CCR3B` writer - desc CCR3B"]
pub type Ccr3bW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR3B"]
    #[inline(always)]
    pub fn ccr3b(&self) -> Ccr3bR {
        Ccr3bR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR3B"]
    #[inline(always)]
    pub fn ccr3b(&mut self) -> Ccr3bW<'_, Ch3ccrbSpec> {
        Ccr3bW::new(self, 0)
    }
}
#[doc = "desc CH3CCRB\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3ccrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3ccrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3ccrbSpec;
impl crate::RegisterSpec for Ch3ccrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3ccrb::R`](R) reader structure"]
impl crate::Readable for Ch3ccrbSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3ccrb::W`](W) writer structure"]
impl crate::Writable for Ch3ccrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3CCRB to value 0"]
impl crate::Resettable for Ch3ccrbSpec {}
