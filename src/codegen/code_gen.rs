use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use super::jit_compile_sum::{SumFunc, JitCompileSum};

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, opt_level: OptimizationLevel) -> Self {
        let module = context.create_module("sum");
        let execution_engine = module
            .create_jit_execution_engine(opt_level)
            .unwrap();

        CodeGen {
            context,
            module,
            builder: context.create_builder(),
            execution_engine,
        }
    }
}

impl<'ctx> JitCompileSum<'ctx> for CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Option<JitFunction<'ctx, SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        unsafe { self.execution_engine.get_function("sum").ok() }
    }
}

