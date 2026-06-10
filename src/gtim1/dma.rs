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
#[doc = "Field `CC1` reader - desc CC1"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC1` writer - desc CC1"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - desc CC2"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC2` writer - desc CC2"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3` reader - desc CC3"]
pub type Cc3R = crate::BitReader;
#[doc = "Field `CC3` writer - desc CC3"]
pub type Cc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4` reader - desc CC4"]
pub type Cc4R = crate::BitReader;
#[doc = "Field `CC4` writer - desc CC4"]
pub type Cc4W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - desc CC1"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CC2"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CC3"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CC4"]
    #[inline(always)]
    pub fn cc4(&self) -> Cc4R {
        Cc4R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 2 - desc CC1"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<'_, DmaSpec> {
        Cc1W::new(self, 2)
    }
    #[doc = "Bit 3 - desc CC2"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, DmaSpec> {
        Cc2W::new(self, 3)
    }
    #[doc = "Bit 4 - desc CC3"]
    #[inline(always)]
    pub fn cc3(&mut self) -> Cc3W<'_, DmaSpec> {
        Cc3W::new(self, 4)
    }
    #[doc = "Bit 5 - desc CC4"]
    #[inline(always)]
    pub fn cc4(&mut self) -> Cc4W<'_, DmaSpec> {
        Cc4W::new(self, 5)
    }
}
#[doc = "DMA Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
