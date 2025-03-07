#[doc = "Register `DLM` reader"]
pub struct R(crate::R<DLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLM` writer"]
pub struct W(crate::W<DLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLM_SPEC>;
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
impl From<crate::W<DLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLM` reader - Most significant byte of the Divisor Latch"]
pub type DLM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLM` writer - Most significant byte of the Divisor Latch"]
pub type DLM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Most significant byte of the Divisor Latch"]
    #[inline(always)]
    pub fn dlm(&self) -> DLM_R {
        DLM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Most significant byte of the Divisor Latch"]
    #[inline(always)]
    pub fn dlm(&mut self) -> DLM_W<0> {
        DLM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch MSB (when DLAB = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm](index.html) module"]
pub struct DLM_SPEC;
impl crate::RegisterSpec for DLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlm::R](R) reader structure"]
impl crate::Readable for DLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlm::W](W) writer structure"]
impl crate::Writable for DLM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DLM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
