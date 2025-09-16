// use super::SbiRet;

// unsafe extern "C" {
//     fn sbi_set_timer(stime_value: u64) -> SbiRet;
// }

// pub fn set_timer(stime_value: u64) -> Result<(), i32> {
//     let result: SbiRet;
//     unsafe {
//         result = sbi_set_timer(stime_value);
//     }

//     if result.error == 0 {
//         Ok(())
//     } else {
//         Err(result.error)
//     }
// }
