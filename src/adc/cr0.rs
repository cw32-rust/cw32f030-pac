#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `EN` reader - desc EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - desc EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BGREN` reader - desc BGREN"]
pub type BgrenR = crate::BitReader;
#[doc = "Field `BGREN` writer - desc BGREN"]
pub type BgrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - desc TSEN"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - desc TSEN"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF` reader - desc REF"]
pub type RefR = crate::FieldReader;
#[doc = "Field `REF` writer - desc REF"]
pub type RefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK` reader - desc CLK"]
pub type ClkR = crate::FieldReader;
#[doc = "Field `CLK` writer - desc CLK"]
pub type ClkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAM` reader - desc SAM"]
pub type SamR = crate::FieldReader;
#[doc = "Field `SAM` writer - desc SAM"]
pub type SamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUF` reader - desc BUF"]
pub type BufR = crate::BitReader;
#[doc = "Field `BUF` writer - desc BUF"]
pub type BufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS` reader - desc BIAS"]
pub type BiasR = crate::FieldReader;
#[doc = "Field `BIAS` writer - desc BIAS"]
pub type BiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BgrenR {
        BgrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - desc REF"]
    #[inline(always)]
    pub fn ref_(&self) -> RefR {
        RefR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - desc CLK"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - desc SAM"]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - desc BUF"]
    #[inline(always)]
    pub fn buf(&self) -> BufR {
        BufR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&self) -> BiasR {
        BiasR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Cr0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cr0Spec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 4 - desc BGREN"]
    #[inline(always)]
    pub fn bgren(&mut self) -> BgrenW<'_, Cr0Spec> {
        BgrenW::new(self, 4)
    }
    #[doc = "Bit 5 - desc TSEN"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<'_, Cr0Spec> {
        TsenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - desc REF"]
    #[inline(always)]
    pub fn ref_(&mut self) -> RefW<'_, Cr0Spec> {
        RefW::new(self, 6)
    }
    #[doc = "Bits 8:10 - desc CLK"]
    #[inline(always)]
    pub fn clk(&mut self) -> ClkW<'_, Cr0Spec> {
        ClkW::new(self, 8)
    }
    #[doc = "Bits 11:12 - desc SAM"]
    #[inline(always)]
    pub fn sam(&mut self) -> SamW<'_, Cr0Spec> {
        SamW::new(self, 11)
    }
    #[doc = "Bit 13 - desc BUF"]
    #[inline(always)]
    pub fn buf(&mut self) -> BufW<'_, Cr0Spec> {
        BufW::new(self, 13)
    }
    #[doc = "Bits 14:15 - desc BIAS"]
    #[inline(always)]
    pub fn bias(&mut self) -> BiasW<'_, Cr0Spec> {
        BiasW::new(self, 14)
    }
}
#[doc = "Control register0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
