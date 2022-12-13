OUTPUT_ARCH(riscv)
ENTRY(_entry)
BASE_ADDRESS = 0x80000000;

SECTIONS
{
        . = BASE_ADDRESS;
        skernel = .;
        stext = .;
        .text : {
                *(.text.entry)
                *(.text .text.*)
                . = ALIGN(4K);
                _trampoline = .;
                *(trampsec)
                . = ALIGN(4K);
                ASSERT(. - _trampoline == 4K, "error: tranpoline larger than one page");

        }
        etext = .;
        srodata = .;
        .rodata : {
                *(.rodata .rodata.*)
                *(.srodata .srodata.*)
        }
        
        . = ALIGN(4K);
        erodata = .;
        sdata = .;
        .data : {
                *(.data .data.*)
                *(.sdata .sdata.*)
        }

        . = ALIGN(4K);
        edata = .;
        .bss : {
                *(.bss.stack)
                sbss = .;
                *(.bss .bss.*)
                *(.sbss .sbss.*)
        }

        . = ALIGN(4K);
        ebss = .;
        ekernel = .;

        /DISCARD/ : {
                *(.eh_frame)
        }
}
