#[doc = "Register `RCR` reader"]
pub type R = crate::R<RcrSpec>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RcrSpec>;
#[doc = "Field `RCR` reader - desc RCR"]
pub type RcrR = crate::FieldReader;
#[doc = "Field `RCR` writer - desc RCR"]
pub type RcrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OV` reader - desc OV"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - desc OV"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UD` reader - desc UD"]
pub type UdR = crate::BitReader;
#[doc = "Field `UD` writer - desc UD"]
pub type UdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - desc RCR"]
    #[inline(always)]
    pub fn rcr(&self) -> RcrR {
        RcrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - desc OV"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UD"]
    #[inline(always)]
    pub fn ud(&self) -> UdR {
        UdR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc RCR"]
    #[inline(always)]
    pub fn rcr(&mut self) -> RcrW<'_, RcrSpec> {
        RcrW::new(self, 0)
    }
    #[doc = "Bit 8 - desc OV"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<'_, RcrSpec> {
        OvW::new(self, 8)
    }
    #[doc = "Bit 9 - desc UD"]
    #[inline(always)]
    pub fn ud(&mut self) -> UdW<'_, RcrSpec> {
        UdW::new(self, 9)
    }
}
#[doc = "desc RCR\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcrSpec;
impl crate::RegisterSpec for RcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RcrSpec {}
