#[doc = "Register `APBRST1` reader"]
pub type R = crate::R<Apbrst1Spec>;
#[doc = "Register `APBRST1` writer"]
pub type W = crate::W<Apbrst1Spec>;
#[doc = "Field `GTIM1` reader - desc GTIM1"]
pub type Gtim1R = crate::BitReader;
#[doc = "Field `GTIM1` writer - desc GTIM1"]
pub type Gtim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTIM2` reader - desc GTIM2"]
pub type Gtim2R = crate::BitReader;
#[doc = "Field `GTIM2` writer - desc GTIM2"]
pub type Gtim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - desc WWDT"]
pub type WwdtR = crate::BitReader;
#[doc = "Field `WWDT` writer - desc WWDT"]
pub type WwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IwdtR = crate::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2` reader - desc SPI2"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI2` writer - desc SPI2"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - desc UART2"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3` reader - desc UART3"]
pub type Uart3R = crate::BitReader;
#[doc = "Field `UART3` writer - desc UART3"]
pub type Uart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - desc I2C1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - desc I2C1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - desc I2C2"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - desc I2C2"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
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
    #[doc = "Bit 3 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&self) -> WwdtR {
        WwdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IwdtR {
        IwdtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&mut self) -> Gtim1W<'_, Apbrst1Spec> {
        Gtim1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&mut self) -> Gtim2W<'_, Apbrst1Spec> {
        Gtim2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Apbrst1Spec> {
        RtcW::new(self, 3)
    }
    #[doc = "Bit 4 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WwdtW<'_, Apbrst1Spec> {
        WwdtW::new(self, 4)
    }
    #[doc = "Bit 5 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&mut self) -> IwdtW<'_, Apbrst1Spec> {
        IwdtW::new(self, 5)
    }
    #[doc = "Bit 6 - desc SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> Spi2W<'_, Apbrst1Spec> {
        Spi2W::new(self, 6)
    }
    #[doc = "Bit 7 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<'_, Apbrst1Spec> {
        Uart2W::new(self, 7)
    }
    #[doc = "Bit 8 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&mut self) -> Uart3W<'_, Apbrst1Spec> {
        Uart3W::new(self, 8)
    }
    #[doc = "Bit 11 - desc I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Apbrst1Spec> {
        I2c1W::new(self, 11)
    }
    #[doc = "Bit 12 - desc I2C2"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, Apbrst1Spec> {
        I2c2W::new(self, 12)
    }
}
#[doc = "APB Reset Control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apbrst1Spec;
impl crate::RegisterSpec for Apbrst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrst1::R`](R) reader structure"]
impl crate::Readable for Apbrst1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbrst1::W`](W) writer structure"]
impl crate::Writable for Apbrst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APBRST1 to value 0"]
impl crate::Resettable for Apbrst1Spec {}
