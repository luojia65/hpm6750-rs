fn main() {
    use std::{env, fs, path::PathBuf};

    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(ld, LINKER).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}

const LINKER: &[u8] = br#"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY
{
  CORE0_LM_SLV : ORIGIN = 0x01000000, LENGTH = 512K
  AXI_SRAM : ORIGIN = 0x01080000, LENGTH = 1M
}
REGION_ALIAS("REGION_TEXT", CORE0_LM_SLV);
REGION_ALIAS("REGION_RODATA", CORE0_LM_SLV);
REGION_ALIAS("REGION_DATA", AXI_SRAM);
REGION_ALIAS("REGION_BSS", AXI_SRAM);
REGION_ALIAS("REGION_HEAP", AXI_SRAM);
REGION_ALIAS("REGION_STACK", AXI_SRAM);
SECTIONS {
    .text : ALIGN(4) {
        *(.text.entry)
        *(.text .text.*)
    } > REGION_TEXT
    .rodata : ALIGN(8) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);
        erodata = .;
    } > REGION_RODATA
    .data : ALIGN(8) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8);
        edata = .;
    } > REGION_DATA
    .bss (NOLOAD) : ALIGN(8) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > REGION_BSS
    /DISCARD/ : {
        *(.eh_frame)
    }
}"#;
