#[doc = "Register `TIME` reader"]
pub type R = crate::R<TimeSpec>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TimeSpec>;
#[doc = "Field `SECOND` reader - desc SECOND"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `SECOND` writer - desc SECOND"]
pub type SecondW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MINUTE` reader - desc MINUTE"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `MINUTE` writer - desc MINUTE"]
pub type MinuteW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HOUR` reader - desc HOUR"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - desc HOUR"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    pub fn second(&self) -> SecondR {
        SecondR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    pub fn minute(&self) -> MinuteR {
        MinuteR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    pub fn second(&mut self) -> SecondW<'_, TimeSpec> {
        SecondW::new(self, 0)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    pub fn minute(&mut self) -> MinuteW<'_, TimeSpec> {
        MinuteW::new(self, 8)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    pub fn hour(&mut self) -> HourW<'_, TimeSpec> {
        HourW::new(self, 16)
    }
}
#[doc = "Time.Second register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeSpec;
impl crate::RegisterSpec for TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TimeSpec {}
