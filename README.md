# highs-sys

[![docs.rs badge](https://docs.rs/highs-sys/badge.svg)](https://docs.rs/highs-sys)
[![rust crate](https://img.shields.io/crates/v/highs-sys.svg)](https://lib.rs/highs-sys)

Rust binding for the HiGHS linear programming solver.
See http://highs.dev.

This repository contains the source for HiGHS itself as a submodule.
You should clone it with 

```
git clone --recursive git@github.com:rust-or/highs-sys.git
```

## Dependencies

This library depends on libstdc++ and libgomp at runtime.
In order to build, it requires cmake and a C++ compiler.

#### Install on debian

```
sudo apt-get install libstdc++6 libgomp1
```

(These are probably already installed on your system)

#### Install on MacOS

```
brew install libomp
```

HiGHS itself is built statically, so you don't need to install it
separately on the target system.

#### Install on Windows

You need to install [CMake](https://cmake.org/download/) and [Clang (available in LLVM)](https://releases.llvm.org/download.html).

They are available in [winget](https://winget.run/).

```powershell
winget install -e --id Kitware.CMake
winget install -e --id LLVM.LLVM
```

#### Feature Flags

`highs_release`: set CMake profile to "Release" regardless of build profile.
`libz`: enable HiGHS libz linking to enable support for reading 'mps.gz'.
`ninja`: set CMake generator to Ninja.

Windows users will likely need to install libz and set the ZLIB_ROOT
environment variable for CMake to locate and link with the library.

Ninja is available in [winget](https://winget.run/).

```powershell
winget install -e --id Ninja-build.Ninja
```

HiGHS itself is built statically, so you don't need to install it
separately on the target system.

## Example

```rust
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

fn main() {
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
    // Define the constraint matrix column-wise
    let astart: &mut [c_int] = &mut [0, 2];
    let aindex: &mut [c_int] = &mut [1, 2, 0, 1, 2];
    let avalue: &mut [f64] = &mut [1.0, 2.0, 1.0, 2.0, 1.0];

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
            colcost.as_mut_ptr(),
            collower.as_mut_ptr(),
            colupper.as_mut_ptr(),
            rowlower.as_mut_ptr(),
            rowupper.as_mut_ptr(),
            astart.as_mut_ptr(),
            aindex.as_mut_ptr(),
            avalue.as_mut_ptr(),
            colvalue.as_mut_ptr(),
            coldual.as_mut_ptr(),
            rowvalue.as_mut_ptr(),
            rowdual.as_mut_ptr(),
            colbasisstatus.as_mut_ptr(),
            rowbasisstatus.as_mut_ptr(),
            modelstatus
        )
    };

    assert_eq!(status, 0);
    // The solution is x_0 = 2 and x_1 = 4
    assert_eq!(colvalue, &[2., 4.]);
}
```

For more examples, have a look at [`tests`](https://github.com/lovasoa/highs-sys/blob/master/tests).
