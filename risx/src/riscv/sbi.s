.section .text

.globl sbi_get_spec_version
sbi_get_spec_version:
	li a6, 0
	li a7, 16
	ecall
	ret

.globl sbi_get_impl_id
sbi_get_impl_id:
	li a6, 1
	li a7, 16
	ecall
	ret

.globl sbi_get_impl_version
sbi_get_impl_version:
	li a6, 2
	li a7, 16
	ecall
	ret

.globl sbi_probe_extension
sbi_probe_extension:
	li a6, 3
	li a7, 16
	ecall
	ret

.globl sbi_get_mvendorid
sbi_get_mvendorid:
	li a6, 4
	li a7, 16
	ecall
	ret

.globl sbi_get_marchid
sbi_get_marchid:
	li a6, 5
	li a7, 16
	ecall
	ret

.globl sbi_get_mimpid
sbi_get_mimpid:
	li a6, 6
	li a7, 16
	ecall
	ret

.globl sbi_set_timer
sbi_set_timer:
	li a6, 0
	li a7, 1414090053
	ecall
	ret

.globl sbi_send_ipi
sbi_send_ipi:
	li a6, 0
	li a7, 7557193
	ecall
	ret

.globl sbi_remote_fence_i
sbi_remote_fence_i:
	li a6, 0
	li a7, 1380339267
	ecall
	ret

.globl sbi_remote_sfence_vma
sbi_remote_sfence_vma:
	li a6, 1
	li a7, 1380339267
	ecall
	ret

.globl sbi_remote_sfence_vma_asid
sbi_remote_sfence_vma_asid:
	li a6, 2
	li a7, 1380339267
	ecall
	ret

.globl sbi_remote_hfence_gvma_vmid
sbi_remote_hfence_gvma_vmid:
	li a6, 3
	li a7, 1380339267
	ecall
	ret

.globl sbi_remote_hfence_gvma
sbi_remote_hfence_gvma:
	li a6, 4
	li a7, 1380339267
	ecall
	ret

.globl sbi_remote_hfence_vvma_asid
sbi_remote_hfence_vvma_asid:
	li a6, 5
	li a7, 1380339267
	ecall
	ret

.globl sbi_remote_hfence_vvma
sbi_remote_hfence_vvma:
	li a6, 6
	li a7, 1380339267
	ecall
	ret

.globl sbi_hart_start
sbi_hart_start:
	li a6, 0
	li a7, 4739917
	ecall
	ret

.globl sbi_hart_stop
sbi_hart_stop:
	li a6, 1
	li a7, 4739917
	ecall
	ret

.globl sbi_hart_get_status
sbi_hart_get_status:
	li a6, 2
	li a7, 4739917
	ecall
	ret

.globl sbi_hart_suspend
sbi_hart_suspend:
	li a6, 3
	li a7, 4739917
	ecall
	ret

.globl sbi_system_reset
sbi_system_reset:
	li a6, 0
	li a7, 1397904212
	ecall
	ret

.globl sbi_debug_console_write
sbi_debug_console_write:
	li a6, 0
	li a7, 1145193294
	ecall
	ret

.globl sbi_debug_console_read
sbi_debug_console_read:
	li a6, 1
	li a7, 1145193294
	ecall
	ret

.globl sbi_debug_console_write_byte
sbi_debug_console_write_byte:
	li a6, 2
	li a7, 1145193294
	ecall
	ret

.globl sbi_system_suspend
sbi_system_suspend:
	li a6, 0
	li a7, 1398100816
	ecall
	ret

