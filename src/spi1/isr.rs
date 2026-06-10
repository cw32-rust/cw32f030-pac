#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXE` reader - desc TXE"]
pub type TxeR = crate::BitReader;
#[doc = "Field `RXNE` reader - desc RXNE"]
pub type RxneR = crate::BitReader;
#[doc = "Field `SSF` reader - desc SSF"]
pub type SsfR = crate::BitReader;
#[doc = "Field `SSR` reader - desc SSR"]
pub type SsrR = crate::BitReader;
#[doc = "Field `UD` reader - desc UD"]
pub type UdR = crate::BitReader;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `SSERR` reader - desc SSERR"]
pub type SserrR = crate::BitReader;
#[doc = "Field `MODF` reader - desc MODF"]
pub type ModfR = crate::BitReader;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `SSLVL` reader - desc SSLVL"]
pub type SslvlR = crate::BitReader;
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
    #[doc = "Bit 8 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SSLVL"]
    #[inline(always)]
    pub fn sslvl(&self) -> SslvlR {
        SslvlR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
