#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `FLTEN` reader - desc FLTEN"]
pub type FltenR = crate::BitReader;
#[doc = "Field `FLTEN` writer - desc FLTEN"]
pub type FltenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTTIME` reader - desc FLTTIME"]
pub type FlttimeR = crate::FieldReader;
#[doc = "Field `FLTTIME` writer - desc FLTTIME"]
pub type FlttimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLTCLK` reader - desc FLTCLK"]
pub type FltclkR = crate::BitReader;
#[doc = "Field `FLTCLK` writer - desc FLTCLK"]
pub type FltclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISE` reader - desc RISE"]
pub type RiseR = crate::BitReader;
#[doc = "Field `RISE` writer - desc RISE"]
pub type RiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALL` reader - desc FALL"]
pub type FallR = crate::BitReader;
#[doc = "Field `FALL` writer - desc FALL"]
pub type FallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL` reader - desc LEVEL"]
pub type LevelR = crate::BitReader;
#[doc = "Field `LEVEL` writer - desc LEVEL"]
pub type LevelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FltenR {
        FltenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&self) -> FlttimeR {
        FlttimeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&self) -> FltclkR {
        FltclkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RISE"]
    #[inline(always)]
    pub fn rise(&self) -> RiseR {
        RiseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc FALL"]
    #[inline(always)]
    pub fn fall(&self) -> FallR {
        FallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FltenW<'_, Cr1Spec> {
        FltenW::new(self, 0)
    }
    #[doc = "Bits 1:3 - desc FLTTIME"]
    #[inline(always)]
    pub fn flttime(&mut self) -> FlttimeW<'_, Cr1Spec> {
        FlttimeW::new(self, 1)
    }
    #[doc = "Bit 4 - desc FLTCLK"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FltclkW<'_, Cr1Spec> {
        FltclkW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RISE"]
    #[inline(always)]
    pub fn rise(&mut self) -> RiseW<'_, Cr1Spec> {
        RiseW::new(self, 5)
    }
    #[doc = "Bit 6 - desc FALL"]
    #[inline(always)]
    pub fn fall(&mut self) -> FallW<'_, Cr1Spec> {
        FallW::new(self, 6)
    }
    #[doc = "Bit 7 - desc LEVEL"]
    #[inline(always)]
    pub fn level(&mut self) -> LevelW<'_, Cr1Spec> {
        LevelW::new(self, 7)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
