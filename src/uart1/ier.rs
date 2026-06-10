#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXE` reader - TxBuf empty"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - TxBuf empty"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - TxBuf empty"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
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
    #[doc = "Bit 0 - TxBuf empty"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, IerSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit complete"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, IerSpec> {
        TcW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive complete"]
    #[inline(always)]
    pub fn rc(&mut self) -> RcW<'_, IerSpec> {
        RcW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame error"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, IerSpec> {
        FeW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, IerSpec> {
        PeW::new(self, 4)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    pub fn cts(&mut self) -> CtsW<'_, IerSpec> {
        CtsW::new(self, 6)
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
