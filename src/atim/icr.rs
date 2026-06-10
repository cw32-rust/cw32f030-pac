#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `UIF` reader - desc UIF"]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - desc UIF"]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1AF` reader - desc C1AF"]
pub type C1afR = crate::BitReader;
#[doc = "Field `C1AF` writer - desc C1AF"]
pub type C1afW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2AF` reader - desc C2AF"]
pub type C2afR = crate::BitReader;
#[doc = "Field `C2AF` writer - desc C2AF"]
pub type C2afW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3AF` reader - desc C3AF"]
pub type C3afR = crate::BitReader;
#[doc = "Field `C3AF` writer - desc C3AF"]
pub type C3afW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1BF` reader - desc C1BF"]
pub type C1bfR = crate::BitReader;
#[doc = "Field `C1BF` writer - desc C1BF"]
pub type C1bfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2BF` reader - desc C2BF"]
pub type C2bfR = crate::BitReader;
#[doc = "Field `C2BF` writer - desc C2BF"]
pub type C2bfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3BF` reader - desc C3BF"]
pub type C3bfR = crate::BitReader;
#[doc = "Field `C3BF` writer - desc C3BF"]
pub type C3bfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1AE` reader - desc C1AE"]
pub type C1aeR = crate::BitReader;
#[doc = "Field `C1AE` writer - desc C1AE"]
pub type C1aeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2AE` reader - desc C2AE"]
pub type C2aeR = crate::BitReader;
#[doc = "Field `C2AE` writer - desc C2AE"]
pub type C2aeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3AE` reader - desc C3AE"]
pub type C3aeR = crate::BitReader;
#[doc = "Field `C3AE` writer - desc C3AE"]
pub type C3aeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1BE` reader - desc C1BE"]
pub type C1beR = crate::BitReader;
#[doc = "Field `C1BE` writer - desc C1BE"]
pub type C1beW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2BE` reader - desc C2BE"]
pub type C2beR = crate::BitReader;
#[doc = "Field `C2BE` writer - desc C2BE"]
pub type C2beW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3BE` reader - desc C3BE"]
pub type C3beR = crate::BitReader;
#[doc = "Field `C3BE` writer - desc C3BE"]
pub type C3beW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - desc BIF"]
pub type BifR = crate::BitReader;
#[doc = "Field `BIF` writer - desc BIF"]
pub type BifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - desc TIF"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - desc TIF"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - desc OVF"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - desc OVF"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDF` reader - desc UNDF"]
pub type UndfR = crate::BitReader;
#[doc = "Field `UNDF` writer - desc UNDF"]
pub type UndfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4AF` reader - desc C4AF"]
pub type C4afR = crate::BitReader;
#[doc = "Field `C4AF` writer - desc C4AF"]
pub type C4afW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc C1AF"]
    #[inline(always)]
    pub fn c1af(&self) -> C1afR {
        C1afR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc C2AF"]
    #[inline(always)]
    pub fn c2af(&self) -> C2afR {
        C2afR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc C3AF"]
    #[inline(always)]
    pub fn c3af(&self) -> C3afR {
        C3afR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc C1BF"]
    #[inline(always)]
    pub fn c1bf(&self) -> C1bfR {
        C1bfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc C2BF"]
    #[inline(always)]
    pub fn c2bf(&self) -> C2bfR {
        C2bfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc C3BF"]
    #[inline(always)]
    pub fn c3bf(&self) -> C3bfR {
        C3bfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc C1AE"]
    #[inline(always)]
    pub fn c1ae(&self) -> C1aeR {
        C1aeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc C2AE"]
    #[inline(always)]
    pub fn c2ae(&self) -> C2aeR {
        C2aeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc C3AE"]
    #[inline(always)]
    pub fn c3ae(&self) -> C3aeR {
        C3aeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc C1BE"]
    #[inline(always)]
    pub fn c1be(&self) -> C1beR {
        C1beR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc C2BE"]
    #[inline(always)]
    pub fn c2be(&self) -> C2beR {
        C2beR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc C3BE"]
    #[inline(always)]
    pub fn c3be(&self) -> C3beR {
        C3beR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BifR {
        BifR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc UNDF"]
    #[inline(always)]
    pub fn undf(&self) -> UndfR {
        UndfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc C4AF"]
    #[inline(always)]
    pub fn c4af(&self) -> C4afR {
        C4afR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UIF"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<'_, IcrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 2 - desc C1AF"]
    #[inline(always)]
    pub fn c1af(&mut self) -> C1afW<'_, IcrSpec> {
        C1afW::new(self, 2)
    }
    #[doc = "Bit 3 - desc C2AF"]
    #[inline(always)]
    pub fn c2af(&mut self) -> C2afW<'_, IcrSpec> {
        C2afW::new(self, 3)
    }
    #[doc = "Bit 4 - desc C3AF"]
    #[inline(always)]
    pub fn c3af(&mut self) -> C3afW<'_, IcrSpec> {
        C3afW::new(self, 4)
    }
    #[doc = "Bit 5 - desc C1BF"]
    #[inline(always)]
    pub fn c1bf(&mut self) -> C1bfW<'_, IcrSpec> {
        C1bfW::new(self, 5)
    }
    #[doc = "Bit 6 - desc C2BF"]
    #[inline(always)]
    pub fn c2bf(&mut self) -> C2bfW<'_, IcrSpec> {
        C2bfW::new(self, 6)
    }
    #[doc = "Bit 7 - desc C3BF"]
    #[inline(always)]
    pub fn c3bf(&mut self) -> C3bfW<'_, IcrSpec> {
        C3bfW::new(self, 7)
    }
    #[doc = "Bit 8 - desc C1AE"]
    #[inline(always)]
    pub fn c1ae(&mut self) -> C1aeW<'_, IcrSpec> {
        C1aeW::new(self, 8)
    }
    #[doc = "Bit 9 - desc C2AE"]
    #[inline(always)]
    pub fn c2ae(&mut self) -> C2aeW<'_, IcrSpec> {
        C2aeW::new(self, 9)
    }
    #[doc = "Bit 10 - desc C3AE"]
    #[inline(always)]
    pub fn c3ae(&mut self) -> C3aeW<'_, IcrSpec> {
        C3aeW::new(self, 10)
    }
    #[doc = "Bit 11 - desc C1BE"]
    #[inline(always)]
    pub fn c1be(&mut self) -> C1beW<'_, IcrSpec> {
        C1beW::new(self, 11)
    }
    #[doc = "Bit 12 - desc C2BE"]
    #[inline(always)]
    pub fn c2be(&mut self) -> C2beW<'_, IcrSpec> {
        C2beW::new(self, 12)
    }
    #[doc = "Bit 13 - desc C3BE"]
    #[inline(always)]
    pub fn c3be(&mut self) -> C3beW<'_, IcrSpec> {
        C3beW::new(self, 13)
    }
    #[doc = "Bit 14 - desc BIF"]
    #[inline(always)]
    pub fn bif(&mut self) -> BifW<'_, IcrSpec> {
        BifW::new(self, 14)
    }
    #[doc = "Bit 15 - desc TIF"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<'_, IcrSpec> {
        TifW::new(self, 15)
    }
    #[doc = "Bit 16 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<'_, IcrSpec> {
        OvfW::new(self, 16)
    }
    #[doc = "Bit 17 - desc UNDF"]
    #[inline(always)]
    pub fn undf(&mut self) -> UndfW<'_, IcrSpec> {
        UndfW::new(self, 17)
    }
    #[doc = "Bit 18 - desc C4AF"]
    #[inline(always)]
    pub fn c4af(&mut self) -> C4afW<'_, IcrSpec> {
        C4afW::new(self, 18)
    }
}
#[doc = "desc ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
