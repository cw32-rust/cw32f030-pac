#[doc = "Register `IDR` reader"]
pub type R = crate::R<IdrSpec>;
#[doc = "Field `PIN0` reader - desc PIN0"]
pub type Pin0R = crate::BitReader;
#[doc = "Field `PIN1` reader - desc PIN1"]
pub type Pin1R = crate::BitReader;
#[doc = "Field `PIN2` reader - desc PIN2"]
pub type Pin2R = crate::BitReader;
#[doc = "Field `PIN3` reader - desc PIN3"]
pub type Pin3R = crate::BitReader;
#[doc = "Field `PIN4` reader - desc PIN4"]
pub type Pin4R = crate::BitReader;
#[doc = "Field `PIN5` reader - desc PIN5"]
pub type Pin5R = crate::BitReader;
#[doc = "Field `PIN6` reader - desc PIN6"]
pub type Pin6R = crate::BitReader;
#[doc = "Field `PIN7` reader - desc PIN7"]
pub type Pin7R = crate::BitReader;
#[doc = "Field `PIN8` reader - desc PIN8"]
pub type Pin8R = crate::BitReader;
#[doc = "Field `PIN9` reader - desc PIN9"]
pub type Pin9R = crate::BitReader;
#[doc = "Field `PIN10` reader - desc PIN10"]
pub type Pin10R = crate::BitReader;
#[doc = "Field `PIN11` reader - desc PIN11"]
pub type Pin11R = crate::BitReader;
#[doc = "Field `PIN12` reader - desc PIN12"]
pub type Pin12R = crate::BitReader;
#[doc = "Field `PIN13` reader - desc PIN13"]
pub type Pin13R = crate::BitReader;
#[doc = "Field `PIN14` reader - desc PIN14"]
pub type Pin14R = crate::BitReader;
#[doc = "Field `PIN15` reader - desc PIN15"]
pub type Pin15R = crate::BitReader;
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
    #[doc = "Bit 2 - desc PIN2"]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PIN3"]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PIN4"]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PIN5"]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 8 - desc PIN8"]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PIN9"]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PIN10"]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PIN11"]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PIN12"]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PIN13"]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PIN14"]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PIN15"]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "desc IDR\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idr::R`](R) reader structure"]
impl crate::Readable for IdrSpec {}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
