#[doc = "Register `DO_GPIOC_VALUE` reader"]
pub struct R(crate::R<DO_GPIOC_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DO_GPIOC_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DO_GPIOC_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DO_GPIOC_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DO_GPIOC_VALUE` writer"]
pub struct W(crate::W<DO_GPIOC_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DO_GPIOC_VALUE_SPEC>;
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
impl From<crate::W<DO_GPIOC_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DO_GPIOC_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPUT` reader - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
pub type OUTPUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTPUT` writer - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
pub type OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DO_GPIOC_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    #[inline(always)]
    pub fn output(&mut self) -> OUTPUT_W<0> {
        OUTPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOC output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [do_gpioc_value](index.html) module"]
pub struct DO_GPIOC_VALUE_SPEC;
impl crate::RegisterSpec for DO_GPIOC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [do_gpioc_value::R](R) reader structure"]
impl crate::Readable for DO_GPIOC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [do_gpioc_value::W](W) writer structure"]
impl crate::Writable for DO_GPIOC_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DO_GPIOC_VALUE to value 0"]
impl crate::Resettable for DO_GPIOC_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
