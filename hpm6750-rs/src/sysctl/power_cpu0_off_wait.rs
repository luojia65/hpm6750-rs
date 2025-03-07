#[doc = "Register `POWER_CPU0_OFF_WAIT` reader"]
pub struct R(crate::R<POWER_CPU0_OFF_WAIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_CPU0_OFF_WAIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_CPU0_OFF_WAIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_CPU0_OFF_WAIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_CPU0_OFF_WAIT` writer"]
pub struct W(crate::W<POWER_CPU0_OFF_WAIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_CPU0_OFF_WAIT_SPEC>;
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
impl From<crate::W<POWER_CPU0_OFF_WAIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_CPU0_OFF_WAIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT` reader - wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
pub type WAIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WAIT` writer - wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
pub type WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, POWER_CPU0_OFF_WAIT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W<0> {
        WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_cpu0_off_wait](index.html) module"]
pub struct POWER_CPU0_OFF_WAIT_SPEC;
impl crate::RegisterSpec for POWER_CPU0_OFF_WAIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_cpu0_off_wait::R](R) reader structure"]
impl crate::Readable for POWER_CPU0_OFF_WAIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_cpu0_off_wait::W](W) writer structure"]
impl crate::Writable for POWER_CPU0_OFF_WAIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER_CPU0_OFF_WAIT to value 0x15"]
impl crate::Resettable for POWER_CPU0_OFF_WAIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x15
    }
}
