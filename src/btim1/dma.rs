#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRS` reader - desc TRS"]
pub type TrsR = crate::BitReader;
#[doc = "Field `TRS` writer - desc TRS"]
pub type TrsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TRS"]
    #[inline(always)]
    pub fn trs(&self) -> TrsR {
        TrsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OV"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<'_, DmaSpec> {
        OvW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TRS"]
    #[inline(always)]
    pub fn trs(&mut self) -> TrsW<'_, DmaSpec> {
        TrsW::new(self, 1)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {}
