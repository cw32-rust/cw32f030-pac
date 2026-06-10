#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `EOC` reader - desc EOC"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - desc EOC"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - desc EOS"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - desc EOS"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOA` reader - desc EOA"]
pub type EoaR = crate::BitReader;
#[doc = "Field `EOA` writer - desc EOA"]
pub type EoaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTL` reader - desc WDTL"]
pub type WdtlR = crate::BitReader;
#[doc = "Field `WDTL` writer - desc WDTL"]
pub type WdtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTH` reader - desc WDTH"]
pub type WdthR = crate::BitReader;
#[doc = "Field `WDTH` writer - desc WDTH"]
pub type WdthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTR` reader - desc WDTR"]
pub type WdtrR = crate::BitReader;
#[doc = "Field `WDTR` writer - desc WDTR"]
pub type WdtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVW` reader - desc OVW"]
pub type OvwR = crate::BitReader;
#[doc = "Field `OVW` writer - desc OVW"]
pub type OvwW<'a, REG> = crate::BitWriter<'a, REG>;
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
}
impl W {
    #[doc = "Bit 0 - desc EOC"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<'_, IcrSpec> {
        EocW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EOS"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<'_, IcrSpec> {
        EosW::new(self, 1)
    }
    #[doc = "Bit 2 - desc EOA"]
    #[inline(always)]
    pub fn eoa(&mut self) -> EoaW<'_, IcrSpec> {
        EoaW::new(self, 2)
    }
    #[doc = "Bit 3 - desc WDTL"]
    #[inline(always)]
    pub fn wdtl(&mut self) -> WdtlW<'_, IcrSpec> {
        WdtlW::new(self, 3)
    }
    #[doc = "Bit 4 - desc WDTH"]
    #[inline(always)]
    pub fn wdth(&mut self) -> WdthW<'_, IcrSpec> {
        WdthW::new(self, 4)
    }
    #[doc = "Bit 5 - desc WDTR"]
    #[inline(always)]
    pub fn wdtr(&mut self) -> WdtrW<'_, IcrSpec> {
        WdtrW::new(self, 5)
    }
    #[doc = "Bit 6 - desc OVW"]
    #[inline(always)]
    pub fn ovw(&mut self) -> OvwW<'_, IcrSpec> {
        OvwW::new(self, 6)
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
