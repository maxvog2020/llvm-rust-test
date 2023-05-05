mod codegen;
use codegen::CodeGen;
use codegen::SumFunc;

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
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

    let sum: JitFunction<SumFunc> = codegen.jit_compile_sum().unwrap();

    let (x, y, z) = (1, 2, 3);

    unsafe {
        assert_eq!(sum.call(x, y, z), x + y + z);
    }
}
