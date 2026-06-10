#[doc = "Register `RISEIE` reader"]
pub type R = crate::R<RiseieSpec>;
#[doc = "Register `RISEIE` writer"]
pub type W = crate::W<RiseieSpec>;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type Pin13R = crate::BitReader;
#[doc = "Field `PIN13` writer - desc PIN13"]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type Pin14R = crate::BitReader;
#[doc = "Field `PIN14` writer - desc PIN14"]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type Pin15R = crate::BitReader;
#[doc = "Field `PIN15` writer - desc PIN15"]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&mut self) -> Pin13W<'_, RiseieSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&mut self) -> Pin14W<'_, RiseieSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&mut self) -> Pin15W<'_, RiseieSpec> {
        Pin15W::new(self, 15)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`riseie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`riseie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RiseieSpec;
impl crate::RegisterSpec for RiseieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`riseie::R`](R) reader structure"]
impl crate::Readable for RiseieSpec {}
#[doc = "`write(|w| ..)` method takes [`riseie::W`](W) writer structure"]
impl crate::Writable for RiseieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RISEIE to value 0"]
impl crate::Resettable for RiseieSpec {}
