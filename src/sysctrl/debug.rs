#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Register `DEBUG` writer"]
pub type W = crate::W<DebugSpec>;
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
#[doc = "Field `BTIM123` reader - desc BTIM123"]
pub type Btim123R = crate::BitReader;
#[doc = "Field `BTIM123` writer - desc BTIM123"]
pub type Btim123W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWT` reader - desc AWT"]
pub type AwtR = crate::BitReader;
#[doc = "Field `AWT` writer - desc AWT"]
pub type AwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - desc RTC"]
pub type RtcR = crate::BitReader;
#[doc = "Field `RTC` writer - desc RTC"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IwdtR = crate::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - desc WWDT"]
pub type WwdtR = crate::BitReader;
#[doc = "Field `WWDT` writer - desc WWDT"]
pub type WwdtW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&self) -> Btim123R {
        Btim123R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AWT"]
    #[inline(always)]
    pub fn awt(&self) -> AwtR {
        AwtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IwdtR {
        IwdtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&self) -> WwdtR {
        WwdtR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc ATIM"]
    #[inline(always)]
    pub fn atim(&mut self) -> AtimW<'_, DebugSpec> {
        AtimW::new(self, 0)
    }
    #[doc = "Bit 1 - desc GTIM1"]
    #[inline(always)]
    pub fn gtim1(&mut self) -> Gtim1W<'_, DebugSpec> {
        Gtim1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc GTIM2"]
    #[inline(always)]
    pub fn gtim2(&mut self) -> Gtim2W<'_, DebugSpec> {
        Gtim2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc GTIM3"]
    #[inline(always)]
    pub fn gtim3(&mut self) -> Gtim3W<'_, DebugSpec> {
        Gtim3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc GTIM4"]
    #[inline(always)]
    pub fn gtim4(&mut self) -> Gtim4W<'_, DebugSpec> {
        Gtim4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc BTIM123"]
    #[inline(always)]
    pub fn btim123(&mut self) -> Btim123W<'_, DebugSpec> {
        Btim123W::new(self, 5)
    }
    #[doc = "Bit 6 - desc AWT"]
    #[inline(always)]
    pub fn awt(&mut self) -> AwtW<'_, DebugSpec> {
        AwtW::new(self, 6)
    }
    #[doc = "Bit 8 - desc RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, DebugSpec> {
        RtcW::new(self, 8)
    }
    #[doc = "Bit 9 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&mut self) -> IwdtW<'_, DebugSpec> {
        IwdtW::new(self, 9)
    }
    #[doc = "Bit 10 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WwdtW<'_, DebugSpec> {
        WwdtW::new(self, 10)
    }
}
#[doc = "Debug Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DebugSpec {}
