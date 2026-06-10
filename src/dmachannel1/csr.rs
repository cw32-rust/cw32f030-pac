#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - desc TCIE"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - desc TCIE"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - desc TEIE"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - desc TEIE"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS` reader - desc TRANS"]
pub type TransR = crate::BitReader;
#[doc = "Field `TRANS` writer - desc TRANS"]
pub type TransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCINC` reader - desc SRCINC"]
pub type SrcincR = crate::BitReader;
#[doc = "Field `SRCINC` writer - desc SRCINC"]
pub type SrcincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSTINC` reader - desc DSTINC"]
pub type DstincR = crate::BitReader;
#[doc = "Field `DSTINC` writer - desc DSTINC"]
pub type DstincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - desc SIZE"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - desc SIZE"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STATUS` reader - desc STATUS"]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TRANS"]
    #[inline(always)]
    pub fn trans(&self) -> TransR {
        TransR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SRCINC"]
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc DSTINC"]
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - desc STATUS"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, CsrSpec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc TEIE"]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<'_, CsrSpec> {
        TeieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TRANS"]
    #[inline(always)]
    pub fn trans(&mut self) -> TransW<'_, CsrSpec> {
        TransW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SRCINC"]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SrcincW<'_, CsrSpec> {
        SrcincW::new(self, 4)
    }
    #[doc = "Bit 5 - desc DSTINC"]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DstincW<'_, CsrSpec> {
        DstincW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc SIZE"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, CsrSpec> {
        SizeW::new(self, 6)
    }
}
#[doc = "Channel.y control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
