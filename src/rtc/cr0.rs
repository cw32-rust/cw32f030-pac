#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `INTERVAL` reader - desc INTERVAL"]
pub type IntervalR = crate::FieldReader;
#[doc = "Field `INTERVAL` writer - desc INTERVAL"]
pub type IntervalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `H24` reader - desc H24"]
pub type H24R = crate::BitReader;
#[doc = "Field `H24` writer - desc H24"]
pub type H24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC1HZ` reader - desc RTC1HZ"]
pub type Rtc1hzR = crate::FieldReader;
#[doc = "Field `RTC1HZ` writer - desc RTC1HZ"]
pub type Rtc1hzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc INTERVAL"]
    #[inline(always)]
    pub fn interval(&self) -> IntervalR {
        IntervalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc H24"]
    #[inline(always)]
    pub fn h24(&self) -> H24R {
        H24R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:6 - desc RTC1HZ"]
    #[inline(always)]
    pub fn rtc1hz(&self) -> Rtc1hzR {
        Rtc1hzR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc INTERVAL"]
    #[inline(always)]
    pub fn interval(&mut self) -> IntervalW<'_, Cr0Spec> {
        IntervalW::new(self, 0)
    }
    #[doc = "Bit 3 - desc H24"]
    #[inline(always)]
    pub fn h24(&mut self) -> H24W<'_, Cr0Spec> {
        H24W::new(self, 3)
    }
    #[doc = "Bits 5:6 - desc RTC1HZ"]
    #[inline(always)]
    pub fn rtc1hz(&mut self) -> Rtc1hzW<'_, Cr0Spec> {
        Rtc1hzW::new(self, 5)
    }
    #[doc = "Bit 7 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr0Spec> {
        StartW::new(self, 7)
    }
}
#[doc = "Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
