//! Tests for the HiPO interior-point solver, only compiled when the `hipo`
//! feature is enabled (i.e. HiGHS was built with `-DHIPO=ON`).
#![cfg(feature = "hipo")]

use std::convert::TryInto;
use std::ffi::CString;

use highs_sys::*;

fn c(n: usize) -> HighsInt {
    n.try_into().unwrap()
}

fn ptr<T>(a: &mut [T]) -> *mut T {
    a.as_mut_ptr()
}

/// Solve a small LP with the HiPO interior-point solver and check the optimum.
///
/// This exercises the whole HiPO path: selecting `solver = "hipo"` (only a
/// valid option value when HiGHS was compiled with HIPO) and running the BLAS
/// backend that `build.rs` linked against.
#[test]
fn hipo_solves_lp() {
    unsafe {
        // Max    f  = 2x_0 + 3x_1
        // s.t.                x_1 <= 6
        //       10 <=  x_0 + 2x_1 <= 14
        //        8 <= 2x_0 +  x_1
        // 0 <= x_0 <= 3; 1 <= x_1
        // Optimum: (x_0, x_1) = (3, 5.5), objective 22.5.
        let highs = Highs_create();

        let opt = CString::new("output_flag").unwrap();
        Highs_setBoolOptionValue(highs, opt.as_ptr(), 0);

        // Select the HiPO interior-point solver.
        let solver_opt = CString::new("solver").unwrap();
        let solver_val = CString::new("hipo").unwrap();
        let status =
            Highs_setStringOptionValue(highs, solver_opt.as_ptr(), solver_val.as_ptr());
        assert_eq!(status, STATUS_OK, "selecting the hipo solver should succeed");

        let inf = Highs_getInfinity(highs);

        let colcost: &mut [f64] = &mut [2.0, 3.0];
        let collower: &mut [f64] = &mut [0.0, 1.0];
        let colupper: &mut [f64] = &mut [3.0, inf];
        let rowlower: &mut [f64] = &mut [-inf, 10.0, 8.0];
        let rowupper: &mut [f64] = &mut [6.0, 14.0, inf];

        // Constraint matrix, row-wise.
        let arstart: &mut [HighsInt] = &mut [0, 1, 3];
        let arindex: &mut [HighsInt] = &mut [1, 0, 1, 0, 1];
        let arvalue: &mut [f64] = &mut [1.0, 1.0, 2.0, 2.0, 1.0];

        use std::ptr::null;
        assert_eq!(
            STATUS_OK,
            Highs_addCols(
                highs,
                c(2),
                ptr(colcost),
                ptr(collower),
                ptr(colupper),
                0,
                null(),
                null(),
                null(),
            ),
            "addCols"
        );
        assert_eq!(
            STATUS_OK,
            Highs_addRows(
                highs,
                c(3),
                ptr(rowlower),
                ptr(rowupper),
                c(5),
                ptr(arstart),
                ptr(arindex),
                ptr(arvalue),
            ),
            "addRows"
        );
        assert_eq!(
            STATUS_OK,
            Highs_changeObjectiveSense(highs, OBJECTIVE_SENSE_MAXIMIZE)
        );

        assert_eq!(Highs_run(highs), STATUS_OK, "run");
        assert_eq!(
            Highs_getModelStatus(highs),
            MODEL_STATUS_OPTIMAL,
            "hipo should reach an optimal solution"
        );

        let mut objective: f64 = 0.0;
        let info_name = CString::new("objective_function_value").unwrap();
        Highs_getHighsDoubleInfoValue(highs, info_name.as_ptr(), &mut objective as *mut f64);
        // Interior-point solvers converge to the optimum only to a tolerance.
        assert!(
            (objective - 22.5).abs() < 1e-5,
            "objective = {}, expected 22.5",
            objective
        );

        let colvalue: &mut [f64] = &mut vec![0.; 2];
        let coldual: &mut [f64] = &mut vec![0.; 2];
        let rowvalue: &mut [f64] = &mut vec![0.; 3];
        let rowdual: &mut [f64] = &mut vec![0.; 3];
        Highs_getSolution(
            highs,
            ptr(colvalue),
            ptr(coldual),
            ptr(rowvalue),
            ptr(rowdual),
        );
        assert!((colvalue[0] - 3.0).abs() < 1e-5, "x_0 = {}", colvalue[0]);
        assert!((colvalue[1] - 5.5).abs() < 1e-5, "x_1 = {}", colvalue[1]);

        Highs_destroy(highs);
    }
}
