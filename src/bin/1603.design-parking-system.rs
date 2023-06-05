/*
 * @lc app=leetcode id=1603 lang=rust
 *
 * [1603] Design Parking System
 */

// @lc code=start
struct ParkingSystem {
    availibleSpots: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self { availibleSpots: vec![big,medium,small] }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        self.availibleSpots[car_type as usize - 1]-=1;
        self.availibleSpots[(car_type as usize) - 1] > -1
    }
}

// @lc code=end

