#[doc = "Register `INT_FLAG` reader"]
pub struct R(crate::R<INT_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_FLAG` writer"]
pub struct W(crate::W<INT_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE` reader - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_FLAG_SPEC, bool, O>;
#[doc = "Field `READ` reader - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
pub type READ_R = crate::BitReader<bool>;
#[doc = "Field `READ` writer - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_FLAG_SPEC, bool, O>;
#[doc = "Field `LOAD` reader - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` writer - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_FLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - fuse write flag, write 1 to clear 0: fuse is not written or writing 1: value in DATA register is programmed into fuse"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<2> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 1 - fuse read flag, write 1 to clear 0: fuse is not read or reading 1: fuse value is put in DATA register"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W<1> {
        READ_W::new(self)
    }
    #[doc = "Bit 0 - fuse load flag, write 1 to clear 0: fuse is not loaded or loading 1: fuse loaded"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_flag](index.html) module"]
pub struct INT_FLAG_SPEC;
impl crate::RegisterSpec for INT_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_flag::R](R) reader structure"]
impl crate::Readable for INT_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_flag::W](W) writer structure"]
impl crate::Writable for INT_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_FLAG to value 0"]
impl crate::Resettable for INT_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
