.section .rodata
test_msg:
        .ascii "Test message from risx!\n"

.section .text.entry
.globl _start

.equ _stack_size, 0x100000              # 1MB stack
_start:
        li      a0, 24                  # number of bytes
        la      a1, test_msg            # lower address
        li      a2, 0                   # upper address
        call    sbi_debug_console_write

        la      sp, _stack0
        call    main

panic:
        j       panic
