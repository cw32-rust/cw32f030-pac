#[doc = "Register `MSCR` reader"]
pub type R = crate::R<MscrSpec>;
#[doc = "Register `MSCR` writer"]
pub type W = crate::W<MscrSpec>;
#[doc = "Field `MMS` reader - desc MMS"]
pub type MmsR = crate::FieldReader;
#[doc = "Field `MMS` writer - desc MMS"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCDS` reader - desc CCDS"]
pub type CcdsR = crate::BitReader;
#[doc = "Field `CCDS` writer - desc CCDS"]
pub type CcdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS` reader - desc TS"]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - desc TS"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMS` reader - desc SMS"]
pub type SmsR = crate::FieldReader;
#[doc = "Field `SMS` writer - desc SMS"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IA1S` reader - desc IA1S"]
pub type Ia1sR = crate::BitReader;
#[doc = "Field `IA1S` writer - desc IA1S"]
pub type Ia1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IB1S` reader - desc IB1S"]
pub type Ib1sR = crate::BitReader;
#[doc = "Field `IB1S` writer - desc IB1S"]
pub type Ib1sW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc MMS"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&self) -> CcdsR {
        CcdsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:7 - desc TS"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - desc SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - desc IA1S"]
    #[inline(always)]
    pub fn ia1s(&self) -> Ia1sR {
        Ia1sR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc IB1S"]
    #[inline(always)]
    pub fn ib1s(&self) -> Ib1sR {
        Ib1sR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc MMS"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<'_, MscrSpec> {
        MmsW::new(self, 0)
    }
    #[doc = "Bit 3 - desc CCDS"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CcdsW<'_, MscrSpec> {
        CcdsW::new(self, 3)
    }
    #[doc = "Bits 5:7 - desc TS"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, MscrSpec> {
        TsW::new(self, 5)
    }
    #[doc = "Bits 8:10 - desc SMS"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<'_, MscrSpec> {
        SmsW::new(self, 8)
    }
    #[doc = "Bit 11 - desc IA1S"]
    #[inline(always)]
    pub fn ia1s(&mut self) -> Ia1sW<'_, MscrSpec> {
        Ia1sW::new(self, 11)
    }
    #[doc = "Bit 12 - desc IB1S"]
    #[inline(always)]
    pub fn ib1s(&mut self) -> Ib1sW<'_, MscrSpec> {
        Ib1sW::new(self, 12)
    }
}
#[doc = "desc MSCR\n\nYou can [`read`](crate::Reg::read) this register and get [`mscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MscrSpec;
impl crate::RegisterSpec for MscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mscr::R`](R) reader structure"]
impl crate::Readable for MscrSpec {}
#[doc = "`write(|w| ..)` method takes [`mscr::W`](W) writer structure"]
impl crate::Writable for MscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSCR to value 0"]
impl crate::Resettable for MscrSpec {}
