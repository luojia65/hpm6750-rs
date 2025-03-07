#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Button status"]
    pub btn_status: crate::Reg<btn_status::BTN_STATUS_SPEC>,
    #[doc = "0x04 - Button interrupt mask"]
    pub btn_irq_mask: crate::Reg<btn_irq_mask::BTN_IRQ_MASK_SPEC>,
    #[doc = "0x08 - Debounce setting"]
    pub led_intense: crate::Reg<led_intense::LED_INTENSE_SPEC>,
}
#[doc = "BTN_STATUS register accessor: an alias for `Reg<BTN_STATUS_SPEC>`"]
pub type BTN_STATUS = crate::Reg<btn_status::BTN_STATUS_SPEC>;
#[doc = "Button status"]
pub mod btn_status;
#[doc = "BTN_IRQ_MASK register accessor: an alias for `Reg<BTN_IRQ_MASK_SPEC>`"]
pub type BTN_IRQ_MASK = crate::Reg<btn_irq_mask::BTN_IRQ_MASK_SPEC>;
#[doc = "Button interrupt mask"]
pub mod btn_irq_mask;
#[doc = "LED_INTENSE register accessor: an alias for `Reg<LED_INTENSE_SPEC>`"]
pub type LED_INTENSE = crate::Reg<led_intense::LED_INTENSE_SPEC>;
#[doc = "Debounce setting"]
pub mod led_intense;
