#[doc = "Register `GTIM3CAP` reader"]
pub type R = crate::R<Gtim3capSpec>;
#[doc = "Register `GTIM3CAP` writer"]
pub type W = crate::W<Gtim3capSpec>;
#[doc = "Field `CH1` reader - desc CH1"]
pub type Ch1R = crate::FieldReader;
#[doc = "Field `CH1` writer - desc CH1"]
pub type Ch1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2` reader - desc CH2"]
pub type Ch2R = crate::FieldReader;
#[doc = "Field `CH2` writer - desc CH2"]
pub type Ch2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH3` reader - desc CH3"]
pub type Ch3R = crate::FieldReader;
#[doc = "Field `CH3` writer - desc CH3"]
pub type Ch3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH4` reader - desc CH4"]
pub type Ch4R = crate::FieldReader;
#[doc = "Field `CH4` writer - desc CH4"]
pub type Ch4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc CH1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc CH2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc CH3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc CH4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc CH1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, Gtim3capSpec> {
        Ch1W::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc CH2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, Gtim3capSpec> {
        Ch2W::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc CH3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, Gtim3capSpec> {
        Ch3W::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc CH4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, Gtim3capSpec> {
        Ch4W::new(self, 12)
    }
}
#[doc = "GTIM3 CAP Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`gtim3cap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtim3cap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gtim3capSpec;
impl crate::RegisterSpec for Gtim3capSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtim3cap::R`](R) reader structure"]
impl crate::Readable for Gtim3capSpec {}
#[doc = "`write(|w| ..)` method takes [`gtim3cap::W`](W) writer structure"]
impl crate::Writable for Gtim3capSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GTIM3CAP to value 0"]
impl crate::Resettable for Gtim3capSpec {}
