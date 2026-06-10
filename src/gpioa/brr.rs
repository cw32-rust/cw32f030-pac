#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BRR0` reader - desc BRR0"]
pub type Brr0R = crate::BitReader;
#[doc = "Field `BRR0` writer - desc BRR0"]
pub type Brr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR1` reader - desc BRR1"]
pub type Brr1R = crate::BitReader;
#[doc = "Field `BRR1` writer - desc BRR1"]
pub type Brr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR2` reader - desc BRR2"]
pub type Brr2R = crate::BitReader;
#[doc = "Field `BRR2` writer - desc BRR2"]
pub type Brr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR3` reader - desc BRR3"]
pub type Brr3R = crate::BitReader;
#[doc = "Field `BRR3` writer - desc BRR3"]
pub type Brr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR4` reader - desc BRR4"]
pub type Brr4R = crate::BitReader;
#[doc = "Field `BRR4` writer - desc BRR4"]
pub type Brr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR5` reader - desc BRR5"]
pub type Brr5R = crate::BitReader;
#[doc = "Field `BRR5` writer - desc BRR5"]
pub type Brr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR6` reader - desc BRR6"]
pub type Brr6R = crate::BitReader;
#[doc = "Field `BRR6` writer - desc BRR6"]
pub type Brr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR7` reader - desc BRR7"]
pub type Brr7R = crate::BitReader;
#[doc = "Field `BRR7` writer - desc BRR7"]
pub type Brr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR8` reader - desc BRR8"]
pub type Brr8R = crate::BitReader;
#[doc = "Field `BRR8` writer - desc BRR8"]
pub type Brr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR9` reader - desc BRR9"]
pub type Brr9R = crate::BitReader;
#[doc = "Field `BRR9` writer - desc BRR9"]
pub type Brr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR10` reader - desc BRR10"]
pub type Brr10R = crate::BitReader;
#[doc = "Field `BRR10` writer - desc BRR10"]
pub type Brr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR11` reader - desc BRR11"]
pub type Brr11R = crate::BitReader;
#[doc = "Field `BRR11` writer - desc BRR11"]
pub type Brr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR12` reader - desc BRR12"]
pub type Brr12R = crate::BitReader;
#[doc = "Field `BRR12` writer - desc BRR12"]
pub type Brr12W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - desc BRR0"]
    #[inline(always)]
    pub fn brr0(&self) -> Brr0R {
        Brr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BRR1"]
    #[inline(always)]
    pub fn brr1(&self) -> Brr1R {
        Brr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc BRR2"]
    #[inline(always)]
    pub fn brr2(&self) -> Brr2R {
        Brr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&self) -> Brr3R {
        Brr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc BRR4"]
    #[inline(always)]
    pub fn brr4(&self) -> Brr4R {
        Brr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc BRR5"]
    #[inline(always)]
    pub fn brr5(&self) -> Brr5R {
        Brr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BRR6"]
    #[inline(always)]
    pub fn brr6(&self) -> Brr6R {
        Brr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BRR7"]
    #[inline(always)]
    pub fn brr7(&self) -> Brr7R {
        Brr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BRR8"]
    #[inline(always)]
    pub fn brr8(&self) -> Brr8R {
        Brr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc BRR9"]
    #[inline(always)]
    pub fn brr9(&self) -> Brr9R {
        Brr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc BRR10"]
    #[inline(always)]
    pub fn brr10(&self) -> Brr10R {
        Brr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc BRR11"]
    #[inline(always)]
    pub fn brr11(&self) -> Brr11R {
        Brr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BRR12"]
    #[inline(always)]
    pub fn brr12(&self) -> Brr12R {
        Brr12R::new(((self.bits >> 12) & 1) != 0)
    }
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
    #[doc = "Bit 0 - desc BRR0"]
    #[inline(always)]
    pub fn brr0(&mut self) -> Brr0W<'_, BrrSpec> {
        Brr0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc BRR1"]
    #[inline(always)]
    pub fn brr1(&mut self) -> Brr1W<'_, BrrSpec> {
        Brr1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc BRR2"]
    #[inline(always)]
    pub fn brr2(&mut self) -> Brr2W<'_, BrrSpec> {
        Brr2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&mut self) -> Brr3W<'_, BrrSpec> {
        Brr3W::new(self, 3)
    }
    #[doc = "Bit 4 - desc BRR4"]
    #[inline(always)]
    pub fn brr4(&mut self) -> Brr4W<'_, BrrSpec> {
        Brr4W::new(self, 4)
    }
    #[doc = "Bit 5 - desc BRR5"]
    #[inline(always)]
    pub fn brr5(&mut self) -> Brr5W<'_, BrrSpec> {
        Brr5W::new(self, 5)
    }
    #[doc = "Bit 6 - desc BRR6"]
    #[inline(always)]
    pub fn brr6(&mut self) -> Brr6W<'_, BrrSpec> {
        Brr6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc BRR7"]
    #[inline(always)]
    pub fn brr7(&mut self) -> Brr7W<'_, BrrSpec> {
        Brr7W::new(self, 7)
    }
    #[doc = "Bit 8 - desc BRR8"]
    #[inline(always)]
    pub fn brr8(&mut self) -> Brr8W<'_, BrrSpec> {
        Brr8W::new(self, 8)
    }
    #[doc = "Bit 9 - desc BRR9"]
    #[inline(always)]
    pub fn brr9(&mut self) -> Brr9W<'_, BrrSpec> {
        Brr9W::new(self, 9)
    }
    #[doc = "Bit 10 - desc BRR10"]
    #[inline(always)]
    pub fn brr10(&mut self) -> Brr10W<'_, BrrSpec> {
        Brr10W::new(self, 10)
    }
    #[doc = "Bit 11 - desc BRR11"]
    #[inline(always)]
    pub fn brr11(&mut self) -> Brr11W<'_, BrrSpec> {
        Brr11W::new(self, 11)
    }
    #[doc = "Bit 12 - desc BRR12"]
    #[inline(always)]
    pub fn brr12(&mut self) -> Brr12W<'_, BrrSpec> {
        Brr12W::new(self, 12)
    }
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
