//@ compile-flags: -C opt-level=3  -Clto=fat  -Zllvm-plugins=/home/manuel/prog/rust-middle/build/x86_64-unknown-linux-gnu/enzyme/build/Enzyme/libEnzyme-19.so -Cpasses=enzyme
//@ no-prefer-dynamic
//@ needs-enzyme
#![feature(autodiff)]

use std::autodiff::autodiff;

#[autodiff(d_square, Reverse, Duplicated, Active)]
#[no_mangle]
fn square(x: &f64) -> f64 {
    x * x
}

// CHECK:define internal fastcc double @diffesquare(double %x.0.val, ptr nocapture align 8 %"x'"
// CHECK-NEXT:invertstart:
// CHECK-NEXT:  %_0 = fmul double %x.0.val, %x.0.val
// CHECK-NEXT:  %0 = fadd fast double %x.0.val, %x.0.val
// CHECK-NEXT:  %1 = load double, ptr %"x'", align 8, !alias.scope !17816, !noalias !17819
// CHECK-NEXT:  %2 = fadd fast double %1, %0
// CHECK-NEXT:  store double %2, ptr %"x'", align 8, !alias.scope !17816, !noalias !17819
// CHECK-NEXT:  ret double %_0
// CHECK-NEXT:}

fn main() {
    let x = 3.0;
    let output = square(&x);
    assert_eq!(9.0, output);

    let mut df_dx = 0.0;
    let output_ = d_square(&x, &mut df_dx, 1.0);
    assert_eq!(output, output_);
    assert_eq!(6.0, df_dx);
}
