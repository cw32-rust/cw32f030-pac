#[doc = "Register `APBRST2` reader"]
pub type R = crate::R<Apbrst2Spec>;
#[doc = "Register `APBRST2` writer"]
pub type W = crate::W<Apbrst2Spec>;
#[doc = "Field `ADC` reader - desc ADC"]
pub type AdcR = crate::BitReader;
#[doc = "Field `ADC` writer - desc ADC"]
pub type AdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC` reader - desc VC"]
pub type VcR = crate::BitReader;
#[doc = "Field `VC` writer - desc VC"]
pub type VcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type AtimR = crate::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type AtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - desc UART1"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM3` reader - desc GTIM3"]
pub type Gtim3R = crate::BitReader;
#[doc = "Field `GTIM3` writer - desc GTIM3"]
pub type Gtim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM4` reader - desc GTIM4"]
pub type Gtim4R = crate::BitReader;
#[doc = "Field `GTIM4` writer - desc GTIM4"]
pub type Gtim4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM` reader - desc BTIM"]
pub type BtimR = crate::BitReader;
#[doc = "Field `BTIM` writer - desc BTIM"]
pub type BtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWT` reader - desc AWT"]
pub type AwtR = crate::BitReader;
#[doc = "Field `AWT` writer - desc AWT"]
pub type AwtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - desc ADC"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc VC"]
    #[inline(always)]
    pub fn vc(&self) -> VcR {
        VcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> AtimR {
        AtimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&self) -> Gtim3R {
        Gtim3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&self) -> Gtim4R {
        Gtim4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BTIM"]
    #[inline(always)]
    pub fn btim(&self) -> BtimR {
        BtimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc AWT"]
    #[inline(always)]
    pub fn awt(&self) -> AwtR {
        AwtR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - desc ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<'_, Apbrst2Spec> {
        AdcW::new(self, 2)
    }
    #[doc = "Bit 4 - desc VC"]
    #[inline(always)]
    pub fn vc(&mut self) -> VcW<'_, Apbrst2Spec> {
        VcW::new(self, 4)
    }
    #[doc = "Bit 7 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&mut self) -> AtimW<'_, Apbrst2Spec> {
        AtimW::new(self, 7)
    }
    #[doc = "Bit 8 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, Apbrst2Spec> {
        Spi1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, Apbrst2Spec> {
        Uart1W::new(self, 9)
    }
    #[doc = "Bit 10 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&mut self) -> Gtim3W<'_, Apbrst2Spec> {
        Gtim3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&mut self) -> Gtim4W<'_, Apbrst2Spec> {
        Gtim4W::new(self, 11)
    }
    #[doc = "Bit 12 - desc BTIM"]
    #[inline(always)]
    pub fn btim(&mut self) -> BtimW<'_, Apbrst2Spec> {
        BtimW::new(self, 12)
    }
    #[doc = "Bit 13 - desc AWT"]
    #[inline(always)]
    pub fn awt(&mut self) -> AwtW<'_, Apbrst2Spec> {
        AwtW::new(self, 13)
    }
}
#[doc = "APB Reset Control Reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrst2Spec;
impl crate::RegisterSpec for Apbrst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst2::R`](R) reader structure"]
impl crate::Readable for Apbrst2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrst2::W`](W) writer structure"]
impl crate::Writable for Apbrst2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRST2 to value 0"]
impl crate::Resettable for Apbrst2Spec {}
