mod codegen;
use codegen::CodeGen;
use codegen::JitCompileSum;

use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() {
    let context = Context::create();
    let codegen = CodeGen::new(&context, OptimizationLevel::Aggressive);
    codegen.jit_compile_sum();
    codegen.module.print_to_stderr();
}

#[test]
fn test_sum_func_call() {
    let context = Context::create();
    let codegen = CodeGen::new(&context, OptimizationLevel::Aggressive);
    let sum = codegen.jit_compile_sum().unwrap();

    let collections = [
        (1, 2, 3),
        (3, 4, 2),
        (4, 2, 1),
        (1, 1, 1),
        (4, 0, 0),
        (0, 0, 2),
    ];

    for (x, y, z) in collections {
        unsafe { assert_eq!(sum.call(x, y, z), x + y + z); }
    }
}
