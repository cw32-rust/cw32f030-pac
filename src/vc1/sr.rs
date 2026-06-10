#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `INTF` reader - desc INTF"]
pub type IntfR = crate::BitReader;
#[doc = "Field `INTF` writer - desc INTF"]
pub type IntfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTV` reader - desc FLTV"]
pub type FltvR = crate::BitReader;
#[doc = "Field `READY` reader - desc READY"]
pub type ReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc INTF"]
    #[inline(always)]
    pub fn intf(&self) -> IntfR {
        IntfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc FLTV"]
    #[inline(always)]
    pub fn fltv(&self) -> FltvR {
        FltvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc INTF"]
    #[inline(always)]
    pub fn intf(&mut self) -> IntfW<'_, SrSpec> {
        IntfW::new(self, 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
