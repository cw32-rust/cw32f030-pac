#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TXE` reader - TxBuf empty"]
pub type TxeR = crate::BitReader;
#[doc = "Field `TC` reader - Transmit complete"]
pub type TcR = crate::BitReader;
#[doc = "Field `RC` reader - Receive complete"]
pub type RcR = crate::BitReader;
#[doc = "Field `FE` reader - Frame error"]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `MATCH` reader - Slave addr match"]
pub type MatchR = crate::BitReader;
#[doc = "Field `CTS` reader - CTS change"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTSLV` reader - CTS PIN level"]
pub type CtslvR = crate::BitReader;
#[doc = "Field `TXBUSY` reader - desc TXBUSY"]
pub type TxbusyR = crate::BitReader;
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
    #[doc = "Bit 5 - Slave addr match"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTS change"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS PIN level"]
    #[inline(always)]
    pub fn ctslv(&self) -> CtslvR {
        CtslvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc TXBUSY"]
    #[inline(always)]
    pub fn txbusy(&self) -> TxbusyR {
        TxbusyR::new(((self.bits >> 8) & 1) != 0)
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
