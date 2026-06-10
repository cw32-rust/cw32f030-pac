#[doc = "Register `CNT2` reader"]
pub type R = crate::R<Cnt2Spec>;
#[doc = "Register `CNT2` writer"]
pub type W = crate::W<Cnt2Spec>;
#[doc = "Field `CNT` reader - desc CNT"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - desc CNT"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REPEAT` reader - desc REPEAT"]
pub type RepeatR = crate::FieldReader;
#[doc = "Field `REPEAT` writer - desc REPEAT"]
pub type RepeatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - desc REPEAT"]
    #[inline(always)]
    pub fn repeat(&self) -> RepeatR {
        RepeatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, Cnt2Spec> {
        CntW::new(self, 0)
    }
    #[doc = "Bits 16:19 - desc REPEAT"]
    #[inline(always)]
    pub fn repeat(&mut self) -> RepeatW<'_, Cnt2Spec> {
        RepeatW::new(self, 16)
    }
}
#[doc = "Channel2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cnt2Spec;
impl crate::RegisterSpec for Cnt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt2::R`](R) reader structure"]
impl crate::Readable for Cnt2Spec {}
#[doc = "`write(|w| ..)` method takes [`cnt2::W`](W) writer structure"]
impl crate::Writable for Cnt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT2 to value 0"]
impl crate::Resettable for Cnt2Spec {}
