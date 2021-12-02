
#[macro_export]
macro_rules! input_lines {
    () => {{
        use std::io::BufRead;
        let path = std::path::PathBuf::from(file!())
            .parent()
            .unwrap()
            .join("input");
        let file = std::fs::File::open(path).unwrap();
        std::io::BufReader::new(file)
            .lines()
            .map(Result::unwrap)
    }};
}

#[macro_export]
macro_rules! input_line_nums {
    () => {{
        crate::input_lines!()
            .map(|s| s.parse().unwrap())
            .collect()
    }};
}
