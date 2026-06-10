#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `AWTSRC` reader - desc AWTSRC"]
pub type AwtsrcR = crate::FieldReader;
#[doc = "Field `AWTSRC` writer - desc AWTSRC"]
pub type AwtsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPEDGE` reader - desc TAMPEDGE"]
pub type TampedgeR = crate::BitReader;
#[doc = "Field `TAMPEDGE` writer - desc TAMPEDGE"]
pub type TampedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCOUT` reader - desc RTCOUT"]
pub type RtcoutR = crate::FieldReader;
#[doc = "Field `RTCOUT` writer - desc RTCOUT"]
pub type RtcoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPEN` reader - desc TAMPEN"]
pub type TampenR = crate::BitReader;
#[doc = "Field `TAMPEN` writer - desc TAMPEN"]
pub type TampenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWTEN` reader - desc AWTEN"]
pub type AwtenR = crate::BitReader;
#[doc = "Field `AWTEN` writer - desc AWTEN"]
pub type AwtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARMAEN` reader - desc ALARMAEN"]
pub type AlarmaenR = crate::BitReader;
#[doc = "Field `ALARMAEN` writer - desc ALARMAEN"]
pub type AlarmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARMBEN` reader - desc ALARMBEN"]
pub type AlarmbenR = crate::BitReader;
#[doc = "Field `ALARMBEN` writer - desc ALARMBEN"]
pub type AlarmbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - desc AWTSRC"]
    #[inline(always)]
    pub fn awtsrc(&self) -> AwtsrcR {
        AwtsrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - desc TAMPEDGE"]
    #[inline(always)]
    pub fn tampedge(&self) -> TampedgeR {
        TampedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc RTCOUT"]
    #[inline(always)]
    pub fn rtcout(&self) -> RtcoutR {
        RtcoutR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc TAMPEN"]
    #[inline(always)]
    pub fn tampen(&self) -> TampenR {
        TampenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AWTEN"]
    #[inline(always)]
    pub fn awten(&self) -> AwtenR {
        AwtenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ALARMAEN"]
    #[inline(always)]
    pub fn alarmaen(&self) -> AlarmaenR {
        AlarmaenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ALARMBEN"]
    #[inline(always)]
    pub fn alarmben(&self) -> AlarmbenR {
        AlarmbenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc AWTSRC"]
    #[inline(always)]
    pub fn awtsrc(&mut self) -> AwtsrcW<'_, Cr2Spec> {
        AwtsrcW::new(self, 0)
    }
    #[doc = "Bit 3 - desc TAMPEDGE"]
    #[inline(always)]
    pub fn tampedge(&mut self) -> TampedgeW<'_, Cr2Spec> {
        TampedgeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - desc RTCOUT"]
    #[inline(always)]
    pub fn rtcout(&mut self) -> RtcoutW<'_, Cr2Spec> {
        RtcoutW::new(self, 4)
    }
    #[doc = "Bit 6 - desc TAMPEN"]
    #[inline(always)]
    pub fn tampen(&mut self) -> TampenW<'_, Cr2Spec> {
        TampenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc AWTEN"]
    #[inline(always)]
    pub fn awten(&mut self) -> AwtenW<'_, Cr2Spec> {
        AwtenW::new(self, 7)
    }
    #[doc = "Bit 9 - desc ALARMAEN"]
    #[inline(always)]
    pub fn alarmaen(&mut self) -> AlarmaenW<'_, Cr2Spec> {
        AlarmaenW::new(self, 9)
    }
    #[doc = "Bit 10 - desc ALARMBEN"]
    #[inline(always)]
    pub fn alarmben(&mut self) -> AlarmbenW<'_, Cr2Spec> {
        AlarmbenW::new(self, 10)
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
