.section .rodata
test_msg:
        .ascii "Test message from risx!\n"

.section .text.entry
.globl _start

.equ _stack_size, 0x100000              # 1MB stack
_start:
        mv      s0, a0                  # a0 = hart id
        mv      s1, a1                  # a1 = dtb ptr

        li      a0, 24                  # number of bytes
        la      a1, test_msg            # lower address
        li      a2, 0                   # upper address
        call    sbi_debug_console_write

        la      sp, _stack0
        mv      a0, s0
        mv      a1, s1
        call    main

panic:
        j       panic
