use ::*;

pub unsafe fn run_script(script: &str) {
    let script = CString::new(script).unwrap();
    emscripten_run_script(script.as_ptr());
}

pub unsafe fn run_script_i32(script: &str) -> i32 {
    let script = CString::new(script).unwrap();
    emscripten_run_script_int(script.as_ptr()) as i32
}

pub unsafe fn run_script_string(script: &str) -> String {
    let script = CString::new(script).unwrap();
    let string = {
        let string = emscripten_run_script_string(script.as_ptr());
        CStr::from_ptr(string)
    };
    String::from(string.to_str().expect("Script returned invalid UTF-8"))
}