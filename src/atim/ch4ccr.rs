#[doc = "Register `CH4CCR` reader"]
pub type R = crate::R<Ch4ccrSpec>;
#[doc = "Register `CH4CCR` writer"]
pub type W = crate::W<Ch4ccrSpec>;
#[doc = "Field `CCR4` reader - desc CCR4"]
pub type Ccr4R = crate::FieldReader<u16>;
#[doc = "Field `CCR4` writer - desc CCR4"]
pub type Ccr4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc CCR4"]
    #[inline(always)]
    pub fn ccr4(&self) -> Ccr4R {
        Ccr4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CCR4"]
    #[inline(always)]
    pub fn ccr4(&mut self) -> Ccr4W<'_, Ch4ccrSpec> {
        Ccr4W::new(self, 0)
    }
}
#[doc = "desc CH4CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4ccrSpec;
impl crate::RegisterSpec for Ch4ccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4ccr::R`](R) reader structure"]
impl crate::Readable for Ch4ccrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4ccr::W`](W) writer structure"]
impl crate::Writable for Ch4ccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4CCR to value 0"]
impl crate::Resettable for Ch4ccrSpec {}
