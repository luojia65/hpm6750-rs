#[doc = "Register `BUF_DATA` reader"]
pub struct R(crate::R<BUF_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_DATA` writer"]
pub struct W(crate::W<BUF_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_DATA_SPEC>;
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
impl From<crate::W<BUF_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF_DATA` reader - Buffer Data These bits enable access to the Host Controller packet buffer."]
pub type BUF_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BUF_DATA` writer - Buffer Data These bits enable access to the Host Controller packet buffer."]
pub type BUF_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    pub fn buf_data(&self) -> BUF_DATA_R {
        BUF_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    pub fn buf_data(&mut self) -> BUF_DATA_W<0> {
        BUF_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_data](index.html) module"]
pub struct BUF_DATA_SPEC;
impl crate::RegisterSpec for BUF_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_data::R](R) reader structure"]
impl crate::Readable for BUF_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_data::W](W) writer structure"]
impl crate::Writable for BUF_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_DATA to value 0"]
impl crate::Resettable for BUF_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
