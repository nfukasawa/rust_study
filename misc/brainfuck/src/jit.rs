use super::operations::Op;
use std::io;
use std::mem;

use cranelift::prelude::*;
use cranelift_module::{default_libcall_names, DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};

const MEM_SIZE: usize = 65535;

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
                fn readbyte<R: io::Read>(input: *mut R) -> u8 {
                    let mut buf = [0; 1];
                    unsafe { (*input).read(&mut buf).unwrap() };
                    buf[0]
                }
                builder.symbol("input", input_ptr as *const u8);
                builder.symbol("readbyte", readbyte::<R> as *const u8);
            }

            {
                let output_ptr: *mut W = &mut *(*output);
                fn writebyte<W: io::Write>(output: *mut W, ch: u8) {
                    unsafe { (*output).write(&[ch]).unwrap() };
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
        self.data_ctx.define_zeroinit(MEM_SIZE as usize);
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
            sig.returns.push(AbiParam::new(types::I8));
            let callee = self
                .module
                .declare_function("readbyte", Linkage::Import, &sig)
                .unwrap();
            self.module.declare_func_in_func(callee, &mut builder.func)
        };

        let writebyte = {
            let mut sig = self.module.make_signature();
            sig.params.push(AbiParam::new(types::I64));
            sig.params.push(AbiParam::new(types::I8));
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
        let ptr = Variable::new(MEM_SIZE / 2 + 1);
        builder.declare_var(ptr, pointer_type);
        builder.def_var(ptr, zero);

        let mut translator = FunctionTranslator {
            builder,
            input,
            output,
            readbyte,
            writebyte,
            mem,
            ptr,
        };
        translator.translate(ops);

        translator.builder.ins().return_(&[]);
        translator.builder.finalize();
        Ok(())
    }
}

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    input: Value,
    output: Value,
    readbyte: codegen::ir::entities::FuncRef,
    writebyte: codegen::ir::entities::FuncRef,
    mem: Value,
    ptr: Variable,
}

impl<'a> FunctionTranslator<'a> {
    fn translate(&mut self, ops: &[Op]) {
        for op in ops {
            match op {
                Op::MovPtr(n) => {
                    let p = self.ptr();
                    let v = self.add_imm(p, *n as i64);
                    self.builder.def_var(self.ptr, v);
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
                    let v = self.readbyte();
                    let p = self.addr();
                    self.store(v, p, *offset);
                }
                Op::LoopBegin(p) => {}
                Op::LoopEnd(p) => {}
                Op::ClearVal(offset) => {}
                Op::MoveMulVal(offset, n, mul) => {}
                Op::MoveMulValN(offset, params) => {}
                Op::SkipToZero(n) => {}
            }
        }
    }

    #[inline]
    fn ptr(&mut self) -> Value {
        self.builder.use_var(self.ptr)
    }

    #[inline]
    fn addr(&mut self) -> Value {
        let p = self.ptr();
        self.builder.ins().iadd(self.mem, p)
    }

    #[inline]
    fn add_imm(&mut self, v: Value, imm: i64) -> Value {
        self.builder.ins().iadd_imm(v, imm)
    }

    #[inline]
    fn load(&mut self, ptr: Value, offset: isize) -> Value {
        self.builder
            .ins()
            .load(types::I8, MemFlags::new(), ptr, offset as i32)
    }

    #[inline]
    fn store(&mut self, val: Value, ptr: Value, offset: isize) {
        self.builder
            .ins()
            .store(MemFlags::new(), val, ptr, offset as i32);
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
}
