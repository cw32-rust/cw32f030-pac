#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `HSIRDY` reader - desc HSIRDY"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSERDY` reader - desc HSERDY"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `PLLRDY` reader - desc PLLRDY"]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `LSIRDY` reader - desc LSIRDY"]
pub type LsirdyR = crate::BitReader;
#[doc = "Field `LSERDY` reader - desc LSERDY"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSEFAIL` reader - desc LSEFAIL"]
pub type LsefailR = crate::BitReader;
#[doc = "Field `HSEFAIL` reader - desc HSEFAIL"]
pub type HsefailR = crate::BitReader;
#[doc = "Field `LSEFAULT` reader - desc LSEFAULT"]
pub type LsefaultR = crate::BitReader;
#[doc = "Field `HSEFAULT` reader - desc HSEFAULT"]
pub type HsefaultR = crate::BitReader;
#[doc = "Field `HSISTABLE` reader - desc HSISTABLE"]
pub type HsistableR = crate::BitReader;
#[doc = "Field `HSESTABLE` reader - desc HSESTABLE"]
pub type HsestableR = crate::BitReader;
#[doc = "Field `PLLSTABLE` reader - desc PLLSTABLE"]
pub type PllstableR = crate::BitReader;
#[doc = "Field `LSISTABLE` reader - desc LSISTABLE"]
pub type LsistableR = crate::BitReader;
#[doc = "Field `LSESTABLE` reader - desc LSESTABLE"]
pub type LsestableR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PLLRDY"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LsirdyR {
        LsirdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LSEFAIL"]
    #[inline(always)]
    pub fn lsefail(&self) -> LsefailR {
        LsefailR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc HSEFAIL"]
    #[inline(always)]
    pub fn hsefail(&self) -> HsefailR {
        HsefailR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc LSEFAULT"]
    #[inline(always)]
    pub fn lsefault(&self) -> LsefaultR {
        LsefaultR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc HSEFAULT"]
    #[inline(always)]
    pub fn hsefault(&self) -> HsefaultR {
        HsefaultR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - desc HSISTABLE"]
    #[inline(always)]
    pub fn hsistable(&self) -> HsistableR {
        HsistableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc HSESTABLE"]
    #[inline(always)]
    pub fn hsestable(&self) -> HsestableR {
        HsestableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PLLSTABLE"]
    #[inline(always)]
    pub fn pllstable(&self) -> PllstableR {
        PllstableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc LSISTABLE"]
    #[inline(always)]
    pub fn lsistable(&self) -> LsistableR {
        LsistableR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc LSESTABLE"]
    #[inline(always)]
    pub fn lsestable(&self) -> LsestableR {
        LsestableR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interupt Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
