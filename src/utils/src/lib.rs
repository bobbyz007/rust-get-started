pub fn println_format(s: &str) {
    // :^ 中间对齐， = 等号填充， .18 截取18个字符
    println!("{}", format!("{:=^80.18}", format!(" {} ", s)));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
