use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::err::Error;

pub fn load_csv(csv_file: PathBuf) -> Result<String, Error> {
    let content = read(csv_file)?;
    Ok(content)
}

pub fn write_csv(content: &str, filename: &str) -> Result<(), Error> {
    write(content, filename)?;
    Ok(())
}

fn read(path: PathBuf) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut input_file = open(path)?;
    input_file.read_to_string(&mut buffer)?;
    if buffer.is_empty() {
        return Err("input file missing")?; // ? 配合From 实现自动转换为定制错误Error
    }
    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn write(content: &str, filename: &str) -> Result<(), Error> {
    let mut output_file = File::create(filename)?;
    output_file.write_all(content.as_bytes())?;
    Ok(())
}

// 单元测试
// 条件编译：只有执行cargo test时才编译下面的模块
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::read::load_csv;

    #[test]
    fn test_valid_load_csv() {
        let filename = PathBuf::from("../csv_challenge/input/challenge.csv");
        let csv_data = load_csv(filename);
        assert!(csv_data.is_ok());
    }
}
