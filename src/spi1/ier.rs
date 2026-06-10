#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXE` writer - desc TXE"]
pub type TxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - desc RXNE"]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSF` reader - desc SSF"]
pub type SsfR = crate::BitReader;
#[doc = "Field `SSF` writer - desc SSF"]
pub type SsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSR` reader - desc SSR"]
pub type SsrR = crate::BitReader;
#[doc = "Field `SSR` writer - desc SSR"]
pub type SsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UD` reader - desc UD"]
pub type UdR = crate::BitReader;
#[doc = "Field `UD` writer - desc UD"]
pub type UdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSERR` reader - desc SSERR"]
pub type SserrR = crate::BitReader;
#[doc = "Field `SSERR` writer - desc SSERR"]
pub type SserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` reader - desc MODF"]
pub type ModfR = crate::BitReader;
#[doc = "Field `MODF` writer - desc MODF"]
pub type ModfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SSF"]
    #[inline(always)]
    pub fn ssf(&self) -> SsfR {
        SsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SSR"]
    #[inline(always)]
    pub fn ssr(&self) -> SsrR {
        SsrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc SSERR"]
    #[inline(always)]
    pub fn sserr(&self) -> SserrR {
        SserrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MODF"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXE"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<'_, IerSpec> {
        TxeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXNE"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<'_, IerSpec> {
        RxneW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SSF"]
    #[inline(always)]
    pub fn ssf(&mut self) -> SsfW<'_, IerSpec> {
        SsfW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SSR"]
    #[inline(always)]
    pub fn ssr(&mut self) -> SsrW<'_, IerSpec> {
        SsrW::new(self, 3)
    }
    #[doc = "Bit 4 - desc UD"]
    #[inline(always)]
    pub fn ud(&mut self) -> UdW<'_, IerSpec> {
        UdW::new(self, 4)
    }
    #[doc = "Bit 5 - desc OV"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<'_, IerSpec> {
        OvW::new(self, 5)
    }
    #[doc = "Bit 6 - desc SSERR"]
    #[inline(always)]
    pub fn sserr(&mut self) -> SserrW<'_, IerSpec> {
        SserrW::new(self, 6)
    }
    #[doc = "Bit 7 - desc MODF"]
    #[inline(always)]
    pub fn modf(&mut self) -> ModfW<'_, IerSpec> {
        ModfW::new(self, 7)
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
