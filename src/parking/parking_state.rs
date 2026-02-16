
enum ParkingSpace {
    Available,
    Occupied(String),
    OutOfService(String),
}

struct ParkingLot {
    spaces: Vec<ParkingSpace>,
}
impl ParkingLot {
    fn new(capacity: usize) -> Self {
        let spaces = (0..capacity).map(|_| ParkingSpace::Available).collect();
        Self { spaces }
    }
    fn set_out_of_service(&mut self, index: usize, reason: String) {
        self.spaces[index] = ParkingSpace::OutOfService(reason);
    }
    fn park(&mut self, plate: String) -> Result<ParkingTicket, String> {
        for (index, space) in self.spaces.iter_mut().enumerate() {
            if let ParkingSpace::Available = space {
                *space = ParkingSpace::Occupied(plate.clone());
                return Ok(ParkingTicket { space_index: index });
            }
        }
        Err("No available spaces".to_string())
    }
}
struct ParkingTicket {
    space_index: usize,
}
struct MallParkingLotFactory;
impl MallParkingLotFactory {
    fn create(capacity: usize) -> ParkingLot {
        ParkingLot::new(capacity)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_skip_out_of_service_spaces() {
        let mut lot: ParkingLot = MallParkingLotFactory::create(3);

        // 模拟：手动将第 0 号车位设置为维修中（这里可能需要你给 ParkingLot 加个方法）
        lot.set_out_of_service(0, "Broken sensor".to_string());

        let t1 = lot.park("CAR-1".to_string()).unwrap();

        // 即使是 Economy (FirstAvailable)，也应该跳过 0 号位，停在 1 号位
        assert_eq!(t1.space_index, 1);
    }
}