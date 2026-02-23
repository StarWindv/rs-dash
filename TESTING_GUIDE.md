# rs-dash 测试框架指南

## 概述

本文档描述了rs-dash项目的测试框架结构和最佳实践。测试框架遵循Rust标准测试实践，并提供了跨平台支持。

## 测试结构

### 1. 模块测试 (Unit Tests)
位置: `src/module_tests.rs`
目的: 测试内部函数和模块
特点:
- 测试单个函数或小模块
- 不依赖外部命令
- 快速执行

### 2. 集成测试 (Integration Tests)
位置: `tests/` 目录
目的: 测试二进制文件的整体行为
特点:
- 测试完整的命令行接口
- 可能依赖外部命令
- 测试用户可见的行为

### 3. 跨平台测试 (Cross-Platform Tests)
位置: `tests/cross_platform.rs`
目的: 确保在Windows和Linux上都能正常工作
特点:
- 使用条件编译 (`#[cfg(windows)]`)
- 处理平台差异
- 测试路径分隔符、命令别名等

## 测试工具

### 核心工具
1. **assert_cmd**: 执行和断言命令行工具
2. **predicates**: 灵活的断言库
3. **tempfile**: 临时文件管理
4. **serial_test**: 串行测试执行

### 辅助工具
1. **which**: 检查命令是否可用
2. **std::env**: 环境变量管理
3. **std::path::Path**: 跨平台路径处理

## 编写测试的最佳实践

### 1. 测试命名
- 使用描述性名称: `test_功能_场景_期望`
- 示例: `test_echo_basic_output`, `test_variable_expansion_with_braces`

### 2. 测试组织
```rust
mod feature_tests {
    use super::*;
    
    #[test]
    fn test_basic_case() { ... }
    
    #[test]
    fn test_edge_case() { ... }
}
```

### 3. 跨平台处理
```rust
#[test]
fn test_platform_specific() {
    #[cfg(windows)]
    {
        // Windows-specific test code
    }
    
    #[cfg(not(windows))]
    {
        // Unix-specific test code
    }
}
```

### 4. 命令可用性检查
```rust
#[test]
fn test_external_command() {
    if !command_available("grep") {
        panic!("Skipping test: grep not available");
    }
    // Test code using grep
}
```

### 5. 临时文件管理
```rust
#[test]
fn test_with_temp_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "content").unwrap();
    
    // Use test_file in test
}
```

## 测试分类

### A. 基本功能测试
- 命令执行
- 变量扩展
- 命令替换
- 管道

### B. 控制结构测试
- if/else语句
- for/while循环
- 函数定义和调用

### C. 错误处理测试
- 语法错误
- 命令未找到
- 权限错误

### D. 性能测试
- 执行速度
- 内存使用
- 并发测试

### E. 兼容性测试
- 与原生dash比较
- 跨平台一致性
- 向后兼容性

## 运行测试

### 运行所有测试
```bash
cargo test
```

### 运行特定测试
```bash
cargo test test_echo_basic
cargo test --test basic
```

### 运行测试并显示输出
```bash
cargo test -- --nocapture
```

### 运行测试并生成覆盖率报告
```bash
cargo tarpaulin --ignore-tests
```

## CI/CD集成

### GitHub Actions示例
```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v2
    
    - name: Install Rust
      run: rustup update stable
    
    - name: Run tests
      run: cargo test --verbose
```

## 故障排除

### 常见问题

1. **测试在Windows上失败**
   - 检查路径分隔符
   - 检查命令别名
   - 检查环境变量大小写

2. **外部命令不可用**
   - 使用`which::which()`检查
   - 提供跳过机制
   - 使用模拟命令

3. **测试间干扰**
   - 使用`serial_test` crate
   - 避免共享状态
   - 使用临时目录

4. **性能测试不稳定**
   - 使用统计方法
   - 设置合理的超时
   - 考虑系统负载

## 贡献指南

### 添加新测试
1. 确定测试类型（单元/集成/跨平台）
2. 选择适当的测试文件
3. 遵循命名约定
4. 添加必要的平台检查
5. 确保测试可重复

### 修改现有测试
1. 确保不破坏现有功能
2. 更新相关文档
3. 运行所有测试验证

### 测试审查
1. 检查测试覆盖率
2. 验证跨平台兼容性
3. 确保测试独立性
4. 验证错误处理

## 参考资源

- [Rust测试指南](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [assert_cmd文档](https://docs.rs/assert_cmd/)
- [predicates文档](https://docs.rs/predicates/)
- [跨平台Rust开发](https://rust-lang.github.io/api-guidelines/interoperability.html)

## 附录

### 平台特定命令映射

| Unix命令 | Windows等价命令 | 测试中的函数 |
|---------|----------------|------------|
| `cat`   | `type`         | `cat_command()` |
| `grep`  | `findstr`      | `grep_command()` |
| `ls`    | `dir`          | `ls_command()` |

### 环境变量处理

| 变量 | Unix | Windows | 测试处理 |
|-----|------|---------|---------|
| PATH | `:`分隔 | `;`分隔 | 自动处理 |
| HOME | `$HOME` | `%USERPROFILE%` | 优先使用HOME，回退到USERPROFILE |
| TMPDIR | `/tmp` | `%TEMP%` | 使用`std::env::temp_dir()` |

### 路径处理示例
```rust
use std::path::Path;

// 跨平台路径连接
let path = Path::new("dir").join("file.txt");

// 路径分隔符
let separator = if cfg!(windows) { ';' } else { ':' };
```