//! 数值处理工具模块
//! 提供数值格式化和转换功能

use rust_decimal::prelude::*;

/// 将数值转换为指定小数位数的f32格式
///
/// # 参数
/// * `value` - 需要格式化的数值，可以是f32、f64或整数类型
/// * `decimals` - 保留的小数位数
///
/// # 返回值
/// 格式化后的f32数值
///
/// # 示例
/// use punicore_utils::utils::number::format_to_decimal;
/// let result = format_to_decimal(3.14159f64, 2);
///
pub fn format_to_f32<T>(value: T, decimals: u32) -> f32
where
    T: Into<f64>,
{
    let decimal_value = Decimal::from_f64(value.into()).unwrap_or(Decimal::ZERO);
    let rounded = decimal_value.round_dp(decimals);
    rounded.to_f32().unwrap_or(0.0)
}

/// 将数值转换为指定小数位数的f64格式
///
/// # 参数
/// * `value` - 需要格式化的数值
/// * `decimals` - 保留的小数位数
///
/// # 返回值
/// 格式化后的f64数值
///
/// # 示例
/// use punicore_utils::utils::number::format_to_decimal_f64;  
/// let result = format_to_decimal_f64(3.14159f64, 2);
///
pub fn format_to_f64<T>(value: T, decimals: u32) -> f64
where
    T: Into<f64>,
{
    let decimal_value = Decimal::from_f64(value.into()).unwrap_or(Decimal::ZERO);
    let rounded = decimal_value.round_dp(decimals);
    rounded.to_f64().unwrap_or(0.0)
}
