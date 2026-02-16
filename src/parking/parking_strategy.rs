/// 1. 定义策略接口
pub trait ParkingStrategy {
    fn find_space(&self, spaces: &[Option<String>]) -> Option<usize>;
}

/// 2. 实现“顺序停车”策略
pub struct FirstAvailableStrategy;
impl ParkingStrategy for FirstAvailableStrategy {
    fn find_space(&self, spaces: &[Option<String>]) -> Option<usize> {
        // 寻找第一个为 None 的索引
        spaces.iter().position(|s| s.is_none())
    }
}

/// 3. 实现“反向停车/分布式”策略
pub struct DistributedStrategy;
impl ParkingStrategy for DistributedStrategy {
    fn find_space(&self, spaces: &[Option<String>]) -> Option<usize> {
        // 从后往前找第一个 None，或者用 rposition
        spaces.iter().rposition(|s| s.is_none())
    }
}

pub struct Ticket {
    pub space_index: usize,
    pub vehicle_plate: String,
}

/// 4. 停车场主体
pub struct ParkingLot {
    spaces: Vec<Option<String>>,
    // 使用 Box<dyn ...> 允许在运行时动态更换策略
    strategy: Box<dyn ParkingStrategy>,
}

impl ParkingLot {
    pub fn new(size: usize, strategy: Box<dyn ParkingStrategy>) -> Self {
        Self {
            spaces: vec![None; size], // 简洁的初始化方式
            strategy,
        }
    }

    pub fn park(&mut self, car_plate: String) -> Option<Ticket> {
        // 委托策略对象寻找位子
        let index = self.strategy.find_space(&self.spaces)?;

        // 修改状态
        self.spaces[index] = Some(car_plate.clone());

        Some(Ticket {
            space_index: index,
            vehicle_plate: car_plate,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_use_first_available_strategy() {
        // 准备一个有3个空位的停车场
        let mut lot = ParkingLot::new(3, Box::new(FirstAvailableStrategy));

        let ticket = lot.park("CAR-1".to_string()).unwrap();
        // 预期停在第 0 号位
        assert_eq!(ticket.space_index, 0);
    }

    #[test]
    fn it_should_use_distributed_strategy() {
        // 注入“找索引号最大”的策略
        let mut lot = ParkingLot::new(3, Box::new(DistributedStrategy));

        let ticket = lot.park("CAR-1".to_string()).unwrap();
        // 预期停在第 2 号位
        assert_eq!(ticket.space_index, 2);
    }
}