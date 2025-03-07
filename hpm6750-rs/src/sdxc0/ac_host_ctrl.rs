#[doc = "Register `AC_HOST_CTRL` reader"]
pub struct R(crate::R<AC_HOST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_HOST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_HOST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_HOST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AC_HOST_CTRL` writer"]
pub struct W(crate::W<AC_HOST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_HOST_CTRL_SPEC>;
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
impl From<crate::W<AC_HOST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_HOST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESET_VAL_ENABLE` reader - Preset Value Enable This bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set, SDCLK frequency generation (Frequency Select and Clock Generator Select) and the driver strength selection are performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode. Values: 0x0 (FALSE): SDCLK and Driver Strength are controlled by Host Driver 0x1 (TRUE): Automatic Selection by Preset Value are Enabled"]
pub type PRESET_VAL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PRESET_VAL_ENABLE` writer - Preset Value Enable This bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set, SDCLK frequency generation (Frequency Select and Clock Generator Select) and the driver strength selection are performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode. Values: 0x0 (FALSE): SDCLK and Driver Strength are controlled by Host Driver 0x1 (TRUE): Automatic Selection by Preset Value are Enabled"]
pub type PRESET_VAL_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `ASYNC_INT_ENABLE` reader - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: 0x0 (FALSE): Disabled 0x1 (TRUE): Enabled"]
pub type ASYNC_INT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ASYNC_INT_ENABLE` writer - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: 0x0 (FALSE): Disabled 0x1 (TRUE): Enabled"]
pub type ASYNC_INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `HOST_VER4_ENABLE` reader - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register 64-bit ADMA Descriptor Size: 128-bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1 Selection of 32-bit/64-bit System Addressing: Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers, UHS-II registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: 0x0 (FALSE): Version 3.00 compatible mode 0x1 (TRUE): Version 4 mode"]
pub type HOST_VER4_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `HOST_VER4_ENABLE` writer - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register 64-bit ADMA Descriptor Size: 128-bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1 Selection of 32-bit/64-bit System Addressing: Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers, UHS-II registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: 0x0 (FALSE): Version 3.00 compatible mode 0x1 (TRUE): Version 4 mode"]
pub type HOST_VER4_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `CMD23_ENABLE` reader - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: 0x0 (FALSE): Auto CMD23 is disabled 0x1 (TRUE): Auto CMD23 is enabled"]
pub type CMD23_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CMD23_ENABLE` writer - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: 0x0 (FALSE): Auto CMD23 is disabled 0x1 (TRUE): Auto CMD23 is enabled"]
pub type CMD23_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `ADMA2_LEN_MODE` reader - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: 0x0 (FALSE): 16-bit Data Length Mode 0x1 (TRUE): 26-bit Data Length Mode"]
pub type ADMA2_LEN_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ADMA2_LEN_MODE` writer - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: 0x0 (FALSE): 16-bit Data Length Mode 0x1 (TRUE): 26-bit Data Length Mode"]
pub type ADMA2_LEN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `SAMPLE_CLK_SEL` reader - Sampling Clock Select This bit is used by the Host Controller to select the sampling clock in SD/eMMC mode to receive CMD and DAT. This bit is set by the tuning procedure and is valid after the completion of tuning (when Execute Tuning is cleared). Setting this bit to 1 means that tuning is completed successfully and setting this bit to 0 means that tuning has failed. The value is reflected on the sample_cclk_sel pin. Values: 0x0 (FALSE): Fixed clock is used to sample data 0x1 (TRUE): Tuned clock is used to sample data"]
pub type SAMPLE_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLE_CLK_SEL` writer - Sampling Clock Select This bit is used by the Host Controller to select the sampling clock in SD/eMMC mode to receive CMD and DAT. This bit is set by the tuning procedure and is valid after the completion of tuning (when Execute Tuning is cleared). Setting this bit to 1 means that tuning is completed successfully and setting this bit to 0 means that tuning has failed. The value is reflected on the sample_cclk_sel pin. Values: 0x0 (FALSE): Fixed clock is used to sample data 0x1 (TRUE): Tuned clock is used to sample data"]
pub type SAMPLE_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `EXEC_TUNING` reader - Execute Tuning This bit is set to 1 to start the tuning procedure in UHS-I/eMMC speed modes and this bit is automatically cleared when tuning procedure is completed. Values: 0x0 (FALSE): Not Tuned or Tuning completed 0x1 (TRUE): Execute Tuning"]
pub type EXEC_TUNING_R = crate::BitReader<bool>;
#[doc = "Field `EXEC_TUNING` writer - Execute Tuning This bit is set to 1 to start the tuning procedure in UHS-I/eMMC speed modes and this bit is automatically cleared when tuning procedure is completed. Values: 0x0 (FALSE): Not Tuned or Tuning completed 0x1 (TRUE): Execute Tuning"]
pub type EXEC_TUNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `SIGNALING_EN` reader - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in UHS-I/eMMC speed modes. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8 signaling fails. The value is reflected on the uhs1_swvolt_en pin. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/SDR104/DDR50). Values: 0x0 (V_3_3): 3.3V Signalling 0x1 (V_1_8): 1.8V Signalling"]
pub type SIGNALING_EN_R = crate::BitReader<bool>;
#[doc = "Field `SIGNALING_EN` writer - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in UHS-I/eMMC speed modes. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8 signaling fails. The value is reflected on the uhs1_swvolt_en pin. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/SDR104/DDR50). Values: 0x0 (V_3_3): 3.3V Signalling 0x1 (V_1_8): 1.8V Signalling"]
pub type SIGNALING_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_HOST_CTRL_SPEC, bool, O>;
#[doc = "Field `UHS_MODE_SEL` reader - UHS Mode/eMMC Speed Mode Select These bits are used to select UHS mode in the SD mode of operation. In eMMC mode, these bits are used to select eMMC Speed mode. UHS Mode (SD/UHS-II mode only): 0x0 (SDR12): SDR12/Legacy 0x1 (SDR25): SDR25/High Speed SDR 0x2 (SDR50): SDR50 0x3 (SDR104): SDR104/HS200 0x4 (DDR50): DDR50/High Speed DDR 0x5 (RSVD5): Reserved 0x6 (RSVD6): Reserved 0x7 (UHS2): UHS-II/HS400 eMMC Speed Mode (eMMC mode only): 0x0: Legacy 0x1: High Speed SDR 0x2: Reserved 0x3: HS200 0x4: High Speed DDR 0x5: Reserved 0x6: Reserved 0x7: HS400"]
pub type UHS_MODE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UHS_MODE_SEL` writer - UHS Mode/eMMC Speed Mode Select These bits are used to select UHS mode in the SD mode of operation. In eMMC mode, these bits are used to select eMMC Speed mode. UHS Mode (SD/UHS-II mode only): 0x0 (SDR12): SDR12/Legacy 0x1 (SDR25): SDR25/High Speed SDR 0x2 (SDR50): SDR50 0x3 (SDR104): SDR104/HS200 0x4 (DDR50): DDR50/High Speed DDR 0x5 (RSVD5): Reserved 0x6 (RSVD6): Reserved 0x7 (UHS2): UHS-II/HS400 eMMC Speed Mode (eMMC mode only): 0x0: Legacy 0x1: High Speed SDR 0x2: Reserved 0x3: HS200 0x4: High Speed DDR 0x5: Reserved 0x6: Reserved 0x7: HS400"]
pub type UHS_MODE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_HOST_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CMD_NOT_ISSUED_AUTO_CMD12` reader - Command Not Issued By Auto CMD12 Error If this bit is set to 1, CMD_wo_DAT is not executed due to an Auto CMD12 Error (D04-D01) in this register. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Values: 0x1 (TRUE): Not Issued 0x0 (FALSE): No Error"]
pub type CMD_NOT_ISSUED_AUTO_CMD12_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_RESP_ERR` reader - Auto CMD Response Error This bit is set when Response Error Check Enable in the Transfer Mode register is set to 1 and an error is detected in R1 response of either Auto CMD12 or CMD13. This status is ignored if any bit between D00 to D04 is set to 1. Values: 0x1 (TRUE): Error 0x0 (FALSE): No Error"]
pub type AUTO_CMD_RESP_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_IDX_ERR` reader - Auto CMD Index Error This bit is set if the command index error occurs in response to a command. Values: 0x1 (TRUE): Error 0x0 (FALSE): No Error"]
pub type AUTO_CMD_IDX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_EBIT_ERR` reader - Auto CMD End Bit Error This bit is set when detecting that the end bit of command response is 0. Values: 0x1 (TRUE): End Bit Error Generated 0x0 (FALSE): No Error"]
pub type AUTO_CMD_EBIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_CRC_ERR` reader - Auto CMD CRC Error This bit is set when detecting a CRC error in the command response. Values: 0x1 (TRUE): CRC Error Generated 0x0 (FALSE): No Error"]
pub type AUTO_CMD_CRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_TOUT_ERR` reader - Auto CMD Timeout Error This bit is set if no response is returned with 64 SDCLK cycles from the end bit of the command. If this bit is set to 1, error status bits (D04-D01) are meaningless. Values: 0x1 (TRUE): Time out 0x0 (FALSE): No Error"]
pub type AUTO_CMD_TOUT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD12_NOT_EXEC` reader - Auto CMD12 Not Executed If multiple memory block data transfer is not started due to a command error, this bit is not set because it is not necessary to issue an Auto CMD12. Setting this bit to 1 means that the Host Controller cannot issue Auto CMD12 to stop multiple memory block data transfer, due to some error. If this bit is set to 1, error status bits (D04-D01) is meaningless. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Values: 0x1 (TRUE): Not Executed 0x0 (FALSE): Executed"]
pub type AUTO_CMD12_NOT_EXEC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - Preset Value Enable This bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set, SDCLK frequency generation (Frequency Select and Clock Generator Select) and the driver strength selection are performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode. Values: 0x0 (FALSE): SDCLK and Driver Strength are controlled by Host Driver 0x1 (TRUE): Automatic Selection by Preset Value are Enabled"]
    #[inline(always)]
    pub fn preset_val_enable(&self) -> PRESET_VAL_ENABLE_R {
        PRESET_VAL_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: 0x0 (FALSE): Disabled 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn async_int_enable(&self) -> ASYNC_INT_ENABLE_R {
        ASYNC_INT_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 28 - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register 64-bit ADMA Descriptor Size: 128-bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1 Selection of 32-bit/64-bit System Addressing: Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers, UHS-II registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: 0x0 (FALSE): Version 3.00 compatible mode 0x1 (TRUE): Version 4 mode"]
    #[inline(always)]
    pub fn host_ver4_enable(&self) -> HOST_VER4_ENABLE_R {
        HOST_VER4_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: 0x0 (FALSE): Auto CMD23 is disabled 0x1 (TRUE): Auto CMD23 is enabled"]
    #[inline(always)]
    pub fn cmd23_enable(&self) -> CMD23_ENABLE_R {
        CMD23_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: 0x0 (FALSE): 16-bit Data Length Mode 0x1 (TRUE): 26-bit Data Length Mode"]
    #[inline(always)]
    pub fn adma2_len_mode(&self) -> ADMA2_LEN_MODE_R {
        ADMA2_LEN_MODE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 23 - Sampling Clock Select This bit is used by the Host Controller to select the sampling clock in SD/eMMC mode to receive CMD and DAT. This bit is set by the tuning procedure and is valid after the completion of tuning (when Execute Tuning is cleared). Setting this bit to 1 means that tuning is completed successfully and setting this bit to 0 means that tuning has failed. The value is reflected on the sample_cclk_sel pin. Values: 0x0 (FALSE): Fixed clock is used to sample data 0x1 (TRUE): Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn sample_clk_sel(&self) -> SAMPLE_CLK_SEL_R {
        SAMPLE_CLK_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Execute Tuning This bit is set to 1 to start the tuning procedure in UHS-I/eMMC speed modes and this bit is automatically cleared when tuning procedure is completed. Values: 0x0 (FALSE): Not Tuned or Tuning completed 0x1 (TRUE): Execute Tuning"]
    #[inline(always)]
    pub fn exec_tuning(&self) -> EXEC_TUNING_R {
        EXEC_TUNING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 19 - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in UHS-I/eMMC speed modes. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8 signaling fails. The value is reflected on the uhs1_swvolt_en pin. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/SDR104/DDR50). Values: 0x0 (V_3_3): 3.3V Signalling 0x1 (V_1_8): 1.8V Signalling"]
    #[inline(always)]
    pub fn signaling_en(&self) -> SIGNALING_EN_R {
        SIGNALING_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 16:18 - UHS Mode/eMMC Speed Mode Select These bits are used to select UHS mode in the SD mode of operation. In eMMC mode, these bits are used to select eMMC Speed mode. UHS Mode (SD/UHS-II mode only): 0x0 (SDR12): SDR12/Legacy 0x1 (SDR25): SDR25/High Speed SDR 0x2 (SDR50): SDR50 0x3 (SDR104): SDR104/HS200 0x4 (DDR50): DDR50/High Speed DDR 0x5 (RSVD5): Reserved 0x6 (RSVD6): Reserved 0x7 (UHS2): UHS-II/HS400 eMMC Speed Mode (eMMC mode only): 0x0: Legacy 0x1: High Speed SDR 0x2: Reserved 0x3: HS200 0x4: High Speed DDR 0x5: Reserved 0x6: Reserved 0x7: HS400"]
    #[inline(always)]
    pub fn uhs_mode_sel(&self) -> UHS_MODE_SEL_R {
        UHS_MODE_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error If this bit is set to 1, CMD_wo_DAT is not executed due to an Auto CMD12 Error (D04-D01) in this register. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Values: 0x1 (TRUE): Not Issued 0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn cmd_not_issued_auto_cmd12(&self) -> CMD_NOT_ISSUED_AUTO_CMD12_R {
        CMD_NOT_ISSUED_AUTO_CMD12_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto CMD Response Error This bit is set when Response Error Check Enable in the Transfer Mode register is set to 1 and an error is detected in R1 response of either Auto CMD12 or CMD13. This status is ignored if any bit between D00 to D04 is set to 1. Values: 0x1 (TRUE): Error 0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_resp_err(&self) -> AUTO_CMD_RESP_ERR_R {
        AUTO_CMD_RESP_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error This bit is set if the command index error occurs in response to a command. Values: 0x1 (TRUE): Error 0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_idx_err(&self) -> AUTO_CMD_IDX_ERR_R {
        AUTO_CMD_IDX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error This bit is set when detecting that the end bit of command response is 0. Values: 0x1 (TRUE): End Bit Error Generated 0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_ebit_err(&self) -> AUTO_CMD_EBIT_ERR_R {
        AUTO_CMD_EBIT_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error This bit is set when detecting a CRC error in the command response. Values: 0x1 (TRUE): CRC Error Generated 0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_crc_err(&self) -> AUTO_CMD_CRC_ERR_R {
        AUTO_CMD_CRC_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error This bit is set if no response is returned with 64 SDCLK cycles from the end bit of the command. If this bit is set to 1, error status bits (D04-D01) are meaningless. Values: 0x1 (TRUE): Time out 0x0 (FALSE): No Error"]
    #[inline(always)]
    pub fn auto_cmd_tout_err(&self) -> AUTO_CMD_TOUT_ERR_R {
        AUTO_CMD_TOUT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Auto CMD12 Not Executed If multiple memory block data transfer is not started due to a command error, this bit is not set because it is not necessary to issue an Auto CMD12. Setting this bit to 1 means that the Host Controller cannot issue Auto CMD12 to stop multiple memory block data transfer, due to some error. If this bit is set to 1, error status bits (D04-D01) is meaningless. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Values: 0x1 (TRUE): Not Executed 0x0 (FALSE): Executed"]
    #[inline(always)]
    pub fn auto_cmd12_not_exec(&self) -> AUTO_CMD12_NOT_EXEC_R {
        AUTO_CMD12_NOT_EXEC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Preset Value Enable This bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set, SDCLK frequency generation (Frequency Select and Clock Generator Select) and the driver strength selection are performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode. Values: 0x0 (FALSE): SDCLK and Driver Strength are controlled by Host Driver 0x1 (TRUE): Automatic Selection by Preset Value are Enabled"]
    #[inline(always)]
    pub fn preset_val_enable(&mut self) -> PRESET_VAL_ENABLE_W<31> {
        PRESET_VAL_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: 0x0 (FALSE): Disabled 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn async_int_enable(&mut self) -> ASYNC_INT_ENABLE_W<30> {
        ASYNC_INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register 64-bit ADMA Descriptor Size: 128-bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1 Selection of 32-bit/64-bit System Addressing: Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers, UHS-II registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: 0x0 (FALSE): Version 3.00 compatible mode 0x1 (TRUE): Version 4 mode"]
    #[inline(always)]
    pub fn host_ver4_enable(&mut self) -> HOST_VER4_ENABLE_W<28> {
        HOST_VER4_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: 0x0 (FALSE): Auto CMD23 is disabled 0x1 (TRUE): Auto CMD23 is enabled"]
    #[inline(always)]
    pub fn cmd23_enable(&mut self) -> CMD23_ENABLE_W<27> {
        CMD23_ENABLE_W::new(self)
    }
    #[doc = "Bit 26 - ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: 0x0 (FALSE): 16-bit Data Length Mode 0x1 (TRUE): 26-bit Data Length Mode"]
    #[inline(always)]
    pub fn adma2_len_mode(&mut self) -> ADMA2_LEN_MODE_W<26> {
        ADMA2_LEN_MODE_W::new(self)
    }
    #[doc = "Bit 23 - Sampling Clock Select This bit is used by the Host Controller to select the sampling clock in SD/eMMC mode to receive CMD and DAT. This bit is set by the tuning procedure and is valid after the completion of tuning (when Execute Tuning is cleared). Setting this bit to 1 means that tuning is completed successfully and setting this bit to 0 means that tuning has failed. The value is reflected on the sample_cclk_sel pin. Values: 0x0 (FALSE): Fixed clock is used to sample data 0x1 (TRUE): Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn sample_clk_sel(&mut self) -> SAMPLE_CLK_SEL_W<23> {
        SAMPLE_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Execute Tuning This bit is set to 1 to start the tuning procedure in UHS-I/eMMC speed modes and this bit is automatically cleared when tuning procedure is completed. Values: 0x0 (FALSE): Not Tuned or Tuning completed 0x1 (TRUE): Execute Tuning"]
    #[inline(always)]
    pub fn exec_tuning(&mut self) -> EXEC_TUNING_W<22> {
        EXEC_TUNING_W::new(self)
    }
    #[doc = "Bit 19 - 1.8V Signaling Enable This bit controls voltage regulator for I/O cell in UHS-I/eMMC speed modes. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8 signaling fails. The value is reflected on the uhs1_swvolt_en pin. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/SDR104/DDR50). Values: 0x0 (V_3_3): 3.3V Signalling 0x1 (V_1_8): 1.8V Signalling"]
    #[inline(always)]
    pub fn signaling_en(&mut self) -> SIGNALING_EN_W<19> {
        SIGNALING_EN_W::new(self)
    }
    #[doc = "Bits 16:18 - UHS Mode/eMMC Speed Mode Select These bits are used to select UHS mode in the SD mode of operation. In eMMC mode, these bits are used to select eMMC Speed mode. UHS Mode (SD/UHS-II mode only): 0x0 (SDR12): SDR12/Legacy 0x1 (SDR25): SDR25/High Speed SDR 0x2 (SDR50): SDR50 0x3 (SDR104): SDR104/HS200 0x4 (DDR50): DDR50/High Speed DDR 0x5 (RSVD5): Reserved 0x6 (RSVD6): Reserved 0x7 (UHS2): UHS-II/HS400 eMMC Speed Mode (eMMC mode only): 0x0: Legacy 0x1: High Speed SDR 0x2: Reserved 0x3: HS200 0x4: High Speed DDR 0x5: Reserved 0x6: Reserved 0x7: HS400"]
    #[inline(always)]
    pub fn uhs_mode_sel(&mut self) -> UHS_MODE_SEL_W<16> {
        UHS_MODE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_host_ctrl](index.html) module"]
pub struct AC_HOST_CTRL_SPEC;
impl crate::RegisterSpec for AC_HOST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_host_ctrl::R](R) reader structure"]
impl crate::Readable for AC_HOST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_host_ctrl::W](W) writer structure"]
impl crate::Writable for AC_HOST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AC_HOST_CTRL to value 0"]
impl crate::Resettable for AC_HOST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
