#[doc = "Register `BSRR` reader"]
pub type R = crate::R<BsrrSpec>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BSS13` reader - desc BSS13"]
pub type Bss13R = crate::BitReader;
#[doc = "Field `BSS13` writer - desc BSS13"]
pub type Bss13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS14` reader - desc BSS14"]
pub type Bss14R = crate::BitReader;
#[doc = "Field `BSS14` writer - desc BSS14"]
pub type Bss14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS15` reader - desc BSS15"]
pub type Bss15R = crate::BitReader;
#[doc = "Field `BSS15` writer - desc BSS15"]
pub type Bss15W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 13 - desc BSS13"]
    #[inline(always)]
    pub fn bss13(&self) -> Bss13R {
        Bss13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc BSS14"]
    #[inline(always)]
    pub fn bss14(&self) -> Bss14R {
        Bss14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc BSS15"]
    #[inline(always)]
    pub fn bss15(&self) -> Bss15R {
        Bss15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - desc BRR13"]
    #[inline(always)]
    pub fn brr13(&self) -> Brr13R {
        Brr13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc BRR14"]
    #[inline(always)]
    pub fn brr14(&self) -> Brr14R {
        Brr14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc BRR15"]
    #[inline(always)]
    pub fn brr15(&self) -> Brr15R {
        Brr15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc BSS13"]
    #[inline(always)]
    pub fn bss13(&mut self) -> Bss13W<'_, BsrrSpec> {
        Bss13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc BSS14"]
    #[inline(always)]
    pub fn bss14(&mut self) -> Bss14W<'_, BsrrSpec> {
        Bss14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc BSS15"]
    #[inline(always)]
    pub fn bss15(&mut self) -> Bss15W<'_, BsrrSpec> {
        Bss15W::new(self, 15)
    }
    #[doc = "Bit 29 - desc BRR13"]
    #[inline(always)]
    pub fn brr13(&mut self) -> Brr13W<'_, BsrrSpec> {
        Brr13W::new(self, 29)
    }
    #[doc = "Bit 30 - desc BRR14"]
    #[inline(always)]
    pub fn brr14(&mut self) -> Brr14W<'_, BsrrSpec> {
        Brr14W::new(self, 30)
    }
    #[doc = "Bit 31 - desc BRR15"]
    #[inline(always)]
    pub fn brr15(&mut self) -> Brr15W<'_, BsrrSpec> {
        Brr15W::new(self, 31)
    }
}
#[doc = "desc BSRR\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr::R`](R) reader structure"]
impl crate::Readable for BsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}
