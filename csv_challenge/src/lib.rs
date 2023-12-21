// 最佳实践： 对于二进制类型crate， 配合 tests/integraton_test 集成测试
// 导出对应的方法，供集成测试调用
mod err;
mod read;
mod write;

pub use read::{load_csv, write_csv};
pub use write::replace_column;