use tvision::raw::TApplication;

fn main() {
    unsafe {
        let mut application = TApplication::new();
        tvision::raw::TProgram_run(&mut application as *mut _ as *mut _);
    }
}
