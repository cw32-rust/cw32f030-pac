#[doc = "Register `COMPEN` reader"]
pub type R = crate::R<CompenSpec>;
#[doc = "Register `COMPEN` writer"]
pub type W = crate::W<CompenSpec>;
#[doc = "Field `COMP` reader - desc COMP"]
pub type CompR = crate::FieldReader<u16>;
#[doc = "Field `COMP` writer - desc COMP"]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STEP` reader - desc STEP"]
pub type StepR = crate::FieldReader;
#[doc = "Field `STEP` writer - desc STEP"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIGN` reader - desc SIGN"]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - desc SIGN"]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQ` reader - desc FREQ"]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - desc FREQ"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - desc COMP"]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - desc STEP"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - desc SIGN"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<'_, CompenSpec> {
        CompW::new(self, 0)
    }
    #[doc = "Bits 12:13 - desc STEP"]
    #[inline(always)]
    pub fn step(&mut self) -> StepW<'_, CompenSpec> {
        StepW::new(self, 12)
    }
    #[doc = "Bit 14 - desc SIGN"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, CompenSpec> {
        SignW::new(self, 14)
    }
    #[doc = "Bit 15 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CompenSpec> {
        EnW::new(self, 15)
    }
    #[doc = "Bits 16:19 - desc FREQ"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, CompenSpec> {
        FreqW::new(self, 16)
    }
}
#[doc = "Compen register\n\nYou can [`read`](crate::Reg::read) this register and get [`compen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompenSpec;
impl crate::RegisterSpec for CompenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compen::R`](R) reader structure"]
impl crate::Readable for CompenSpec {}
#[doc = "`write(|w| ..)` method takes [`compen::W`](W) writer structure"]
impl crate::Writable for CompenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMPEN to value 0"]
impl crate::Resettable for CompenSpec {}
