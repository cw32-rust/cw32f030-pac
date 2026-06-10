#[doc = "Register `FILTER` reader"]
pub type R = crate::R<FilterSpec>;
#[doc = "Register `FILTER` writer"]
pub type W = crate::W<FilterSpec>;
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
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FltclkR = crate::FieldReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FltclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bits 16:18 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FltclkR {
        FltclkR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, FilterSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, FilterSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, FilterSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, FilterSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, FilterSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bits 16:18 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FltclkW<'_, FilterSpec> {
        FltclkW::new(self, 16)
    }
}
#[doc = "desc FILTER\n\nYou can [`read`](crate::Reg::read) this register and get [`filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterSpec;
impl crate::RegisterSpec for FilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter::R`](R) reader structure"]
impl crate::Readable for FilterSpec {}
#[doc = "`write(|w| ..)` method takes [`filter::W`](W) writer structure"]
impl crate::Writable for FilterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FilterSpec {}
