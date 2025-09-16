// use super::SbiRet;

// unsafe extern "C" {
//     pub fn sbi_remote_fence_i(hart_mask: u32, hart_mask_base: u32) -> SbiRet;
//     pub fn sbi_remote_sfence_vma(
//         hart_mask: u32,
//         hart_mask_base: u32,
//         start_addr: u32,
//         size: u32,
//     ) -> SbiRet;
//     pub fn sbi_remote_sfence_vma_asid(
//         hart_mask: u32,
//         hart_mask_base: u32,
//         start_addr: u32,
//         size: u32,
//         asid: u32,
//     ) -> SbiRet;
//     pub fn sbi_remote_hfence_gvma_vmid(
//         hart_mask: u32,
//         hart_mask_base: u32,
//         start_addr: u32,
//         size: u32,
//         vmid: u32,
//     ) -> SbiRet;
//     pub fn sbi_remote_hfence_gvma(
//         hart_mask: u32,
//         hart_mask_base: u32,
//         start_addr: u32,
//         size: u32,
//     ) -> SbiRet;
//     pub fn sbi_remote_hfence_vvma_asid(
//         hart_mask: u32,
//         hart_mask_base: u32,
//         start_addr: u32,
//         size: u32,
//         asid: u32,
//     ) -> SbiRet;
//     pub fn sbi_remote_hfence_vvma(
//         hart_mask: u32,
//         hart_mask_base: u32,
//         start_addr: u32,
//         size: u32,
//     ) -> SbiRet;
// }

// // pub fn fence_i() -> Result<{
// //     unsafe {
// //         sbi_remote_fence_i(0, 0);
// //     }
// // }
