#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TDR` writer - desc TDR"]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl W {
    #[doc = "Bits 0:8 - desc TDR"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<'_, TdrSpec> {
        TdrW::new(self, 0)
    }
}
#[doc = "Data reg for write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TdrSpec {}
