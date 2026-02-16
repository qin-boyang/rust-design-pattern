# Rust Design Patterns Practice

This project demonstrates practical examples of implementing classic design patterns in Rust, helping developers understand how to apply these patterns under Rust's type system and ownership model.

## 📋 Project Overview

This is a learning project showcasing Rust design pattern implementations. It currently includes a parking management system as the main example, demonstrating the practical application of Strategy, Factory, and State patterns.

## 🎯 Implemented Design Patterns

### 1. Strategy Pattern
**Location**: `src/parking/parking_strategy.rs`

Implements flexible parking strategy selection mechanism:

- **FirstAvailableStrategy**: Sequential parking strategy - finds the first available parking space
- **DistributedStrategy**: Distributed parking strategy - finds parking spaces from back to front

#### Core Components
```rust
pub trait ParkingStrategy {
    fn find_space(&self, spaces: &[Option<String>]) -> Option<usize>;
}

pub struct ParkingLot {
    spaces: Vec<Option<String>>,
    strategy: Box<dyn ParkingStrategy>, // Runtime strategy switching
}
```

### 2. Factory Pattern
**Location**: `src/parking/parking_strategy_factory.rs`

Provides factory for creating parking lot instances:

- **EconomyParkingStrategy**: Economy parking strategy
- **LuxuryParkingStrategy**: Luxury parking strategy (VIP priority)

#### Factory Implementation
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

## 🚀 Quick Start

### Requirements
- Rust 1.70 or higher
- Cargo package manager

### Clone and Build
```bash
git clone <repository-url>
cd rust-design-pattern
cargo build
```

### Run Tests
```bash
# Run all tests
cargo test

# Run specific module tests
cargo test parking
```

## 🔧 Project Structure

```
src/
├── lib.rs                           # Library entry point
└── parking/                         # Parking module
    ├── mod.rs                       # Module declaration
    ├── parking_strategy.rs          # Strategy pattern implementation
    ├── parking_strategy_factory.rs  # Factory pattern implementation
    └── parking_state.rs             # State pattern implementation
```

## 📖 Usage Examples

### Strategy Pattern Example
```rust
use rust_design_pattern::parking::parking_strategy::*;

// Create parking lots with different strategies
let mut first_lot = ParkingLot::new(10, Box::new(FirstAvailableStrategy));
let mut dist_lot = ParkingLot::new(10, Box::new(DistributedStrategy));

// Parking behavior varies based on strategy
let ticket1 = first_lot.park("ABC123".to_string()); // Park in first available spot
let ticket2 = dist_lot.park("XYZ789".to_string());   // Park in last available spot
```

### Factory Pattern Example
```rust
use rust_design_pattern::parking::parking_strategy_factory::*;

// Use factory to create different types of parking lots
let mut economy_lot = MallParkingLotFactory::create(LotLevel::Economy, 5);
let mut luxury_lot = MallParkingLotFactory::create(LotLevel::Luxury, 5);

// Different parking lot types use different parking strategies
let eco_ticket = economy_lot.park("ECONOMY".to_string());
let lux_ticket = luxury_lot.park("LUXURY".to_string());
```

## 🧪 Test Coverage

The project includes comprehensive unit test coverage:

### Strategy Pattern Tests
- `it_should_use_first_available_strategy`: Validates sequential parking strategy
- `it_should_use_distributed_strategy`: Validates distributed parking strategy

### Factory Pattern Tests
- `it_should_create_economy_lot_with_first_available_strategy`: Economy parking lot test
- `it_should_create_luxury_lot_with_vip_strategy`: Luxury parking lot test
- `it_should_return_error_when_full`: Full parking space error handling test

### State Pattern Tests
- `it_should_skip_out_of_service_spaces`: Validates skipping out-of-service spaces functionality

Run tests:
```bash
cargo test --lib parking
```

## 🔍 Design Pattern Highlights

### Rust-Specific Implementation Considerations

1. **Trait Objects**: Using `Box<dyn Trait>` for runtime polymorphism
2. **Ownership System**: Properly managing data ownership and lifetimes
3. **Error Handling**: Using `Result<T, E>` instead of simple `Option`
4. **Memory Safety**: Leveraging Rust's compile-time checks for memory safety

### Best Practices Demonstrated

- ✅ Using enums to define strategy types
- ✅ Defining unified interfaces through traits
- ✅ Implementing runtime strategy switching
- ✅ Providing factory methods to simplify object creation
- ✅ Complete error handling mechanisms
- ✅ State encapsulation and transitions in State pattern

## 📚 Learning Resources

### Recommended Reading
- [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Design Patterns in Rust](https://rust-unofficial.github.io/patterns/)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)

### Related Patterns
- Observer Pattern
- Decorator Pattern
- Command Pattern

## 🤝 Contribution Guidelines

Welcome to submit Issues and Pull Requests to improve this project!

### Development Process
1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add some amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## 🙏 Acknowledgements

Thanks to all developers who contribute to the Rust ecosystem!

---

*Happy Coding with Rust! 🦀*