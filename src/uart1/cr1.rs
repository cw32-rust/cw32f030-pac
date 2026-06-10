#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `TXEN` reader - desc TXEN"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - desc TXEN"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - desc RXEN"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - desc RXEN"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - desc PARITY"]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - desc PARITY"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC` reader - desc SYNC"]
pub type SyncR = crate::BitReader;
#[doc = "Field `SYNC` writer - desc SYNC"]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER` reader - desc OVER"]
pub type OverR = crate::FieldReader;
#[doc = "Field `OVER` writer - desc OVER"]
pub type OverW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc SYNC"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - desc OVER"]
    #[inline(always)]
    pub fn over(&self) -> OverR {
        OverR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc TXEN"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, Cr1Spec> {
        TxenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RXEN"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, Cr1Spec> {
        RxenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - desc PARITY"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, Cr1Spec> {
        ParityW::new(self, 2)
    }
    #[doc = "Bits 4:5 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr1Spec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 6 - desc SYNC"]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, Cr1Spec> {
        SyncW::new(self, 6)
    }
    #[doc = "Bit 8 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Cr1Spec> {
        StartW::new(self, 8)
    }
    #[doc = "Bits 9:10 - desc OVER"]
    #[inline(always)]
    pub fn over(&mut self) -> OverW<'_, Cr1Spec> {
        OverW::new(self, 9)
    }
}
#[doc = "Control register1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
