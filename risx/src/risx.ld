OUTPUT_ARCH ( "riscv" )
ENTRY ( _start )

SECTIONS
{
    /*
     * OpenSBI is loaded at 0x80000000
     * The kernel has to be loaded at a higher address
     */
    . = 0x80200000;

    .text : {
        *(.text.entry)
        . = ALIGN(16);
        *(.text .text.*)
        PROVIDE ( _end_text = . );
    }

    .rodata : {
        . = ALIGN(16);
        *(.srodata .srodata.*)
        . = ALIGN(16);
        *(.rodata .rodata.*)
    }

    .data : {
        . = ALIGN(16);
        *(.sdata .sdata.*)
        . = ALIGN(16);
        *(.data .data.*)
    }

    .bss : {
        . = ALIGN(16);
        *(.sbss .sbss.*)
        . = ALIGN(16);
        *(.bss .bss.*)
    }

    . = ALIGN(16);

    /* Allocate 1MB stack */
    PROVIDE ( _stack_top = . );
    _stack0 = . + 0x100000;

    /* Free memory starts after the kernel */
    PROVIDE ( _end = . );
}
