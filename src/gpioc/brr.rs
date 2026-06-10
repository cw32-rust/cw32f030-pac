#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BRR13` reader - desc BRR13"]
pub type Brr13R = crate::BitReader;
#[doc = "Field `BRR13` writer - desc BRR13"]
pub type Brr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR14` reader - desc BRR14"]
pub type Brr14R = crate::BitReader;
#[doc = "Field `BRR14` writer - desc BRR14"]
pub type Brr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR15` reader - desc BRR15"]
pub type Brr15R = crate::BitReader;
#[doc = "Field `BRR15` writer - desc BRR15"]
pub type Brr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc BRR13"]
    #[inline(always)]
    pub fn brr13(&self) -> Brr13R {
        Brr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BRR14"]
    #[inline(always)]
    pub fn brr14(&self) -> Brr14R {
        Brr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BRR15"]
    #[inline(always)]
    pub fn brr15(&self) -> Brr15R {
        Brr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc BRR13"]
    #[inline(always)]
    pub fn brr13(&mut self) -> Brr13W<'_, BrrSpec> {
        Brr13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc BRR14"]
    #[inline(always)]
    pub fn brr14(&mut self) -> Brr14W<'_, BrrSpec> {
        Brr14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc BRR15"]
    #[inline(always)]
    pub fn brr15(&mut self) -> Brr15W<'_, BrrSpec> {
        Brr15W::new(self, 15)
    }
}
#[doc = "desc BRR\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
