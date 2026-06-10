#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `WCNT` reader - desc WCNT"]
pub type WcntR = crate::FieldReader;
#[doc = "Field `WCNT` writer - desc WCNT"]
pub type WcntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - desc WCNT"]
    #[inline(always)]
    pub fn wcnt(&self) -> WcntR {
        WcntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc WCNT"]
    #[inline(always)]
    pub fn wcnt(&mut self) -> WcntW<'_, Cr0Spec> {
        WcntW::new(self, 0)
    }
    #[doc = "Bit 7 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr0Spec> {
        EnW::new(self, 7)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
