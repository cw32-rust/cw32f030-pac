#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `PC` reader - desc PC"]
pub type PcR = crate::BitReader;
#[doc = "Field `PC` writer - desc PC"]
pub type PcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGELOCK` reader - desc PAGELOCK"]
pub type PagelockR = crate::BitReader;
#[doc = "Field `PAGELOCK` writer - desc PAGELOCK"]
pub type PagelockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG` reader - desc PROG"]
pub type ProgR = crate::BitReader;
#[doc = "Field `PROG` writer - desc PROG"]
pub type ProgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PC"]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    pub fn pagelock(&self) -> PagelockR {
        PagelockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    pub fn prog(&self) -> ProgR {
        ProgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PC"]
    #[inline(always)]
    pub fn pc(&mut self) -> PcW<'_, IerSpec> {
        PcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    pub fn pagelock(&mut self) -> PagelockW<'_, IerSpec> {
        PagelockW::new(self, 1)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    pub fn prog(&mut self) -> ProgW<'_, IerSpec> {
        ProgW::new(self, 4)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
