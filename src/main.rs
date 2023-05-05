mod codegen;
use codegen::CodeGen;
use codegen::JitCompileSum;

use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn print_sum_code(opt_level: OptimizationLevel) {
    let context = Context::create();
    let codegen = CodeGen::new(&context, opt_level);

    codegen.jit_compile_sum();
    print!("{}\n\n", codegen.module.print_to_string().to_string());
}

fn main() {
    print_sum_code(OptimizationLevel::None);
    print_sum_code(OptimizationLevel::Less);
    print_sum_code(OptimizationLevel::Aggressive);
}

