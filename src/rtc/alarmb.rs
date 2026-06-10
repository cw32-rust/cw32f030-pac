#[doc = "Register `ALARMB` reader"]
pub type R = crate::R<AlarmbSpec>;
#[doc = "Register `ALARMB` writer"]
pub type W = crate::W<AlarmbSpec>;
#[doc = "Field `SECOND` reader - desc SECOND"]
pub type SecondR = crate::FieldReader;
#[doc = "Field `SECOND` writer - desc SECOND"]
pub type SecondW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SECONDEN` reader - desc SECONDEN"]
pub type SecondenR = crate::BitReader;
#[doc = "Field `SECONDEN` writer - desc SECONDEN"]
pub type SecondenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINUTE` reader - desc MINUTE"]
pub type MinuteR = crate::FieldReader;
#[doc = "Field `MINUTE` writer - desc MINUTE"]
pub type MinuteW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MINUTEEN` reader - desc MINUTEEN"]
pub type MinuteenR = crate::BitReader;
#[doc = "Field `MINUTEEN` writer - desc MINUTEEN"]
pub type MinuteenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOUR` reader - desc HOUR"]
pub type HourR = crate::FieldReader;
#[doc = "Field `HOUR` writer - desc HOUR"]
pub type HourW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HOUREN` reader - desc HOUREN"]
pub type HourenR = crate::BitReader;
#[doc = "Field `HOUREN` writer - desc HOUREN"]
pub type HourenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEEKMASK` reader - desc WEEKMASK"]
pub type WeekmaskR = crate::FieldReader;
#[doc = "Field `WEEKMASK` writer - desc WEEKMASK"]
pub type WeekmaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    pub fn second(&self) -> SecondR {
        SecondR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - desc SECONDEN"]
    #[inline(always)]
    pub fn seconden(&self) -> SecondenR {
        SecondenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    pub fn minute(&self) -> MinuteR {
        MinuteR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - desc MINUTEEN"]
    #[inline(always)]
    pub fn minuteen(&self) -> MinuteenR {
        MinuteenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    pub fn hour(&self) -> HourR {
        HourR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - desc HOUREN"]
    #[inline(always)]
    pub fn houren(&self) -> HourenR {
        HourenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - desc WEEKMASK"]
    #[inline(always)]
    pub fn weekmask(&self) -> WeekmaskR {
        WeekmaskR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - desc SECOND"]
    #[inline(always)]
    pub fn second(&mut self) -> SecondW<'_, AlarmbSpec> {
        SecondW::new(self, 0)
    }
    #[doc = "Bit 7 - desc SECONDEN"]
    #[inline(always)]
    pub fn seconden(&mut self) -> SecondenW<'_, AlarmbSpec> {
        SecondenW::new(self, 7)
    }
    #[doc = "Bits 8:14 - desc MINUTE"]
    #[inline(always)]
    pub fn minute(&mut self) -> MinuteW<'_, AlarmbSpec> {
        MinuteW::new(self, 8)
    }
    #[doc = "Bit 15 - desc MINUTEEN"]
    #[inline(always)]
    pub fn minuteen(&mut self) -> MinuteenW<'_, AlarmbSpec> {
        MinuteenW::new(self, 15)
    }
    #[doc = "Bits 16:21 - desc HOUR"]
    #[inline(always)]
    pub fn hour(&mut self) -> HourW<'_, AlarmbSpec> {
        HourW::new(self, 16)
    }
    #[doc = "Bit 23 - desc HOUREN"]
    #[inline(always)]
    pub fn houren(&mut self) -> HourenW<'_, AlarmbSpec> {
        HourenW::new(self, 23)
    }
    #[doc = "Bits 24:30 - desc WEEKMASK"]
    #[inline(always)]
    pub fn weekmask(&mut self) -> WeekmaskW<'_, AlarmbSpec> {
        WeekmaskW::new(self, 24)
    }
}
#[doc = "Alarm - B\n\nYou can [`read`](crate::Reg::read) this register and get [`alarmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlarmbSpec;
impl crate::RegisterSpec for AlarmbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarmb::R`](R) reader structure"]
impl crate::Readable for AlarmbSpec {}
#[doc = "`write(|w| ..)` method takes [`alarmb::W`](W) writer structure"]
impl crate::Writable for AlarmbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALARMB to value 0"]
impl crate::Resettable for AlarmbSpec {}
