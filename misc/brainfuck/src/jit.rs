use super::operations::Op;
use std::io;
use std::mem;

use cranelift::prelude::*;
use cranelift_module::{default_libcall_names, DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};

const DATA_SIZE: usize = 65535;

macro_rules! mem_size {
    ( $s:expr ) => {
        4 * $s
    };
}

pub struct JIT {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: Module<SimpleJITBackend>,
}

impl JIT {
    pub fn new<R: io::Read, W: io::Write>(input: &mut Box<R>, output: &mut Box<W>) -> Self {
        if cfg!(windows) {
            unimplemented!();
        }

        let module = {
            let mut builder = SimpleJITBuilder::new(default_libcall_names());
            {
                let input_ptr: *mut R = &mut *(*input);
                fn readbyte<R: io::Read>(input: *mut R) -> i32 {
                    let mut buf = [0; 1];
                    unsafe { (*input).read(&mut buf).unwrap() };
                    buf[0] as i32
                }
                builder.symbol("input", input_ptr as *const u8);
                builder.symbol("readbyte", readbyte::<R> as *const u8);
            }

            {
                let output_ptr: *mut W = &mut *(*output);
                fn writebyte<W: io::Write>(output: *mut W, ch: i32) {
                    unsafe { (*output).write(&[ch as u8]).unwrap() };
                }
                builder.symbol("output", output_ptr as *const u8);
                builder.symbol("writebyte", writebyte::<W> as *const u8);
            }

            Module::new(builder)
        };

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }

    pub fn exec(&mut self, ops: &[Op]) {
        let main = self.compile(ops).unwrap();
        let main = unsafe { mem::transmute::<_, fn() -> ()>(main) };
        main();
    }

    fn compile(&mut self, ops: &[Op]) -> Result<*const u8, String> {
        self.initialize_memory();
        self.translate(&ops).unwrap();

        let main = self
            .module
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .unwrap();
        self.module.define_function(main, &mut self.ctx).unwrap();
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();
        let code = self.module.get_finalized_function(main);
        Ok(code)
    }

    fn initialize_memory(&mut self) {
        self.data_ctx.define_zeroinit(mem_size!(DATA_SIZE) as usize);
        let id = self
            .module
            .declare_data("mem", Linkage::Export, true, None)
            .unwrap();
        self.module.define_data(id, &self.data_ctx).unwrap();
        self.data_ctx.clear();
        self.module.finalize_definitions();
    }

    fn translate(&mut self, ops: &[Op]) -> Result<(), String> {
        let pointer_type = self.module.target_config().pointer_type();

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_ebb = builder.create_ebb();
        builder.switch_to_block(entry_ebb);
        builder.seal_block(entry_ebb);

        let input = {
            let sym = self
                .module
                .declare_data("input", Linkage::Import, true, None)
                .unwrap();
            let id = self.module.declare_data_in_func(sym, &mut builder.func);
            builder.ins().symbol_value(pointer_type, id)
        };

        let output = {
            let sym = self
                .module
                .declare_data("output", Linkage::Import, true, None)
                .unwrap();
            let id = self.module.declare_data_in_func(sym, &mut builder.func);
            builder.ins().symbol_value(pointer_type, id)
        };

        let readbyte = {
            let mut sig = self.module.make_signature();
            sig.params.push(AbiParam::new(types::I64));
            sig.returns.push(AbiParam::new(types::I32));
            let callee = self
                .module
                .declare_function("readbyte", Linkage::Import, &sig)
                .unwrap();
            self.module.declare_func_in_func(callee, &mut builder.func)
        };

        let writebyte = {
            let mut sig = self.module.make_signature();
            sig.params.push(AbiParam::new(types::I64));
            sig.params.push(AbiParam::new(types::I32));
            let callee = self
                .module
                .declare_function("writebyte", Linkage::Import, &sig)
                .unwrap();
            self.module.declare_func_in_func(callee, &mut builder.func)
        };

        let mem = {
            let sym = self
                .module
                .declare_data("mem", Linkage::Export, true, None)
                .unwrap();
            let id = self.module.declare_data_in_func(sym, &mut builder.func);
            builder.ins().symbol_value(pointer_type, id)
        };

        let zero = builder.ins().iconst(pointer_type, 0);
        let ptr = Variable::new(mem_size!(DATA_SIZE / 2 + 1));
        builder.declare_var(ptr, pointer_type);
        builder.def_var(ptr, zero);

        let mut translator =
            FunctionTranslator::new(builder, mem, ptr, input, output, readbyte, writebyte);
        translator.translate(ops);
        translator.builder.ins().return_(&[]);
        translator.builder.finalize();
        Ok(())
    }
}

type FFICallback = codegen::ir::entities::FuncRef;

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,

    mem: Value,
    ptr: Variable,

    input: Value,
    output: Value,
    readbyte: FFICallback,
    writebyte: FFICallback,

    loop_stack: Vec<(Ebb, Ebb)>,
}

impl<'a> FunctionTranslator<'a> {
    fn new(
        builder: FunctionBuilder<'a>,
        mem: Value,
        ptr: Variable,
        input: Value,
        output: Value,
        readbyte: FFICallback,
        writebyte: FFICallback,
    ) -> Self {
        Self {
            builder,
            mem,
            ptr,
            input,
            output,
            readbyte,
            writebyte,
            loop_stack: Vec::new(),
        }
    }

