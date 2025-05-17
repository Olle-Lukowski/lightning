pub struct Test {
    name: &'static str,
    should_fail: bool,
    func: fn() -> Result<&'static str, &'static str>,
}

pub fn runner(_tests: &[&Test]) {}
