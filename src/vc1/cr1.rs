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
#[doc = "Field `FALLIE` reader - desc FALLIE"]
pub type FallieR = crate::BitReader;
#[doc = "Field `FALLIE` writer - desc FALLIE"]
pub type FallieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RISEIE` reader - desc RISEIE"]
pub type RiseieR = crate::BitReader;
#[doc = "Field `RISEIE` writer - desc RISEIE"]
pub type RiseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHIE` reader - desc HIGHIE"]
pub type HighieR = crate::BitReader;
#[doc = "Field `HIGHIE` writer - desc HIGHIE"]
pub type HighieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMCLR` reader - desc ATIMCLR"]
pub type AtimclrR = crate::BitReader;
#[doc = "Field `ATIMCLR` writer - desc ATIMCLR"]
pub type AtimclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIMBK` reader - desc ATIMBK"]
pub type AtimbkR = crate::BitReader;
#[doc = "Field `ATIMBK` writer - desc ATIMBK"]
pub type AtimbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKCH1B` reader - desc BLANKCH1B"]
pub type Blankch1bR = crate::BitReader;
#[doc = "Field `BLANKCH1B` writer - desc BLANKCH1B"]
pub type Blankch1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKCH2B` reader - desc BLANKCH2B"]
pub type Blankch2bR = crate::BitReader;
#[doc = "Field `BLANKCH2B` writer - desc BLANKCH2B"]
pub type Blankch2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKCH3B` reader - desc BLANKCH3B"]
pub type Blankch3bR = crate::BitReader;
#[doc = "Field `BLANKCH3B` writer - desc BLANKCH3B"]
pub type Blankch3bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKFLT` reader - desc BLANKFLT"]
pub type BlankfltR = crate::FieldReader;
#[doc = "Field `BLANKFLT` writer - desc BLANKFLT"]
pub type BlankfltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&self) -> FallieR {
        FallieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&self) -> RiseieR {
        RiseieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&self) -> HighieR {
        HighieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ATIMCLR"]
    #[inline(always)]
    pub fn atimclr(&self) -> AtimclrR {
        AtimclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ATIMBK"]
    #[inline(always)]
    pub fn atimbk(&self) -> AtimbkR {
        AtimbkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BLANKCH1B"]
    #[inline(always)]
    pub fn blankch1b(&self) -> Blankch1bR {
        Blankch1bR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BLANKCH2B"]
    #[inline(always)]
    pub fn blankch2b(&self) -> Blankch2bR {
        Blankch2bR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BLANKCH3B"]
    #[inline(always)]
    pub fn blankch3b(&self) -> Blankch3bR {
        Blankch3bR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - desc BLANKFLT"]
    #[inline(always)]
    pub fn blankflt(&self) -> BlankfltR {
        BlankfltR::new(((self.bits >> 13) & 7) as u8)
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
    #[doc = "Bit 5 - desc FALLIE"]
    #[inline(always)]
    pub fn fallie(&mut self) -> FallieW<'_, Cr1Spec> {
        FallieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RISEIE"]
    #[inline(always)]
    pub fn riseie(&mut self) -> RiseieW<'_, Cr1Spec> {
        RiseieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc HIGHIE"]
    #[inline(always)]
    pub fn highie(&mut self) -> HighieW<'_, Cr1Spec> {
        HighieW::new(self, 7)
    }
    #[doc = "Bit 8 - desc ATIMCLR"]
    #[inline(always)]
    pub fn atimclr(&mut self) -> AtimclrW<'_, Cr1Spec> {
        AtimclrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc ATIMBK"]
    #[inline(always)]
    pub fn atimbk(&mut self) -> AtimbkW<'_, Cr1Spec> {
        AtimbkW::new(self, 9)
    }
    #[doc = "Bit 10 - desc BLANKCH1B"]
    #[inline(always)]
    pub fn blankch1b(&mut self) -> Blankch1bW<'_, Cr1Spec> {
        Blankch1bW::new(self, 10)
    }
    #[doc = "Bit 11 - desc BLANKCH2B"]
    #[inline(always)]
    pub fn blankch2b(&mut self) -> Blankch2bW<'_, Cr1Spec> {
        Blankch2bW::new(self, 11)
    }
    #[doc = "Bit 12 - desc BLANKCH3B"]
    #[inline(always)]
    pub fn blankch3b(&mut self) -> Blankch3bW<'_, Cr1Spec> {
        Blankch3bW::new(self, 12)
    }
    #[doc = "Bits 13:15 - desc BLANKFLT"]
    #[inline(always)]
    pub fn blankflt(&mut self) -> BlankfltW<'_, Cr1Spec> {
        BlankfltW::new(self, 13)
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
