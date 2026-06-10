#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TriggerSpec>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TriggerSpec>;
#[doc = "Field `ATIM` reader - desc ATIM"]
pub type AtimR = crate::BitReader;
#[doc = "Field `ATIM` writer - desc ATIM"]
pub type AtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type Gtim1R = crate::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type Gtim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM2` reader - desc GTIM2"]
pub type Gtim2R = crate::BitReader;
#[doc = "Field `GTIM2` writer - desc GTIM2"]
pub type Gtim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM3` reader - desc GTIM3"]
pub type Gtim3R = crate::BitReader;
#[doc = "Field `GTIM3` writer - desc GTIM3"]
pub type Gtim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM4` reader - desc GTIM4"]
pub type Gtim4R = crate::BitReader;
#[doc = "Field `GTIM4` writer - desc GTIM4"]
pub type Gtim4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM1` reader - desc BTIM1"]
pub type Btim1R = crate::BitReader;
#[doc = "Field `BTIM1` writer - desc BTIM1"]
pub type Btim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM2` reader - desc BTIM2"]
pub type Btim2R = crate::BitReader;
#[doc = "Field `BTIM2` writer - desc BTIM2"]
pub type Btim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIM3` reader - desc BTIM3"]
pub type Btim3R = crate::BitReader;
#[doc = "Field `BTIM3` writer - desc BTIM3"]
pub type Btim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - desc UART1"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - desc UART1"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - desc UART2"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3` reader - desc UART3"]
pub type Uart3R = crate::BitReader;
#[doc = "Field `UART3` writer - desc UART3"]
pub type Uart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - desc SPI1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - desc SPI1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - desc I2C2"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - desc I2C2"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&self) -> AtimR {
        AtimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&self) -> Gtim1R {
        Gtim1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&self) -> Gtim2R {
        Gtim2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&self) -> Gtim3R {
        Gtim3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&self) -> Gtim4R {
        Gtim4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BTIM1"]
    #[inline(always)]
    pub fn btim1(&self) -> Btim1R {
        Btim1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BTIM2"]
    #[inline(always)]
    pub fn btim2(&self) -> Btim2R {
        Btim2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BTIM3"]
    #[inline(always)]
    pub fn btim3(&self) -> Btim3R {
        Btim3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&mut self) -> AtimW<'_, TriggerSpec> {
        AtimW::new(self, 0)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&mut self) -> Gtim1W<'_, TriggerSpec> {
        Gtim1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&mut self) -> Gtim2W<'_, TriggerSpec> {
        Gtim2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&mut self) -> Gtim3W<'_, TriggerSpec> {
        Gtim3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&mut self) -> Gtim4W<'_, TriggerSpec> {
        Gtim4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc BTIM1"]
    #[inline(always)]
    pub fn btim1(&mut self) -> Btim1W<'_, TriggerSpec> {
        Btim1W::new(self, 5)
    }
    #[doc = "Bit 6 - desc BTIM2"]
    #[inline(always)]
    pub fn btim2(&mut self) -> Btim2W<'_, TriggerSpec> {
        Btim2W::new(self, 6)
    }
    #[doc = "Bit 7 - desc BTIM3"]
    #[inline(always)]
    pub fn btim3(&mut self) -> Btim3W<'_, TriggerSpec> {
        Btim3W::new(self, 7)
    }
    #[doc = "Bit 8 - desc UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<'_, TriggerSpec> {
        Uart1W::new(self, 8)
    }
    #[doc = "Bit 9 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, TriggerSpec> {
        Uart2W::new(self, 9)
    }
    #[doc = "Bit 10 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&mut self) -> Uart3W<'_, TriggerSpec> {
        Uart3W::new(self, 10)
    }
    #[doc = "Bit 11 - desc SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<'_, TriggerSpec> {
        Spi1W::new(self, 11)
    }
    #[doc = "Bit 12 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, TriggerSpec> {
        Spi2W::new(self, 12)
    }
    #[doc = "Bit 13 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, TriggerSpec> {
        I2c1W::new(self, 13)
    }
    #[doc = "Bit 14 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, TriggerSpec> {
        I2c2W::new(self, 14)
    }
    #[doc = "Bit 15 - desc DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, TriggerSpec> {
        DmaW::new(self, 15)
    }
}
#[doc = "desc TRIGGER\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TriggerSpec {}
