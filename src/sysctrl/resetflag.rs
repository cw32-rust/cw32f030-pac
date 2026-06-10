#[doc = "Register `RESETFLAG` reader"]
pub type R = crate::R<ResetflagSpec>;
#[doc = "Register `RESETFLAG` writer"]
pub type W = crate::W<ResetflagSpec>;
#[doc = "Field `POR` reader - desc POR"]
pub type PorR = crate::BitReader;
#[doc = "Field `POR` writer - desc POR"]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVD` reader - desc LVD"]
pub type LvdR = crate::BitReader;
#[doc = "Field `LVD` writer - desc LVD"]
pub type LvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDT` reader - desc IWDT"]
pub type IwdtR = crate::BitReader;
#[doc = "Field `IWDT` writer - desc IWDT"]
pub type IwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT` reader - desc WWDT"]
pub type WwdtR = crate::BitReader;
#[doc = "Field `WWDT` writer - desc WWDT"]
pub type WwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTB` reader - desc RSTB"]
pub type RstbR = crate::BitReader;
#[doc = "Field `RSTB` writer - desc RSTB"]
pub type RstbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKUP` reader - desc LOCKUP"]
pub type LockupR = crate::BitReader;
#[doc = "Field `LOCKUP` writer - desc LOCKUP"]
pub type LockupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRESETREQ` reader - desc SYSRESETREQ"]
pub type SysresetreqR = crate::BitReader;
#[doc = "Field `SYSRESETREQ` writer - desc SYSRESETREQ"]
pub type SysresetreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc POR"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - desc LVD"]
    #[inline(always)]
    pub fn lvd(&self) -> LvdR {
        LvdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&self) -> IwdtR {
        IwdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&self) -> WwdtR {
        WwdtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RSTB"]
    #[inline(always)]
    pub fn rstb(&self) -> RstbR {
        RstbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SYSRESETREQ"]
    #[inline(always)]
    pub fn sysresetreq(&self) -> SysresetreqR {
        SysresetreqR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc POR"]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<'_, ResetflagSpec> {
        PorW::new(self, 0)
    }
    #[doc = "Bit 3 - desc LVD"]
    #[inline(always)]
    pub fn lvd(&mut self) -> LvdW<'_, ResetflagSpec> {
        LvdW::new(self, 3)
    }
    #[doc = "Bit 4 - desc IWDT"]
    #[inline(always)]
    pub fn iwdt(&mut self) -> IwdtW<'_, ResetflagSpec> {
        IwdtW::new(self, 4)
    }
    #[doc = "Bit 5 - desc WWDT"]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WwdtW<'_, ResetflagSpec> {
        WwdtW::new(self, 5)
    }
    #[doc = "Bit 6 - desc RSTB"]
    #[inline(always)]
    pub fn rstb(&mut self) -> RstbW<'_, ResetflagSpec> {
        RstbW::new(self, 6)
    }
    #[doc = "Bit 8 - desc LOCKUP"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LockupW<'_, ResetflagSpec> {
        LockupW::new(self, 8)
    }
    #[doc = "Bit 9 - desc SYSRESETREQ"]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SysresetreqW<'_, ResetflagSpec> {
        SysresetreqW::new(self, 9)
    }
}
#[doc = "Reset Status Reg\n\nYou can [`read`](crate::Reg::read) this register and get [`resetflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetflagSpec;
impl crate::RegisterSpec for ResetflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetflag::R`](R) reader structure"]
impl crate::Readable for ResetflagSpec {}
#[doc = "`write(|w| ..)` method takes [`resetflag::W`](W) writer structure"]
impl crate::Writable for ResetflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESETFLAG to value 0"]
impl crate::Resettable for ResetflagSpec {}
