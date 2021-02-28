#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const MODEL_STATUS_NOTSET: c_int = 0;
pub const MODEL_STATUS_LOAD_ERROR: c_int = 1;
pub const MODEL_STATUS_MODEL_ERROR: c_int = 2;
pub const MODEL_STATUS_PRESOLVE_ERROR: c_int = 3;
pub const MODEL_STATUS_SOLVE_ERROR: c_int = 4;
pub const MODEL_STATUS_POSTSOLVE_ERROR: c_int = 5;
pub const MODEL_STATUS_MODEL_EMPTY: c_int = 6;
pub const MODEL_STATUS_PRIMAL_INFEASIBLE: c_int = 7;
pub const MODEL_STATUS_PRIMAL_UNBOUNDED: c_int = 8;
pub const MODEL_STATUS_OPTIMAL: c_int = 9;
pub const MODEL_STATUS_REACHED_DUAL_OBJECTIVE_VALUE_UPPER_BOUND: c_int = 10;
pub const MODEL_STATUS_REACHED_TIME_LIMIT: c_int = 11;
pub const MODEL_STATUS_REACHED_ITERATION_LIMIT: c_int = 12;
pub const MODEL_STATUS_PRIMAL_DUAL_INFEASIBLE: c_int = 13;
pub const MODEL_STATUS_DUAL_INFEASIBLE: c_int = 14;

pub const STATUS_OK: c_int = 0;
pub const STATUS_WARNING: c_int = 1;
pub const STATUS_ERROR: c_int = 2;


#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::ffi::CString;
    use std::os::raw::c_int;

    use super::*;

    #[test]
    fn highs_call() {
        // This illustrates the use of Highs_call, the simple C interface to
        // HiGHS. It's designed to solve the general LP problem
        //
        // Min c^Tx subject to L <= Ax <= U; l <= x <= u
        //
        // where A is a matrix with m rows and n columns
        //
        // The scalar n is numcol
        // The scalar m is numrow
        //
        // The vector c is colcost
        // The vector l is collower
        // The vector u is colupper
        // The vector L is rowlower
        // The vector U is rowupper
        //
        // The matrix A is represented in packed column-wise form: only its
        // nonzeros are stored
        //
        // * The number of nonzeros in A is nnz
        //
        // * The row indices of the nonnzeros in A are stored column-by-column
        // in aindex
        //
        // * The values of the nonnzeros in A are stored column-by-column in
        // avalue
        //
        // * The position in aindex/avalue of the index/value of the first
        // nonzero in each column is stored in astart
        //
        // Note that astart[0] must be zero
        //
        // After a successful call to Highs_call, the primal and dual
        // solution, and the simplex basis are returned as follows
        //
        // The vector x is colvalue
        // The vector Ax is rowvalue
        // The vector of dual values for the variables x is coldual
        // The vector of dual values for the variables Ax is rowdual
        // The basic/nonbasic status of the variables x is colbasisstatus
        // The basic/nonbasic status of the variables Ax is rowbasisstatus
        //
        // The status of the solution obtained is modelstatus
        //
        // To solve maximization problems, the values in c must be negated
        //
        // The use of Highs_call is illustrated for the LP
        //
        // Min    f  = 2x_0 + 3x_1
        // s.t.                x_1 <= 6
        //       10 <=  x_0 + 2x_1 <= 14
        //        8 <= 2x_0 +  x_1
        // 0 <= x_0 <= 3; 1 <= x_1

        let numcol: usize = 2;
        let numrow: usize = 3;
        let nnz: usize = 5;

        // Define the column costs, lower bounds and upper bounds
        let colcost: &[f64] = &[2.0, 3.0];
        let collower: &[f64] = &[0.0, 1.0];
        let colupper: &[f64] = &[3.0, 1.0e30];
        // Define the row lower bounds and upper bounds
        let rowlower: &[f64] = &[-1.0e30, 10.0, 8.0];
        let rowupper: &[f64] = &[6.0, 14.0, 1.0e30];
        // Define the constraint matrix column-wise
        let astart: &[c_int] = &[0, 2];
        let aindex: &[c_int] = &[1, 2, 0, 1, 2];
        let avalue: &[f64] = &[1.0, 2.0, 1.0, 2.0, 1.0];

        let colvalue: &mut [f64] = &mut vec![0.; numcol];
        let coldual: &mut [f64] = &mut vec![0.; numcol];
        let rowvalue: &mut [f64] = &mut vec![0.; numrow];
        let rowdual: &mut [f64] = &mut vec![0.; numrow];

        let colbasisstatus: &mut [c_int] = &mut vec![0; numcol];
        let rowbasisstatus: &mut [c_int] = &mut vec![0; numrow];

        let modelstatus: &mut c_int = &mut 0;

        let status: c_int = unsafe {
            Highs_call(
                numcol.try_into().unwrap(),
                numrow.try_into().unwrap(),
                nnz.try_into().unwrap(),
                colcost.as_ptr(),
                collower.as_ptr(),
                colupper.as_ptr(),
                rowlower.as_ptr(),
                rowupper.as_ptr(),
                astart.as_ptr(),
                aindex.as_ptr(),
                avalue.as_ptr(),
                colvalue.as_mut_ptr(),
                coldual.as_mut_ptr(),
                rowvalue.as_mut_ptr(),
                rowdual.as_mut_ptr(),
                colbasisstatus.as_mut_ptr(),
                rowbasisstatus.as_mut_ptr(),
                modelstatus,
            )
        };

        assert_eq!(status, STATUS_OK);
        assert_eq!(colvalue, &[2., 4.]);
    }

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
            let success = Highs_addCols(highs, c(numcol), ptr(colcost), ptr(collower), ptr(colupper), 0, null(), null(), null());
            assert_ne!(0, success, "addCols");
            // Add three rows to the 2-column LP
            let success = Highs_addRows(highs, c(numrow), ptr(rowlower), ptr(rowupper), c(nnz), ptr(arstart), ptr(arindex), ptr(arvalue));
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
            Highs_getHighsDoubleInfoValue(highs, info_name.as_ptr(), (&mut objective_function_value) as *mut f64);
            assert_eq!(objective_function_value, 2. * 3. + 3. * 5.5);

            let colvalue: &mut [f64] = &mut vec![0.; numcol];
            let coldual: &mut [f64] = &mut vec![0.; numcol];
            let rowvalue: &mut [f64] = &mut vec![0.; numrow];
            let rowdual: &mut [f64] = &mut vec![0.; numrow];


            // Get the primal and dual solution
            Highs_getSolution(highs, ptr(colvalue), ptr(coldual), ptr(rowvalue), ptr(rowdual));
            assert_eq!(colvalue, &[3.0, 5.5]);


            let colbasisstatus: &mut [c_int] = &mut vec![0; numcol];
            let rowbasisstatus: &mut [c_int] = &mut vec![0; numrow];
            // Get the basis
            Highs_getBasis(highs, ptr(colbasisstatus), ptr(rowbasisstatus));
            assert_eq!(colbasisstatus, &[2, 1]);

            Highs_destroy(highs);
        }
    }
}
