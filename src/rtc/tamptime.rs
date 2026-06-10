#[doc = "Register `TAMPTIME` reader"]
pub type R = crate::R<TamptimeSpec>;
#[doc = "Register `TAMPTIME` writer"]
pub type W = crate::W<TamptimeSpec>;
#[doc = "Field `SECOND` reader - desc SECOND"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `MINUTE` reader - desc MINUTE"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `HOUR` reader - desc HOUR"]
pub type HourR = crate::FieldReader;
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
impl W {}
#[doc = "desc TAMPTIME\n\nYou can [`read`](crate::Reg::read) this register and get [`tamptime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamptime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamptimeSpec;
impl crate::RegisterSpec for TamptimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamptime::R`](R) reader structure"]
impl crate::Readable for TamptimeSpec {}
#[doc = "`write(|w| ..)` method takes [`tamptime::W`](W) writer structure"]
impl crate::Writable for TamptimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAMPTIME to value 0"]
impl crate::Resettable for TamptimeSpec {}
