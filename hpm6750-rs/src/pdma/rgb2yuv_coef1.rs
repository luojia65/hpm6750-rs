#[doc = "Register `RGB2YUV_COEF1` reader"]
pub struct R(crate::R<RGB2YUV_COEF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGB2YUV_COEF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGB2YUV_COEF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGB2YUV_COEF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGB2YUV_COEF1` writer"]
pub struct W(crate::W<RGB2YUV_COEF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGB2YUV_COEF1_SPEC>;
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
impl From<crate::W<RGB2YUV_COEF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGB2YUV_COEF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C1` reader - CSC parameters C1"]
pub type C1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C1` writer - CSC parameters C1"]
pub type C1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF1_SPEC, u16, u16, 11, O>;
#[doc = "Field `C4` reader - CSC parameters C4"]
pub type C4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C4` writer - CSC parameters C4"]
pub type C4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGB2YUV_COEF1_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 16:26 - CSC parameters C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10 - CSC parameters C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26 - CSC parameters C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W<16> {
        C1_W::new(self)
    }
    #[doc = "Bits 0:10 - CSC parameters C4"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4_W<0> {
        C4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RGB2YUV coefficients register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgb2yuv_coef1](index.html) module"]
pub struct RGB2YUV_COEF1_SPEC;
impl crate::RegisterSpec for RGB2YUV_COEF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgb2yuv_coef1::R](R) reader structure"]
impl crate::Readable for RGB2YUV_COEF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgb2yuv_coef1::W](W) writer structure"]
impl crate::Writable for RGB2YUV_COEF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RGB2YUV_COEF1 to value 0"]
impl crate::Resettable for RGB2YUV_COEF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
