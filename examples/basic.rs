use metal_irconverter::sys;

fn main() {
    unsafe {
        let compiler = sys::IRCompilerCreate();
        sys::IRCompilerDestroy(compiler);
    }
}