#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TC` reader - Transmit complete"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - Transmit complete"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - Receive complete"]
pub type RcR = crate::BitReader;
#[doc = "Field `RC` writer - Receive complete"]
pub type RcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - Frame error"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - Frame error"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity error"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - CTS change"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - CTS change"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Transmit complete"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive complete"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit complete"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, IcrSpec> {
        TcW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive complete"]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<'_, IcrSpec> {
        RcW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame error"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, IcrSpec> {
        FeW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, IcrSpec> {
        PeW::new(self, 4)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, IcrSpec> {
        CtsW::new(self, 6)
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
