pub struct Test {
    pub name: &'static str,
    pub should_fail: bool,
    pub func: fn() -> Result<&'static str, &'static str>,
}

pub fn runner(tests: &[&Test]) {
    for test in tests {
        (test.func)().expect("FAIL");
    }
}
