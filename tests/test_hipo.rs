// A discovered system HiGHS may be built without HiPO.
#![cfg(all(feature = "hipo", not(feature = "discover")))]

use std::ffi::CString;

use highs_sys::*;

#[test]
fn hipo_solves_lp() {
    let name = |s: &str| CString::new(s).unwrap();
    unsafe {
        // Max    f  = 2x_0 + 3x_1
        // s.t.                x_1 <= 6
        //       10 <=  x_0 + 2x_1 <= 14
        //        8 <= 2x_0 +  x_1
        // 0 <= x_0 <= 3; 1 <= x_1
        let highs = Highs_create();
        Highs_setBoolOptionValue(highs, name("output_flag").as_ptr(), 0);

        // HiGHS rejects this value when it was built without HiPO.
        assert_eq!(
            Highs_setStringOptionValue(highs, name("solver").as_ptr(), name("hipo").as_ptr()),
            STATUS_OK
        );
        // Presolve could solve this small LP without calling HiPO.
        assert_eq!(
            Highs_setStringOptionValue(highs, name("presolve").as_ptr(), name("off").as_ptr()),
            STATUS_OK
        );

        let inf = Highs_getInfinity(highs);
        assert_eq!(
            Highs_passLp(
                highs,
                2,
                3,
                5,
                MATRIX_FORMAT_COLUMN_WISE,
                OBJECTIVE_SENSE_MAXIMIZE,
                0.0,
                [2.0, 3.0].as_ptr(),
                [0.0, 1.0].as_ptr(),
                [3.0, inf].as_ptr(),
                [-inf, 10.0, 8.0].as_ptr(),
                [6.0, 14.0, inf].as_ptr(),
                [0, 2].as_ptr(),
                [1, 2, 0, 1, 2].as_ptr(),
                [1.0, 2.0, 1.0, 2.0, 1.0].as_ptr(),
            ),
            STATUS_OK
        );

        assert_eq!(Highs_run(highs), STATUS_OK);
        assert_eq!(Highs_getModelStatus(highs), MODEL_STATUS_OPTIMAL);

        let mut ipm_iterations: HighsInt = 0;
        Highs_getIntInfoValue(
            highs,
            name("ipm_iteration_count").as_ptr(),
            &mut ipm_iterations,
        );
        assert!(ipm_iterations > 0, "HiPO did not run");

        let mut objective = 0.0;
        Highs_getDoubleInfoValue(
            highs,
            name("objective_function_value").as_ptr(),
            &mut objective,
        );
        assert!((objective - 22.5).abs() < 1e-6, "objective = {}", objective);

        Highs_destroy(highs);
    }
}
