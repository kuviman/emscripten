use ::*;

pub fn set_element_css_size(target: Selector, size: (f64, f64)) -> HtmlResult<()> {
    let result = unsafe {
        emscripten_set_element_css_size(
            selector_as_ptr!(target),
            size.0 as c_double,
            size.0 as c_double,
        )
    };
    match parse_html_result(result) {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn get_element_css_size(target: Selector) -> HtmlResult<(f64, f64)> {
    let mut width: c_double = unsafe { mem::uninitialized() };
    let mut height: c_double = unsafe { mem::uninitialized() };
    let result = unsafe {
        emscripten_get_element_css_size(selector_as_ptr!(target), &mut width, &mut height)
    };
    match parse_html_result(result) {
        None => Ok((width as f64, height as f64)),
        Some(err) => Err(err),
    }
}
