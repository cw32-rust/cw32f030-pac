#[doc = "Register `RISEIE` reader"]
pub type R = crate::R<RiseieSpec>;
#[doc = "Register `RISEIE` writer"]
pub type W = crate::W<RiseieSpec>;
#[doc = "Field `PIN0` reader - desc PIN0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN0` writer - desc PIN0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` reader - desc PIN1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN1` writer - desc PIN1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` reader - desc PIN3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN3` writer - desc PIN3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` reader - desc PIN6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN6` writer - desc PIN6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` reader - desc PIN7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN7` writer - desc PIN7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, RiseieSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, RiseieSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, RiseieSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, RiseieSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, RiseieSpec> {
        Pin7W::new(self, 7)
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
