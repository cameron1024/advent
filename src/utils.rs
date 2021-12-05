#[macro_export]
macro_rules! input_const {
    ($path:literal) => {{
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/input", $path))
    }};
}

#[macro_export]
macro_rules! input_lines {
    ($path:literal) => {{
        $crate::input_const!($path)
            .lines()
    }};
}

#[cfg(test)]
const CHECK_INPUT_CONST: &str = input_const!("1");

