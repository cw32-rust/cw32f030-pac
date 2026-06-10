#[doc = "Register `CH1CCRB` reader"]
pub type R = crate::R<Ch1ccrbSpec>;
#[doc = "Register `CH1CCRB` writer"]
pub type W = crate::W<Ch1ccrbSpec>;
#[doc = "Field `CCR1B` reader - desc CCR1B"]
pub type Ccr1bR = crate::FieldReader<u16>;
#[doc = "Field `CCR1B` writer - desc CCR1B"]
pub type Ccr1bW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR1B"]
    #[inline(always)]
    pub fn ccr1b(&self) -> Ccr1bR {
        Ccr1bR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR1B"]
    #[inline(always)]
    pub fn ccr1b(&mut self) -> Ccr1bW<'_, Ch1ccrbSpec> {
        Ccr1bW::new(self, 0)
    }
}
#[doc = "desc CH1CCRB\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ccrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ccrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1ccrbSpec;
impl crate::RegisterSpec for Ch1ccrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1ccrb::R`](R) reader structure"]
impl crate::Readable for Ch1ccrbSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1ccrb::W`](W) writer structure"]
impl crate::Writable for Ch1ccrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CCRB to value 0"]
impl crate::Resettable for Ch1ccrbSpec {}
