#[doc = "Register `KEY_1_DATA_1` reader"]
pub struct R(crate::R<KEY_1_DATA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_1_DATA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_1_DATA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_1_DATA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_1_DATA_1` writer"]
pub struct W(crate::W<KEY_1_DATA_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_1_DATA_1_SPEC>;
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
impl From<crate::W<KEY_1_DATA_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_1_DATA_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - security key data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - security key data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY_1_DATA_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - security key data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - security key data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_1_data_1](index.html) module"]
pub struct KEY_1_DATA_1_SPEC;
impl crate::RegisterSpec for KEY_1_DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_1_data_1::R](R) reader structure"]
impl crate::Readable for KEY_1_DATA_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_1_data_1::W](W) writer structure"]
impl crate::Writable for KEY_1_DATA_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_1_DATA_1 to value 0"]
impl crate::Resettable for KEY_1_DATA_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
