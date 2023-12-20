// 高效处理 Option<T>，包括其unwrap， map， and_then方法

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use chrono::{NaiveDate};

pub fn err_handled_application() {
    match get_current_date_multiple_error() {
        Ok(date) => println!("We've time travelled to {}!!", date),
        Err(e) => {
            eprintln!("Oh noes, we don't know which era we're in! :(");
            if let Some(err) = e.downcast_ref::<reqwest::Error>() {
                eprintln!("Request Error: {}", err)
            } else if let Some(err) = e.downcast_ref::<chrono::format::ParseError>() {
                eprintln!("Parse Error: {}", err)
            }
        },
    }
}

#[allow(dead_code)]
fn get_current_date() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let result = reqwest::blocking::get(url);

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let body = response.json::<HashMap<String, i32>>();

    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };

    let date = json["years"].to_string();

    Ok(date)
}

// ? operator 用于Result和Option类型
#[allow(dead_code)]
fn get_current_date_with_question_mark_operator() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;
    let date = res["years"].to_string();

    Ok(date)
}

// 可能返回 chrono::format::ParseError 或 reqwest::Error，需要trait对象封箱包装
fn get_current_date_multiple_error() -> Result<String, Box<dyn Error>> {
    let url = "https://postman-echo.com/time/object";
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;

    let formatted_date = format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);
    //报错 chrono::format::ParseError couldn't convert the error to `reqwest::Error`
    let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y#-%m-%d")?; // deliberately make err
    let date = parsed_date.format("%Y %B %d").to_string();

    Ok(date)
}

// 在library中考虑定制错误
pub fn err_handled_library() {
    match get_current_date_custom_error() {
        Ok(date) => println!("We've time travelled to {}!!", date),
        Err(e) => {
            eprintln!("Oh noes, we don't know which era we're in! :(");
            match e {
                MyCustomError::HttpError(e) => eprintln!("Custom Request Error: {}", e),
                MyCustomError::ParseError(e) => eprintln!("Custom Parse Error: {}", e),
            }
        },
    }
}

// 定制错误要实现Error trait， 必须先实现Debug和Display，因为Error继承它们。
#[derive(Debug)]
pub enum MyCustomError {
    HttpError(reqwest::Error),
    ParseError(chrono::format::ParseError),
}
impl Display for MyCustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyCustomError::HttpError(e) => write!(f, "HTTP Error...{}", e),
            MyCustomError::ParseError(e) => write!(f, "Parse Error...{}", e),
        }
    }
}
impl Error for MyCustomError {}
// 配合 ？操作符 实现内部异常转换为 定制异常。 可以不使用map_err来简化代码
impl From<reqwest::Error> for MyCustomError {
    fn from(e: reqwest::Error) -> Self {
        MyCustomError::HttpError(e)
    }
}
impl From<chrono::format::ParseError> for MyCustomError {
    fn from(e: chrono::format::ParseError) -> Self {
        MyCustomError::ParseError(e)
    }
}

fn get_current_date_custom_error() -> Result<String, MyCustomError> {
    let url = "https://postman-echo.com/time/object";
    // 转化为定制异常
    /*let res = reqwest::blocking::get(url).map_err(|_| MyCustomError::HttpError)?
        .json::<HashMap<String, i32>>().map_err(|_| MyCustomError::HttpError)?;*/

    // 通过？ operator和 实现From trait，可以实现自动转换为定制异常
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;


    let formatted_date = format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);
    // 转化为定制异常
    /*let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y#-%m-%d") // deliberately make err
        .map_err(|_| MyCustomError::ParseError )?;*/

    // 通过？ operator和 实现From trait，可以实现自动转换为定制异常
    let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y#-%m-%d")?;
    let date = parsed_date.format("%Y %B %d").to_string();

    Ok(date)
}