#[doc = "Register `CPU_CPU1_LOCK` reader"]
pub struct R(crate::R<CPU_CPU1_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CPU1_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CPU1_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CPU1_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CPU1_LOCK` writer"]
pub struct W(crate::W<CPU_CPU1_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CPU1_LOCK_SPEC>;
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
impl From<crate::W<CPU_CPU1_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CPU1_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPR` reader - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
pub type GPR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPR` writer - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
pub type GPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CPU1_LOCK_SPEC, u16, u16, 14, O>;
#[doc = "Field `LOCK` reader - Lock bit for CPU_LOCK"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock bit for CPU_LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_CPU1_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:15 - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 1 - Lock bit for CPU_LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
    #[inline(always)]
    pub fn gpr(&mut self) -> GPR_W<2> {
        GPR_W::new(self)
    }
    #[doc = "Bit 1 - Lock bit for CPU_LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<1> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_cpu1_lock](index.html) module"]
pub struct CPU_CPU1_LOCK_SPEC;
impl crate::RegisterSpec for CPU_CPU1_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_cpu1_lock::R](R) reader structure"]
impl crate::Readable for CPU_CPU1_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_cpu1_lock::W](W) writer structure"]
impl crate::Writable for CPU_CPU1_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_CPU1_LOCK to value 0x02"]
impl crate::Resettable for CPU_CPU1_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
