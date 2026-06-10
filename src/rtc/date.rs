#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `DAY` reader - desc DAY"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - desc DAY"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MONTH` reader - desc MONTH"]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - desc MONTH"]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEAR` reader - desc YEAR"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - desc YEAR"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WEEK` reader - desc WEEK"]
pub type WeekR = crate::FieldReader;
#[doc = "Field `WEEK` writer - desc WEEK"]
pub type WeekW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - desc DAY"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc MONTH"]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc YEAR"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - desc WEEK"]
    #[inline(always)]
    pub fn week(&self) -> WeekR {
        WeekR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DAY"]
    #[inline(always)]
    pub fn day(&mut self) -> DayW<'_, DateSpec> {
        DayW::new(self, 0)
    }
    #[doc = "Bits 8:15 - desc MONTH"]
    #[inline(always)]
    pub fn month(&mut self) -> MonthW<'_, DateSpec> {
        MonthW::new(self, 8)
    }
    #[doc = "Bits 16:23 - desc YEAR"]
    #[inline(always)]
    pub fn year(&mut self) -> YearW<'_, DateSpec> {
        YearW::new(self, 16)
    }
    #[doc = "Bits 24:26 - desc WEEK"]
    #[inline(always)]
    pub fn week(&mut self) -> WeekW<'_, DateSpec> {
        WeekW::new(self, 24)
    }
}
#[doc = "Time.Second register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::Resettable for DateSpec {}
