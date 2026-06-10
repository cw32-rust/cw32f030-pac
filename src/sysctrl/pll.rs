#[doc = "Register `PLL` reader"]
pub type R = crate::R<PllSpec>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PllSpec>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FREQIN` reader - desc FREQIN"]
pub type FreqinR = crate::FieldReader;
#[doc = "Field `FREQIN` writer - desc FREQIN"]
pub type FreqinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUL` reader - desc MUL"]
pub type MulR = crate::FieldReader;
#[doc = "Field `MUL` writer - desc MUL"]
pub type MulW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FREQOUT` reader - desc FREQOUT"]
pub type FreqoutR = crate::FieldReader;
#[doc = "Field `FREQOUT` writer - desc FREQOUT"]
pub type FreqoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAITCYCLE` reader - desc WAITCYCLE"]
pub type WaitcycleR = crate::FieldReader;
#[doc = "Field `WAITCYCLE` writer - desc WAITCYCLE"]
pub type WaitcycleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type StableR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc FREQIN"]
    #[inline(always)]
    pub fn freqin(&self) -> FreqinR {
        FreqinR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - desc MUL"]
    #[inline(always)]
    pub fn mul(&self) -> MulR {
        MulR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:11 - desc FREQOUT"]
    #[inline(always)]
    pub fn freqout(&self) -> FreqoutR {
        FreqoutR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&self) -> WaitcycleR {
        WaitcycleR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> StableR {
        StableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, PllSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc FREQIN"]
    #[inline(always)]
    pub fn freqin(&mut self) -> FreqinW<'_, PllSpec> {
        FreqinW::new(self, 2)
    }
    #[doc = "Bits 4:8 - desc MUL"]
    #[inline(always)]
    pub fn mul(&mut self) -> MulW<'_, PllSpec> {
        MulW::new(self, 4)
    }
    #[doc = "Bits 9:11 - desc FREQOUT"]
    #[inline(always)]
    pub fn freqout(&mut self) -> FreqoutW<'_, PllSpec> {
        FreqoutW::new(self, 9)
    }
    #[doc = "Bits 12:14 - desc WAITCYCLE"]
    #[inline(always)]
    pub fn waitcycle(&mut self) -> WaitcycleW<'_, PllSpec> {
        WaitcycleW::new(self, 12)
    }
}
#[doc = "PLL Control Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`pll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllSpec;
impl crate::RegisterSpec for PllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PllSpec {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PllSpec {}