    fn translate(&mut self, ops: &[Op]) {
        for op in ops {
            match op {
                Op::MovPtr(n) => {
                    let p = self.ptr();
                    let p = self.ptr_offset(p, *n);
                    self.set_ptr(p);
                }
                Op::AddVal(offset, n) => {
                    let p = self.addr();
                    let v = self.load(p, *offset);
                    let v = self.add_imm(v, i64::from(*n));
                    self.store(v, p, *offset);
                }
                Op::WriteVal(offset) => {
                    let p = self.addr();
                    let v = self.load(p, *offset);
                    self.writebyte(v);
                }
                Op::ReadVal(offset) => {
                    let p = self.addr();
                    let v = self.readbyte();
                    self.store(v, p, *offset);
                }
                Op::LoopBegin(_) => {
                    let end_block = self.loop_begin();
                    let p = self.addr();
                    let v = self.load(p, 0);
                    self.branch_when_zero(v, end_block);
                }
                Op::LoopEnd(_) => {
                    self.loop_end();
                }
                Op::ClearVal(offset) => {
                    let p = self.addr();
                    let zero = self.const_val(0);
                    self.store(zero, p, *offset);
                }
                Op::MoveMulVal(offset, d, mul) => {
                    let p = self.addr();
                    let v = self.load(p, *offset);

                    let m = self.mul_imm(v, *mul as i64);
                    let to = self.ptr_offset(p, *d);
                    let x = self.load(to, *offset);
                    let x = self.add(x, m);
                    self.store(x, to, *offset);

                    let zero = self.const_val(0);
                    self.store(zero, p, *offset);
                }
                Op::MoveMulValN(offset, params) => {
                    let p = self.addr();
                    let v = self.load(p, *offset);
                    for (d, mul) in params {
                        let m = self.mul_imm(v, *mul as i64);
                        let to = self.ptr_offset(p, *d);
                        let x = self.load(to, *offset);
                        let x = self.add(x, m);
                        self.store(x, to, *offset);
                    }
                    let zero = self.const_val(0);
                    self.store(zero, p, *offset);
                }
                Op::SkipToZero(n) => {
                    let end_block = self.loop_begin();
                    let p = self.addr();
                    let v = self.load(p, 0);
                    self.branch_when_zero(v, end_block);
                    let p = self.ptr_offset(p, *n);
                    self.set_ptr(p);
                    self.loop_end();
                }
            }
        }
    }

    #[inline]
    fn ptr(&mut self) -> Value {
        self.builder.use_var(self.ptr)
    }

    #[inline]
    fn set_ptr(&mut self, p: Value) {
        self.builder.def_var(self.ptr, p);
    }

    #[inline]
    fn addr(&mut self) -> Value {
        let p = self.ptr();
        self.add(self.mem, p)
    }

    #[inline]
    fn ptr_offset(&mut self, p: Value, offset: isize) -> Value {
        self.add_imm(p, mem_size!(offset) as i64)
    }

    #[inline]
    fn const_val(&mut self, v: i64) -> Value {
        self.builder.ins().iconst(types::I32, v)
    }

    #[inline]
    fn add(&mut self, v1: Value, v2: Value) -> Value {
        self.builder.ins().iadd(v1, v2)
    }

    #[inline]
    fn add_imm(&mut self, v: Value, imm: i64) -> Value {
        self.builder.ins().iadd_imm(v, imm)
    }

    #[inline]
    fn mul_imm(&mut self, v: Value, imm: i64) -> Value {
        self.builder.ins().imul_imm(v, imm)
    }

    #[inline]
    fn load(&mut self, ptr: Value, offset: isize) -> Value {
        self.builder
            .ins()
            .load(types::I32, MemFlags::new(), ptr, mem_size!(offset) as i32)
    }

    #[inline]
    fn store(&mut self, val: Value, ptr: Value, offset: isize) {
        self.builder
            .ins()
            .store(MemFlags::new(), val, ptr, mem_size!(offset) as i32);
    }

    #[inline]
    fn writebyte(&mut self, v: Value) {
        self.builder.ins().call(self.writebyte, &[self.output, v]);
    }

    #[inline]
    fn readbyte(&mut self) -> Value {
        let call = self.builder.ins().call(self.readbyte, &[self.input]);
        self.builder.inst_results(call)[0]
    }

    #[inline]
    fn loop_begin(&mut self) -> Ebb {
        let begin = self.builder.create_ebb();
        let end = self.builder.create_ebb();
        self.builder.ins().jump(begin, &[]);
        self.builder.switch_to_block(begin);
        self.loop_stack.push((begin, end));
        end
    }

    #[inline]
    fn loop_end(&mut self) {
        if let Some((begin, end)) = self.loop_stack.pop() {
            self.builder.ins().jump(begin, &[]);
            self.builder.switch_to_block(end);
            self.builder.seal_block(begin);
            self.builder.seal_block(end);
        } else {
            panic!("loop begin not found");
        }
    }

    #[inline]
    fn branch_when_zero(&mut self, v: Value, block: Ebb) {
        self.builder.ins().brz(v, block, &[]);
    }
}
