/*
 * @lc app=leetcode id=2336 lang=rust
 *
 * [2336] Smallest Number in Infinite Set
 */



// @lc code=start
struct SmallestInfiniteSet {
    removedInt:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self { removedInt: vec![] }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        
        for x in 0..self.removedInt.len(){
            if(self.removedInt[x] != x as i32+1){
                self.removedInt.insert(x, x as i32+1);
                return x as i32 +1;
            }
        }
        self.removedInt.push(self.removedInt.len() as i32+1);
        return self.removedInt.len() as i32;
    }
    
    fn add_back(&mut self, num: i32) {
        match self.removedInt.binary_search(&num){
            Ok(x) => {self.removedInt.remove(x);},
            _ => ()
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
// @lc code=end

