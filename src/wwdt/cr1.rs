#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `WINR` reader - desc WINR"]
pub type WinrR = crate::FieldReader;
#[doc = "Field `WINR` writer - desc WINR"]
pub type WinrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PRS` reader - desc PRS"]
pub type PrsR = crate::FieldReader;
#[doc = "Field `PRS` writer - desc PRS"]
pub type PrsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IE` reader - desc IE"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - desc IE"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - desc WINR"]
    #[inline(always)]
    pub fn winr(&self) -> WinrR {
        WinrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:9 - desc PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - desc IE"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc WINR"]
    #[inline(always)]
    pub fn winr(&mut self) -> WinrW<'_, Cr1Spec> {
        WinrW::new(self, 0)
    }
    #[doc = "Bits 7:9 - desc PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<'_, Cr1Spec> {
        PrsW::new(self, 7)
    }
    #[doc = "Bit 10 - desc IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, Cr1Spec> {
        IeW::new(self, 10)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
