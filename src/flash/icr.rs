#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
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
    pub fn pc(&mut self) -> PcW<'_, IcrSpec> {
        PcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PAGELOCK"]
    #[inline(always)]
    pub fn pagelock(&mut self) -> PagelockW<'_, IcrSpec> {
        PagelockW::new(self, 1)
    }
    #[doc = "Bit 4 - desc PROG"]
    #[inline(always)]
    pub fn prog(&mut self) -> ProgW<'_, IcrSpec> {
        ProgW::new(self, 4)
    }
}
#[doc = "Interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
