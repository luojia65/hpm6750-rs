#[doc = "Register `HPF_B` reader"]
pub struct R(crate::R<HPF_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPF_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPF_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPF_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPF_B` writer"]
pub struct W(crate::W<HPF_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPF_B_SPEC>;
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
impl From<crate::W<HPF_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPF_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEF` reader - coef B of the Order-1 HPF"]
pub type COEF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COEF` writer - coef B of the Order-1 HPF"]
pub type COEF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPF_B_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - coef B of the Order-1 HPF"]
    #[inline(always)]
    pub fn coef(&self) -> COEF_R {
        COEF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - coef B of the Order-1 HPF"]
    #[inline(always)]
    pub fn coef(&mut self) -> COEF_W<0> {
        COEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HPF B Coef Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpf_b](index.html) module"]
pub struct HPF_B_SPEC;
impl crate::RegisterSpec for HPF_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpf_b::R](R) reader structure"]
impl crate::Readable for HPF_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpf_b::W](W) writer structure"]
impl crate::Writable for HPF_B_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HPF_B to value 0"]
impl crate::Resettable for HPF_B_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
