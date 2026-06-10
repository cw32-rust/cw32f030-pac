#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `ADDREN` reader - desc ADDREN"]
pub type AddrenR = crate::BitReader;
#[doc = "Field `ADDREN` writer - desc ADDREN"]
pub type AddrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGNAL` reader - desc SIGNAL"]
pub type SignalR = crate::BitReader;
#[doc = "Field `SIGNAL` writer - desc SIGNAL"]
pub type SignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - desc CTSEN"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - desc CTSEN"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - desc RTSEN"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - desc RTSEN"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINV` reader - desc RXINV"]
pub type RxinvR = crate::BitReader;
#[doc = "Field `RXINV` writer - desc RXINV"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXINV` reader - desc TXINV"]
pub type TxinvR = crate::BitReader;
#[doc = "Field `TXINV` writer - desc TXINV"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARX` reader - desc DMARX"]
pub type DmarxR = crate::BitReader;
#[doc = "Field `DMARX` writer - desc DMARX"]
pub type DmarxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATX` reader - desc DMATX"]
pub type DmatxR = crate::BitReader;
#[doc = "Field `DMATX` writer - desc DMATX"]
pub type DmatxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOURCE` reader - desc SOURCE"]
pub type SourceR = crate::FieldReader;
#[doc = "Field `SOURCE` writer - desc SOURCE"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc ADDREN"]
    #[inline(always)]
    pub fn addren(&self) -> AddrenR {
        AddrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SIGNAL"]
    #[inline(always)]
    pub fn signal(&self) -> SignalR {
        SignalR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXINV"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TXINV"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&self) -> DmarxR {
        DmarxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&self) -> DmatxR {
        DmatxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ADDREN"]
    #[inline(always)]
    pub fn addren(&mut self) -> AddrenW<'_, Cr2Spec> {
        AddrenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SIGNAL"]
    #[inline(always)]
    pub fn signal(&mut self) -> SignalW<'_, Cr2Spec> {
        SignalW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CTSEN"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, Cr2Spec> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc RTSEN"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, Cr2Spec> {
        RtsenW::new(self, 3)
    }
    #[doc = "Bit 4 - desc RXINV"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, Cr2Spec> {
        RxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TXINV"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, Cr2Spec> {
        TxinvW::new(self, 5)
    }
    #[doc = "Bit 6 - desc DMARX"]
    #[inline(always)]
    pub fn dmarx(&mut self) -> DmarxW<'_, Cr2Spec> {
        DmarxW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DMATX"]
    #[inline(always)]
    pub fn dmatx(&mut self) -> DmatxW<'_, Cr2Spec> {
        DmatxW::new(self, 7)
    }
    #[doc = "Bits 8:9 - desc SOURCE"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, Cr2Spec> {
        SourceW::new(self, 8)
    }
}
#[doc = "Control register2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
