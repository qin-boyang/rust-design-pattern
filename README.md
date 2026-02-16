# Rust 设计模式实践

本项目展示了在 Rust 中实现经典设计模式的实际示例，帮助开发者理解如何在 Rust 的类型系统和所有权模型下应用这些模式。

## 📋 项目概述

这是一个展示 Rust 设计模式实现的学习项目，目前包含了停车场管理系统作为主要示例，演示了策略模式和工厂模式的实际应用。

## 🎯 已实现的设计模式

### 1. 策略模式 (Strategy Pattern)
**位置**: `src/parking/parking_strategy.rs`

实现了灵活的停车策略选择机制：

- **FirstAvailableStrategy**: 顺序停车策略 - 寻找第一个可用停车位
- **DistributedStrategy**: 分布式停车策略 - 从后往前寻找停车位

#### 核心组件
```rust
pub trait ParkingStrategy {
    fn find_space(&self, spaces: &[Option<String>]) -> Option<usize>;
}

pub struct ParkingLot {
    spaces: Vec<Option<String>>,
    strategy: Box<dyn ParkingStrategy>, // 运行时策略切换
}
```

### 2. 工厂模式 (Factory Pattern)
**位置**: `src/parking/parking_strategy_factory.rs`

提供了停车场实例的创建工厂：

- **EconomyParkingStrategy**: 经济型停车策略
- **LuxuryParkingStrategy**: 豪华型停车策略（VIP优先）

#### 工厂实现
```rust
struct MallParkingLotFactory;
impl MallParkingLotFactory {
    fn create(level: LotLevel, capacity: u64) -> ParkingLot {
        match level {
            LotLevel::Economy => ParkingLot::new(capacity, Box::new(EconomyParkingStrategy)),
            LotLevel::Luxury => ParkingLot::new(capacity, Box::new(LuxuryParkingStrategy)),
        }
    }
}
```

## 🚀 快速开始

### 环境要求
- Rust 1.70 或更高版本
- Cargo 包管理器

### 克隆和构建
```bash
git clone <repository-url>
cd rust-design-pattern
cargo build
```

### 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test parking
```

## 🔧 项目结构

```
src/
├── lib.rs                    # 库入口文件
└── parking/                  # 停车场模块
    ├── mod.rs               # 模块声明
    ├── parking_strategy.rs  # 策略模式实现
    └── parking_strategy_factory.rs  # 工厂模式实现
```

## 📖 使用示例

### 策略模式示例
```rust
use rust_design_pattern::parking::parking_strategy::*;

// 创建使用不同策略的停车场
let mut first_lot = ParkingLot::new(10, Box::new(FirstAvailableStrategy));
let mut dist_lot = ParkingLot::new(10, Box::new(DistributedStrategy));

// 停车行为会根据策略不同而变化
let ticket1 = first_lot.park("ABC123".to_string()); // 停在第一个空位
let ticket2 = dist_lot.park("XYZ789".to_string());   // 停在最后一个空位
```

### 工厂模式示例
```rust
use rust_design_pattern::parking::parking_strategy_factory::*;

// 使用工厂创建不同类型的停车场
let mut economy_lot = MallParkingLotFactory::create(LotLevel::Economy, 5);
let mut luxury_lot = MallParkingLotFactory::create(LotLevel::Luxury, 5);

// 不同类型停车场采用不同的停车策略
let eco_ticket = economy_lot.park("ECONOMY".to_string());
let lux_ticket = luxury_lot.park("LUXURY".to_string());
```

## 🧪 测试说明

项目包含完整的单元测试覆盖：

### 策略模式测试
- `it_should_use_first_available_strategy`: 验证顺序停车策略
- `it_should_use_distributed_strategy`: 验证分布式停车策略

### 工厂模式测试
- `it_should_create_economy_lot_with_first_available_strategy`: 经济型停车场测试
- `it_should_create_luxury_lot_with_vip_strategy`: 豪华型停车场测试
- `it_should_return_error_when_full`: 停车位已满错误处理测试

运行测试：
```bash
cargo test --lib parking
```

## 🔍 设计模式要点

### Rust 特有的实现考虑

1. **Trait 对象**: 使用 `Box<dyn Trait>` 实现运行时多态
2. **所有权系统**: 合理管理数据所有权和生命周期
3. **错误处理**: 使用 `Result<T, E>` 替代简单的 `Option`
4. **内存安全**: 利用 Rust 的编译时检查保证内存安全

### 最佳实践展示

- ✅ 使用枚举定义策略类型
- ✅ 通过 trait 定义统一接口
- ✅ 实现运行时策略切换
- ✅ 提供工厂方法简化对象创建
- ✅ 完整的错误处理机制

## 📚 学习资源

### 推荐阅读
- [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Design Patterns in Rust](https://rust-unofficial.github.io/patterns/)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)

### 相关模式
- 观察者模式 (Observer Pattern)
- 装饰器模式 (Decorator Pattern)
- 命令模式 (Command Pattern)

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request 来改进这个项目！

### 开发流程
1. Fork 项目仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

感谢所有为 Rust 生态系统做出贡献的开发者们！

---

*Happy Coding with Rust! 🦀*