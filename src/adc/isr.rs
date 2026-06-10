#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOA` reader - desc EOA"]
pub type EoaR = crate::BitReader;
#[doc = "Field `WDTL` reader - desc WDTL"]
pub type WdtlR = crate::BitReader;
#[doc = "Field `WDTH` reader - desc WDTH"]
pub type WdthR = crate::BitReader;
#[doc = "Field `WDTR` reader - desc WDTR"]
pub type WdtrR = crate::BitReader;
#[doc = "Field `OVW` reader - desc OVW"]
pub type OvwR = crate::BitReader;
#[doc = "Field `READY` reader - desc READY"]
pub type ReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc EOA"]
    #[inline(always)]
    pub fn eoa(&self) -> EoaR {
        EoaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc WDTL"]
    #[inline(always)]
    pub fn wdtl(&self) -> WdtlR {
        WdtlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc WDTH"]
    #[inline(always)]
    pub fn wdth(&self) -> WdthR {
        WdthR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WDTR"]
    #[inline(always)]
    pub fn wdtr(&self) -> WdtrR {
        WdtrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVW"]
    #[inline(always)]
    pub fn ovw(&self) -> OvwR {
        OvwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 7) & 1) != 0)
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
