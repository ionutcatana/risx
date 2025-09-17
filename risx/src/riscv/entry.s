.section .text.entry
.globl _start

_start:
        # stack for the boot hart
        la      sp, _stack0
        slli    t0, a0, 16
        add     sp, sp, t0

        # call main with args
        # a0: hartid
        # a1: dtb address
        call    main

_halt:
        j       _halt
