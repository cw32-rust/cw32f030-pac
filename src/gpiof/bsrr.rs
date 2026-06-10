#[doc = "Register `BSRR` reader"]
pub type R = crate::R<BsrrSpec>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BSS0` reader - desc BSS0"]
pub type Bss0R = crate::BitReader;
#[doc = "Field `BSS0` writer - desc BSS0"]
pub type Bss0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS1` reader - desc BSS1"]
pub type Bss1R = crate::BitReader;
#[doc = "Field `BSS1` writer - desc BSS1"]
pub type Bss1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS3` reader - desc BSS3"]
pub type Bss3R = crate::BitReader;
#[doc = "Field `BSS3` writer - desc BSS3"]
pub type Bss3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS6` reader - desc BSS6"]
pub type Bss6R = crate::BitReader;
#[doc = "Field `BSS6` writer - desc BSS6"]
pub type Bss6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSS7` reader - desc BSS7"]
pub type Bss7R = crate::BitReader;
#[doc = "Field `BSS7` writer - desc BSS7"]
pub type Bss7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc BSS0"]
    #[inline(always)]
    pub fn bss0(&self) -> Bss0R {
        Bss0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BSS1"]
    #[inline(always)]
    pub fn bss1(&self) -> Bss1R {
        Bss1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc BSS3"]
    #[inline(always)]
    pub fn bss3(&self) -> Bss3R {
        Bss3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc BSS6"]
    #[inline(always)]
    pub fn bss6(&self) -> Bss6R {
        Bss6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BSS7"]
    #[inline(always)]
    pub fn bss7(&self) -> Bss7R {
        Bss7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BSS0"]
    #[inline(always)]
    pub fn bss0(&mut self) -> Bss0W<'_, BsrrSpec> {
        Bss0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc BSS1"]
    #[inline(always)]
    pub fn bss1(&mut self) -> Bss1W<'_, BsrrSpec> {
        Bss1W::new(self, 1)
    }
    #[doc = "Bit 3 - desc BSS3"]
    #[inline(always)]
    pub fn bss3(&mut self) -> Bss3W<'_, BsrrSpec> {
        Bss3W::new(self, 3)
    }
    #[doc = "Bit 6 - desc BSS6"]
    #[inline(always)]
    pub fn bss6(&mut self) -> Bss6W<'_, BsrrSpec> {
        Bss6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc BSS7"]
    #[inline(always)]
    pub fn bss7(&mut self) -> Bss7W<'_, BsrrSpec> {
        Bss7W::new(self, 7)
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
