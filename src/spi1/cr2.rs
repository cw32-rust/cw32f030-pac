#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `HDOE` reader - desc HDOE"]
pub type HdoeR = crate::BitReader;
#[doc = "Field `HDOE` writer - desc HDOE"]
pub type HdoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc HDOE"]
    #[inline(always)]
    pub fn hdoe(&self) -> HdoeR {
        HdoeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc HDOE"]
    #[inline(always)]
    pub fn hdoe(&mut self) -> HdoeW<'_, Cr2Spec> {
        HdoeW::new(self, 0)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
