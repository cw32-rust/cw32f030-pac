#[doc = "Register `CH4CR` reader"]
pub type R = crate::R<Ch4crSpec>;
#[doc = "Register `CH4CR` writer"]
pub type W = crate::W<Ch4crSpec>;
#[doc = "Field `BUFE` reader - desc BUFE"]
pub type BufeR = crate::BitReader;
#[doc = "Field `BUFE` writer - desc BUFE"]
pub type BufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIE` reader - desc CIE"]
pub type CieR = crate::BitReader;
#[doc = "Field `CIE` writer - desc CIE"]
pub type CieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDE` reader - desc CDE"]
pub type CdeR = crate::BitReader;
#[doc = "Field `CDE` writer - desc CDE"]
pub type CdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIS` reader - desc CIS"]
pub type CisR = crate::FieldReader;
#[doc = "Field `CIS` writer - desc CIS"]
pub type CisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C4EN` reader - desc C4EN"]
pub type C4enR = crate::BitReader;
#[doc = "Field `C4EN` writer - desc C4EN"]
pub type C4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BUFE"]
    #[inline(always)]
    pub fn bufe(&self) -> BufeR {
        BufeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CIE"]
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CDE"]
    #[inline(always)]
    pub fn cde(&self) -> CdeR {
        CdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - desc CIS"]
    #[inline(always)]
    pub fn cis(&self) -> CisR {
        CisR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - desc C4EN"]
    #[inline(always)]
    pub fn c4en(&self) -> C4enR {
        C4enR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BUFE"]
    #[inline(always)]
    pub fn bufe(&mut self) -> BufeW<'_, Ch4crSpec> {
        BufeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CIE"]
    #[inline(always)]
    pub fn cie(&mut self) -> CieW<'_, Ch4crSpec> {
        CieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CDE"]
    #[inline(always)]
    pub fn cde(&mut self) -> CdeW<'_, Ch4crSpec> {
        CdeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - desc CIS"]
    #[inline(always)]
    pub fn cis(&mut self) -> CisW<'_, Ch4crSpec> {
        CisW::new(self, 3)
    }
    #[doc = "Bit 5 - desc C4EN"]
    #[inline(always)]
    pub fn c4en(&mut self) -> C4enW<'_, Ch4crSpec> {
        C4enW::new(self, 5)
    }
}
#[doc = "desc CH4CR\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4crSpec;
impl crate::RegisterSpec for Ch4crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cr::R`](R) reader structure"]
impl crate::Readable for Ch4crSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4cr::W`](W) writer structure"]
impl crate::Writable for Ch4crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4CR to value 0"]
impl crate::Resettable for Ch4crSpec {}
