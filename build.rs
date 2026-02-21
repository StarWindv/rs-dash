use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 将包版本写入环境变量，供编译期使用
    println!("cargo:rustc-env=CARGO_PKG_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=CARGO_PKG_NAME={}", env!("CARGO_PKG_NAME"));
    println!("cargo:rustc-env=CARGO_PKG_DESCRIPTION={}", env!("CARGO_PKG_DESCRIPTION"));
    
    // 添加编译时间信息
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
}