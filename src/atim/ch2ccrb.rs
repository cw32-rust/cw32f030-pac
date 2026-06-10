#[doc = "Register `CH2CCRB` reader"]
pub type R = crate::R<Ch2ccrbSpec>;
#[doc = "Register `CH2CCRB` writer"]
pub type W = crate::W<Ch2ccrbSpec>;
#[doc = "Field `CCR2B` reader - desc CCR2B"]
pub type Ccr2bR = crate::FieldReader<u16>;
#[doc = "Field `CCR2B` writer - desc CCR2B"]
pub type Ccr2bW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR2B"]
    #[inline(always)]
    pub fn ccr2b(&self) -> Ccr2bR {
        Ccr2bR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR2B"]
    #[inline(always)]
    pub fn ccr2b(&mut self) -> Ccr2bW<'_, Ch2ccrbSpec> {
        Ccr2bW::new(self, 0)
    }
}
#[doc = "desc CH2CCRB\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ccrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ccrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2ccrbSpec;
impl crate::RegisterSpec for Ch2ccrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2ccrb::R`](R) reader structure"]
impl crate::Readable for Ch2ccrbSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2ccrb::W`](W) writer structure"]
impl crate::Writable for Ch2ccrbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2CCRB to value 0"]
impl crate::Resettable for Ch2ccrbSpec {}
