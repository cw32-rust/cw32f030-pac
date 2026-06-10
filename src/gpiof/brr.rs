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
#[doc = "Field `BRR3` reader - desc BRR3"]
pub type Brr3R = crate::BitReader;
#[doc = "Field `BRR3` writer - desc BRR3"]
pub type Brr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR6` reader - desc BRR6"]
pub type Brr6R = crate::BitReader;
#[doc = "Field `BRR6` writer - desc BRR6"]
pub type Brr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRR7` reader - desc BRR7"]
pub type Brr7R = crate::BitReader;
#[doc = "Field `BRR7` writer - desc BRR7"]
pub type Brr7W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&self) -> Brr3R {
        Brr3R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 3 - desc BRR3"]
    #[inline(always)]
    pub fn brr3(&mut self) -> Brr3W<'_, BrrSpec> {
        Brr3W::new(self, 3)
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
