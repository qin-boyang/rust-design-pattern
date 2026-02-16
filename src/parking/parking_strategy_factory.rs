enum LotLevel {
    Economy,
    Luxury,
}
trait ParkingStrategy {
    fn find_space(&mut self, space: &Vec<Option<String>>) -> Option<usize>;
}
struct EconomyParkingStrategy;
impl ParkingStrategy for EconomyParkingStrategy {
    fn find_space(&mut self, spaces: &Vec<Option<String>>) -> Option<usize> {
        spaces.iter().position(|space| space.is_none())
    }
}

struct LuxuryParkingStrategy;
impl ParkingStrategy for LuxuryParkingStrategy {
    fn find_space(&mut self, spaces: &Vec<Option<String>>) -> Option<usize> {
        // 创建 luxury parking strategy
        spaces.iter().rposition(|space| space.is_none())
    }
}
struct ParkingLot {
    spaces: Vec<Option<String>>,
    strategy: Box<dyn ParkingStrategy>,
}

impl ParkingLot {
    fn new(capacity: u64, strategy: Box<dyn ParkingStrategy>) -> ParkingLot {
        Self {
            spaces: vec![None; capacity as usize], // 修复：确保 Vec 有长度
            strategy,
        }
    }
    fn park(&mut self, vehicle: String) -> Result<ParkingTicket, String> {
        let space_index = self.strategy.find_space(&self.spaces);
        match space_index {
            Some(index) => {
                self.spaces[index] = Some(vehicle.clone());
                Ok(ParkingTicket { vehicle, space_index: index })
            }
            None => Err("Parking lot is full".to_string()),
        }
    }
}
struct ParkingTicket {
    vehicle: String,
    space_index: usize,
}

struct MallParkingLotFactory;
impl MallParkingLotFactory {
    fn create(level: LotLevel, capacity: u64) -> ParkingLot {
        match level {
            LotLevel::Economy => {
                // 创建经济型停车场
                ParkingLot::new(capacity, Box::new(EconomyParkingStrategy))
            }
            LotLevel::Luxury => {
                // 创建豪华型停车场
                ParkingLot::new(capacity, Box::new(LuxuryParkingStrategy))
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_economy_lot_with_first_available_strategy() {
        // 使用工厂创建经济型停车场
        let mut economy_lot: ParkingLot = MallParkingLotFactory::create(LotLevel::Economy, 3);

        let t1: ParkingTicket = economy_lot.park("CAR-1".to_string()).expect("Should park");
        let t2: ParkingTicket = economy_lot.park("CAR-2".to_string()).expect("Should park");

        // 经济型应该是从索引 0 开始顺序停放
        assert_eq!(t1.space_index, 0);
        assert_eq!(t2.space_index, 1);
    }

    #[test]
    fn it_should_create_luxury_lot_with_vip_strategy() {
        // 使用工厂创建豪华型停车场
        let mut luxury_lot: ParkingLot = MallParkingLotFactory::create(LotLevel::Luxury, 3);

        let t1 = luxury_lot.park("VIP-1".to_string()).expect("Should park");
        let t2 = luxury_lot.park("VIP-2".to_string()).expect("Should park");

        // 豪华型使用 VIP 策略：寻找最后一个空位（索引递减）
        assert_eq!(t1.space_index, 2);
        assert_eq!(t2.space_index, 1);
    }

    #[test]
    fn it_should_return_error_when_full() {
        let mut lot: ParkingLot = MallParkingLotFactory::create(LotLevel::Economy, 1);
        lot.park("CAR-1".to_string()).unwrap();

        let result = lot.park("CAR-2".to_string());

        // 考点：这里建议返回 Result 而不是 Option，能体现更好的错误处理习惯
        assert!(result.is_err());
    }
}