use inkwell::execution_engine::JitFunction;

pub type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

pub trait JitCompileSum<'s> {
    fn jit_compile_sum(&self) -> Option<JitFunction<'s, SumFunc>>;
}

