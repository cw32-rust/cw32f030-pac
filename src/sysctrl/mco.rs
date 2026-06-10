#[doc = "Register `MCO` reader"]
pub type R = crate::R<McoSpec>;
#[doc = "Register `MCO` writer"]
pub type W = crate::W<McoSpec>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, McoSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, McoSpec> {
        DivW::new(self, 4)
    }
}
#[doc = "Master Clock Output Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`mco::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McoSpec;
impl crate::RegisterSpec for McoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mco::R`](R) reader structure"]
impl crate::Readable for McoSpec {}
#[doc = "`write(|w| ..)` method takes [`mco::W`](W) writer structure"]
impl crate::Writable for McoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCO to value 0"]
impl crate::Resettable for McoSpec {}
