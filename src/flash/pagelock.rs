#[doc = "Register `PAGELOCK` reader"]
pub type R = crate::R<PagelockSpec>;
#[doc = "Register `PAGELOCK` writer"]
pub type W = crate::W<PagelockSpec>;
#[doc = "Field `LOCK0` reader - Page0 - 7"]
pub type Lock0R = crate::BitReader;
#[doc = "Field `LOCK0` writer - Page0 - 7"]
pub type Lock0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK1` reader - Page8 - 15"]
pub type Lock1R = crate::BitReader;
#[doc = "Field `LOCK1` writer - Page8 - 15"]
pub type Lock1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK2` reader - Page16 - 23"]
pub type Lock2R = crate::BitReader;
#[doc = "Field `LOCK2` writer - Page16 - 23"]
pub type Lock2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK3` reader - Page24 - 31"]
pub type Lock3R = crate::BitReader;
#[doc = "Field `LOCK3` writer - Page24 - 31"]
pub type Lock3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK4` reader - Page32 - 39"]
pub type Lock4R = crate::BitReader;
#[doc = "Field `LOCK4` writer - Page32 - 39"]
pub type Lock4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK5` reader - Page40 - 47"]
pub type Lock5R = crate::BitReader;
#[doc = "Field `LOCK5` writer - Page40 - 47"]
pub type Lock5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK6` reader - Page48 - 55"]
pub type Lock6R = crate::BitReader;
#[doc = "Field `LOCK6` writer - Page48 - 55"]
pub type Lock6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK7` reader - Page56 - 63"]
pub type Lock7R = crate::BitReader;
#[doc = "Field `LOCK7` writer - Page56 - 63"]
pub type Lock7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK8` reader - Page64 - 71"]
pub type Lock8R = crate::BitReader;
#[doc = "Field `LOCK8` writer - Page64 - 71"]
pub type Lock8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK9` reader - Page72 - 79"]
pub type Lock9R = crate::BitReader;
#[doc = "Field `LOCK9` writer - Page72 - 79"]
pub type Lock9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK10` reader - Page80 - 87"]
pub type Lock10R = crate::BitReader;
#[doc = "Field `LOCK10` writer - Page80 - 87"]
pub type Lock10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK11` reader - Page88 - 95"]
pub type Lock11R = crate::BitReader;
#[doc = "Field `LOCK11` writer - Page88 - 95"]
pub type Lock11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK12` reader - Page96 - 103"]
pub type Lock12R = crate::BitReader;
#[doc = "Field `LOCK12` writer - Page96 - 103"]
pub type Lock12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK13` reader - Page104 - 111"]
pub type Lock13R = crate::BitReader;
#[doc = "Field `LOCK13` writer - Page104 - 111"]
pub type Lock13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK14` reader - Page112 - 119"]
pub type Lock14R = crate::BitReader;
#[doc = "Field `LOCK14` writer - Page112 - 119"]
pub type Lock14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK15` reader - Page120 - 127"]
pub type Lock15R = crate::BitReader;
#[doc = "Field `LOCK15` writer - Page120 - 127"]
pub type Lock15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - desc KEY"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Page0 - 7"]
    #[inline(always)]
    pub fn lock0(&self) -> Lock0R {
        Lock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page8 - 15"]
    #[inline(always)]
    pub fn lock1(&self) -> Lock1R {
        Lock1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page16 - 23"]
    #[inline(always)]
    pub fn lock2(&self) -> Lock2R {
        Lock2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Page24 - 31"]
    #[inline(always)]
    pub fn lock3(&self) -> Lock3R {
        Lock3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Page32 - 39"]
    #[inline(always)]
    pub fn lock4(&self) -> Lock4R {
        Lock4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Page40 - 47"]
    #[inline(always)]
    pub fn lock5(&self) -> Lock5R {
        Lock5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Page48 - 55"]
    #[inline(always)]
    pub fn lock6(&self) -> Lock6R {
        Lock6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Page56 - 63"]
    #[inline(always)]
    pub fn lock7(&self) -> Lock7R {
        Lock7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Page64 - 71"]
    #[inline(always)]
    pub fn lock8(&self) -> Lock8R {
        Lock8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Page72 - 79"]
    #[inline(always)]
    pub fn lock9(&self) -> Lock9R {
        Lock9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Page80 - 87"]
    #[inline(always)]
    pub fn lock10(&self) -> Lock10R {
        Lock10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Page88 - 95"]
    #[inline(always)]
    pub fn lock11(&self) -> Lock11R {
        Lock11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Page96 - 103"]
    #[inline(always)]
    pub fn lock12(&self) -> Lock12R {
        Lock12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Page104 - 111"]
    #[inline(always)]
    pub fn lock13(&self) -> Lock13R {
        Lock13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Page112 - 119"]
    #[inline(always)]
    pub fn lock14(&self) -> Lock14R {
        Lock14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Page120 - 127"]
    #[inline(always)]
    pub fn lock15(&self) -> Lock15R {
        Lock15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Page0 - 7"]
    #[inline(always)]
    pub fn lock0(&mut self) -> Lock0W<'_, PagelockSpec> {
        Lock0W::new(self, 0)
    }
    #[doc = "Bit 1 - Page8 - 15"]
    #[inline(always)]
    pub fn lock1(&mut self) -> Lock1W<'_, PagelockSpec> {
        Lock1W::new(self, 1)
    }
    #[doc = "Bit 2 - Page16 - 23"]
    #[inline(always)]
    pub fn lock2(&mut self) -> Lock2W<'_, PagelockSpec> {
        Lock2W::new(self, 2)
    }
    #[doc = "Bit 3 - Page24 - 31"]
    #[inline(always)]
    pub fn lock3(&mut self) -> Lock3W<'_, PagelockSpec> {
        Lock3W::new(self, 3)
    }
    #[doc = "Bit 4 - Page32 - 39"]
    #[inline(always)]
    pub fn lock4(&mut self) -> Lock4W<'_, PagelockSpec> {
        Lock4W::new(self, 4)
    }
    #[doc = "Bit 5 - Page40 - 47"]
    #[inline(always)]
    pub fn lock5(&mut self) -> Lock5W<'_, PagelockSpec> {
        Lock5W::new(self, 5)
    }
    #[doc = "Bit 6 - Page48 - 55"]
    #[inline(always)]
    pub fn lock6(&mut self) -> Lock6W<'_, PagelockSpec> {
        Lock6W::new(self, 6)
    }
    #[doc = "Bit 7 - Page56 - 63"]
    #[inline(always)]
    pub fn lock7(&mut self) -> Lock7W<'_, PagelockSpec> {
        Lock7W::new(self, 7)
    }
    #[doc = "Bit 8 - Page64 - 71"]
    #[inline(always)]
    pub fn lock8(&mut self) -> Lock8W<'_, PagelockSpec> {
        Lock8W::new(self, 8)
    }
    #[doc = "Bit 9 - Page72 - 79"]
    #[inline(always)]
    pub fn lock9(&mut self) -> Lock9W<'_, PagelockSpec> {
        Lock9W::new(self, 9)
    }
    #[doc = "Bit 10 - Page80 - 87"]
    #[inline(always)]
    pub fn lock10(&mut self) -> Lock10W<'_, PagelockSpec> {
        Lock10W::new(self, 10)
    }
    #[doc = "Bit 11 - Page88 - 95"]
    #[inline(always)]
    pub fn lock11(&mut self) -> Lock11W<'_, PagelockSpec> {
        Lock11W::new(self, 11)
    }
    #[doc = "Bit 12 - Page96 - 103"]
    #[inline(always)]
    pub fn lock12(&mut self) -> Lock12W<'_, PagelockSpec> {
        Lock12W::new(self, 12)
    }
    #[doc = "Bit 13 - Page104 - 111"]
    #[inline(always)]
    pub fn lock13(&mut self) -> Lock13W<'_, PagelockSpec> {
        Lock13W::new(self, 13)
    }
    #[doc = "Bit 14 - Page112 - 119"]
    #[inline(always)]
    pub fn lock14(&mut self) -> Lock14W<'_, PagelockSpec> {
        Lock14W::new(self, 14)
    }
    #[doc = "Bit 15 - Page120 - 127"]
    #[inline(always)]
    pub fn lock15(&mut self) -> Lock15W<'_, PagelockSpec> {
        Lock15W::new(self, 15)
    }
    #[doc = "Bits 16:31 - desc KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, PagelockSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Page Write Erase Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`pagelock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagelock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PagelockSpec;
impl crate::RegisterSpec for PagelockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pagelock::R`](R) reader structure"]
impl crate::Readable for PagelockSpec {}
#[doc = "`write(|w| ..)` method takes [`pagelock::W`](W) writer structure"]
impl crate::Writable for PagelockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAGELOCK to value 0"]
impl crate::Resettable for PagelockSpec {}
