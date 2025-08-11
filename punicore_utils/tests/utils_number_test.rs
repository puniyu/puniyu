use punicore_utils::utils::number::{format_to_f32, format_to_f64};
#[test]
fn format_to_f32_test() {
    // 测试f64转f32并保留2位小数
    assert_eq!(format_to_f32(std::f64::consts::PI, 2), 3.14f32);
    assert_eq!(format_to_f32(2.1f64, 2), 2.10f32);
    assert_eq!(format_to_f32(5.0f64, 2), 5.00f32);

    // 测试f32输入
    assert_eq!(format_to_f32(std::f32::consts::PI, 2), 3.14f32);

    // 测试整数输入
    assert_eq!(format_to_f32(100u64 as f32, 2), 100.00f32);

    // 测试不同小数位数
    assert_eq!(format_to_f32(std::f64::consts::PI, 1), 3.1f32);
    let lhs = format_to_f32(std::f64::consts::PI, 3);
    let rhs = std::f32::consts::PI;
    assert!((lhs - rhs).abs() < 0.001);
}

#[test]
fn format_to_f64_test() {
    // 测试f64格式化
    assert_eq!(format_to_f64(std::f64::consts::PI, 2), 3.14f64);
    assert_eq!(format_to_f64(2.1f64, 2), 2.10f64);
    assert_eq!(format_to_f64(5.0f64, 2), 5.00f64);
}
