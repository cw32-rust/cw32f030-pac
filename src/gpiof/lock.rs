#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Field `PIN0` reader - desc PIN0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN0` writer - desc PIN0"]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN1` reader - desc PIN1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN1` writer - desc PIN1"]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN3` reader - desc PIN3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN3` writer - desc PIN3"]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN6` reader - desc PIN6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN6` writer - desc PIN6"]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN7` reader - desc PIN7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN7` writer - desc PIN7"]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` writer - KEY = 0x5A5A"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PIN0"]
    #[inline(always)]
    pub fn pin0(&mut self) -> Pin0W<'_, LockSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - desc PIN1"]
    #[inline(always)]
    pub fn pin1(&mut self) -> Pin1W<'_, LockSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&mut self) -> Pin3W<'_, LockSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 6 - desc PIN6"]
    #[inline(always)]
    pub fn pin6(&mut self) -> Pin6W<'_, LockSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - desc PIN7"]
    #[inline(always)]
    pub fn pin7(&mut self) -> Pin7W<'_, LockSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bits 16:31 - KEY = 0x5A5A"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, LockSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "desc LOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
