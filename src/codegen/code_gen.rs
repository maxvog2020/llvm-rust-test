use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::OptimizationLevel;

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

