#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AhbenSpec>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AhbenSpec>;
#[doc = "Field `DMA` reader - desc DMA"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - desc DMA"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH` reader - desc FLASH"]
pub type FlashR = crate::BitReader;
#[doc = "Field `FLASH` writer - desc FLASH"]
pub type FlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - desc CRC"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - desc CRC"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - desc GPIOA"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - desc GPIOA"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - desc GPIOB"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - desc GPIOB"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOC` reader - desc GPIOC"]
pub type GpiocR = crate::BitReader;
#[doc = "Field `GPIOC` writer - desc GPIOC"]
pub type GpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOF` reader - desc GPIOF"]
pub type GpiofR = crate::BitReader;
#[doc = "Field `GPIOF` writer - desc GPIOF"]
pub type GpiofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GPIOC"]
    #[inline(always)]
    pub fn gpioc(&self) -> GpiocR {
        GpiocR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - desc GPIOF"]
    #[inline(always)]
    pub fn gpiof(&self) -> GpiofR {
        GpiofR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, AhbenSpec> {
        DmaW::new(self, 0)
    }
    #[doc = "Bit 1 - desc FLASH"]
    #[inline(always)]
    pub fn flash(&mut self) -> FlashW<'_, AhbenSpec> {
        FlashW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, AhbenSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 4 - desc GPIOA"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<'_, AhbenSpec> {
        GpioaW::new(self, 4)
    }
    #[doc = "Bit 5 - desc GPIOB"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<'_, AhbenSpec> {
        GpiobW::new(self, 5)
    }
    #[doc = "Bit 6 - desc GPIOC"]
    #[inline(always)]
    pub fn gpioc(&mut self) -> GpiocW<'_, AhbenSpec> {
        GpiocW::new(self, 6)
    }
    #[doc = "Bit 9 - desc GPIOF"]
    #[inline(always)]
    pub fn gpiof(&mut self) -> GpiofW<'_, AhbenSpec> {
        GpiofW::new(self, 9)
    }
}
#[doc = "AHB Clock Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenSpec;
impl crate::RegisterSpec for AhbenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AhbenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AhbenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBEN to value 0"]
impl crate::Resettable for AhbenSpec {}
