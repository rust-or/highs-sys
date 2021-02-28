use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_int;

use highs_sys::*;

fn c(n: usize) -> c_int {
    n.try_into().unwrap()
}

fn ptr<T>(a: &mut [T]) -> *mut T {
    a.as_mut_ptr()
}

#[test]
fn highs_functions() {
    unsafe {
        // Form and solve the LP
        // Max    f  = 2x_0 + 3x_1
        // s.t.                x_1 <= 6
        //       10 <=  x_0 + 2x_1 <= 14
        //        8 <= 2x_0 +  x_1
        // 0 <= x_0 <= 3; 1 <= x_1

        let highs = Highs_create();

        let numcol: usize = 2;
        let numrow: usize = 3;
        let nnz: usize = 5;

        // Define the column costs, lower bounds and upper bounds
        let colcost: &mut [f64] = &mut [2.0, 3.0];
        let collower: &mut [f64] = &mut [0.0, 1.0];
        let colupper: &mut [f64] = &mut [3.0, 1.0e30];
        // Define the row lower bounds and upper bounds
        let rowlower: &mut [f64] = &mut [-1.0e30, 10.0, 8.0];
        let rowupper: &mut [f64] = &mut [6.0, 14.0, 1.0e30];

        // Define the constraint matrix row-wise, as it is added to the LP
        // with the rows
        let arstart: &mut [c_int] = &mut [0, 1, 3];
        let arindex: &mut [c_int] = &mut [1, 0, 1, 0, 1];
        let arvalue: &mut [f64] = &mut [1.0, 1.0, 2.0, 2.0, 1.0];

        use std::ptr::null;

        // Add two columns to the empty LP
        let success = Highs_addCols(
            highs,
            c(numcol),
            ptr(colcost),
            ptr(collower),
            ptr(colupper),
            0,
            null(),
            null(),
            null(),
        );
        assert_ne!(0, success, "addCols");
        // Add three rows to the 2-column LP
        let success = Highs_addRows(
            highs,
            c(numrow),
            ptr(rowlower),
            ptr(rowupper),
            c(nnz),
            ptr(arstart),
            ptr(arindex),
            ptr(arvalue),
        );
        assert_ne!(0, success, "addRows");

        // -1 = maximize
        Highs_changeObjectiveSense(highs, -1);

        let simplex_scale_strategy = 3;
        let option_name = CString::new("simplex_scale_strategy").unwrap();
        Highs_setHighsIntOptionValue(highs, option_name.as_ptr(), simplex_scale_strategy);

        // Solving the problem without printing to the standard output
        Highs_runQuiet(highs);
        let status = Highs_run(highs);
        assert_eq!(status, STATUS_OK);

        let model_status = Highs_getModelStatus(highs, 0);
        assert_eq!(model_status, MODEL_STATUS_OPTIMAL);

        let mut objective_function_value = 0.;
        let info_name = CString::new("objective_function_value").unwrap();
        Highs_getHighsDoubleInfoValue(
            highs,
            info_name.as_ptr(),
            (&mut objective_function_value) as *mut f64,
        );
        assert_eq!(objective_function_value, 2. * 3. + 3. * 5.5);

        let colvalue: &mut [f64] = &mut vec![0.; numcol];
        let coldual: &mut [f64] = &mut vec![0.; numcol];
        let rowvalue: &mut [f64] = &mut vec![0.; numrow];
        let rowdual: &mut [f64] = &mut vec![0.; numrow];

        // Get the primal and dual solution
        Highs_getSolution(
            highs,
            ptr(colvalue),
            ptr(coldual),
            ptr(rowvalue),
            ptr(rowdual),
        );
        assert_eq!(colvalue, &[3.0, 5.5]);

        let colbasisstatus: &mut [c_int] = &mut vec![0; numcol];
        let rowbasisstatus: &mut [c_int] = &mut vec![0; numrow];
        // Get the basis
        Highs_getBasis(highs, ptr(colbasisstatus), ptr(rowbasisstatus));
        assert_eq!(colbasisstatus, &[2, 1]);

        Highs_destroy(highs);
    }
}
