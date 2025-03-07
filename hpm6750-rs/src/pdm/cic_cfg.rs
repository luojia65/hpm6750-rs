#[doc = "Register `CIC_CFG` reader"]
pub struct R(crate::R<CIC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIC_CFG` writer"]
pub struct W(crate::W<CIC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIC_CFG_SPEC>;
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
impl From<crate::W<CIC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POST_SCALE` reader - the shift value after CIC results."]
pub type POST_SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POST_SCALE` writer - the shift value after CIC results."]
pub type POST_SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIC_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `SGD` reader - Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused"]
pub type SGD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SGD` writer - Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused"]
pub type SGD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIC_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CIC_DEC_RATIO` reader - CIC decimation factor"]
pub type CIC_DEC_RATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CIC_DEC_RATIO` writer - CIC decimation factor"]
pub type CIC_DEC_RATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIC_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 10:15 - the shift value after CIC results."]
    #[inline(always)]
    pub fn post_scale(&self) -> POST_SCALE_R {
        POST_SCALE_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused"]
    #[inline(always)]
    pub fn sgd(&self) -> SGD_R {
        SGD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:7 - CIC decimation factor"]
    #[inline(always)]
    pub fn cic_dec_ratio(&self) -> CIC_DEC_RATIO_R {
        CIC_DEC_RATIO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - the shift value after CIC results."]
    #[inline(always)]
    pub fn post_scale(&mut self) -> POST_SCALE_W<10> {
        POST_SCALE_W::new(self)
    }
    #[doc = "Bits 8:9 - Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused"]
    #[inline(always)]
    pub fn sgd(&mut self) -> SGD_W<8> {
        SGD_W::new(self)
    }
    #[doc = "Bits 0:7 - CIC decimation factor"]
    #[inline(always)]
    pub fn cic_dec_ratio(&mut self) -> CIC_DEC_RATIO_W<0> {
        CIC_DEC_RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cic_cfg](index.html) module"]
pub struct CIC_CFG_SPEC;
impl crate::RegisterSpec for CIC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cic_cfg::R](R) reader structure"]
impl crate::Readable for CIC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cic_cfg::W](W) writer structure"]
impl crate::Writable for CIC_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIC_CFG to value 0"]
impl crate::Resettable for CIC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
