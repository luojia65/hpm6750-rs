#[doc = "Register `GROUP1_1_VALUE` reader"]
pub struct R(crate::R<GROUP1_1_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GROUP1_1_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GROUP1_1_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GROUP1_1_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GROUP1_1_VALUE` writer"]
pub struct W(crate::W<GROUP1_1_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP1_1_VALUE_SPEC>;
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
impl From<crate::W<GROUP1_1_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP1_1_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK` reader - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
pub type LINK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINK` writer - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
pub type LINK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GROUP1_1_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    #[inline(always)]
    pub fn link(&mut self) -> LINK_W<0> {
        LINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Goup setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group1_1_value](index.html) module"]
pub struct GROUP1_1_VALUE_SPEC;
impl crate::RegisterSpec for GROUP1_1_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [group1_1_value::R](R) reader structure"]
impl crate::Readable for GROUP1_1_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [group1_1_value::W](W) writer structure"]
impl crate::Writable for GROUP1_1_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GROUP1_1_VALUE to value 0"]
impl crate::Resettable for GROUP1_1_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
