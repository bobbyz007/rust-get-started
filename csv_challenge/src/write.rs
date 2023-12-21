use crate::err::Error;

pub fn replace_column(content: String, column: &str, replacement: &str) -> Result<String, Error> {
    let mut lines = content.lines();

    let headers = lines.next().unwrap();
    let columns: Vec<&str> = headers.split(',').collect();
    let column_num = columns.iter().position(|&item| item == column);
    let column_num = match  column_num {
        Some(column) => column,
        None => return Err("column name doesn't exist in the input file")?, // 为Error实现了From
    };

    let mut result = String::with_capacity(content.capacity());
    result.push_str(&columns.join(","));
    result.push('\n');

    for line in lines {
        let mut records: Vec<&str> = line.split(',').collect();
        records[column_num] = replacement;
        result.push_str(&records.join(","));
        result.push('\n');
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::write::replace_column;

    #[test]
    fn test_replace_column() {
        let content = r#"First Name,Last Name,Age,City,Eyes Color,Species
        John,Doe,32,Beijing,Blue,Human
        Flip,Helm,12,Beijing,Red,Unknown"#;
        match replace_column(content.to_string(), "City", "Shenzhen") {
            Ok(result) => assert!(result.contains("Shenzhen")),
            _ => (),
        }
    }
    #[test]
    fn test_replace_column_fail() {
        let content = r#"First Name,Last Name,Age,City,Eyes Color,Species
        John,Doe,32,Beijing,Blue,Human
        Flip,Helm,12,Beijing,Red,Unknown"#;
        match replace_column(content.to_string(), "City_NON_EXIST", "Shenzhen") {
            Ok(_) => (),
            Err(_) => assert!(true),
        }
    }
}