#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `CHMUX` reader - desc CHMUX"]
pub type ChmuxR = crate::FieldReader;
#[doc = "Field `CHMUX` writer - desc CHMUX"]
pub type ChmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DISCARD` reader - desc DISCARD"]
pub type DiscardR = crate::BitReader;
#[doc = "Field `DISCARD` writer - desc DISCARD"]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIGN` reader - desc ALIGN"]
pub type AlignR = crate::BitReader;
#[doc = "Field `ALIGN` writer - desc ALIGN"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTCH` reader - desc WDTCH"]
pub type WdtchR = crate::FieldReader;
#[doc = "Field `WDTCH` writer - desc WDTCH"]
pub type WdtchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WDTALL` reader - desc WDTALL"]
pub type WdtallR = crate::BitReader;
#[doc = "Field `WDTALL` writer - desc WDTALL"]
pub type WdtallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc CHMUX"]
    #[inline(always)]
    pub fn chmux(&self) -> ChmuxR {
        ChmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - desc DISCARD"]
    #[inline(always)]
    pub fn discard(&self) -> DiscardR {
        DiscardR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - desc WDTCH"]
    #[inline(always)]
    pub fn wdtch(&self) -> WdtchR {
        WdtchR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - desc WDTALL"]
    #[inline(always)]
    pub fn wdtall(&self) -> WdtallR {
        WdtallR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CHMUX"]
    #[inline(always)]
    pub fn chmux(&mut self) -> ChmuxW<'_, Cr1Spec> {
        ChmuxW::new(self, 0)
    }
    #[doc = "Bit 5 - desc DISCARD"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<'_, Cr1Spec> {
        DiscardW::new(self, 5)
    }
    #[doc = "Bit 6 - desc ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<'_, Cr1Spec> {
        AlignW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Cr1Spec> {
        DmaenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - desc WDTCH"]
    #[inline(always)]
    pub fn wdtch(&mut self) -> WdtchW<'_, Cr1Spec> {
        WdtchW::new(self, 8)
    }
    #[doc = "Bit 13 - desc WDTALL"]
    #[inline(always)]
    pub fn wdtall(&mut self) -> WdtallW<'_, Cr1Spec> {
        WdtallW::new(self, 13)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
