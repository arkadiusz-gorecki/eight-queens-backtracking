#[derive(PartialEq)]
pub struct Queen {
    pub x: usize,
    pub y: usize,
}

impl Queen {
    pub fn new(x: usize, y: usize) -> Queen {
        Queen { x, y }
    }
    pub fn x_set(&mut self, x: usize) {
        self.x = x;
    }
    pub fn y_set(&mut self, y: usize) {
        self.y = y;
    }
    pub fn attacked(&self, target: &Queen) -> bool {
        if self.x == target.x || self.y == target.y {
            return true;
        }
        let dx: i32 = self.x as i32 - target.x as i32;
        let dy: i32 = self.y as i32 - target.y as i32;
        let a = i32::abs(dx / dy);
        if a == 1 {
            return true;
        }
            return false;
    }
    fn make_move(&mut self, size: usize) {
        //let q = self.pieces.get_mut(i).expect("Nie istnieje queen w vec, out of range!!");
        if self.y + 1 >= size {
            self.y_set(0);
        } else {
            self.y_set(self.y + 1);
        }
    }
}

pub mod brute {
    use crate::*;
    pub struct BruteBoard {
        size: usize,
        pieces: Vec<Queen>,
    }

    impl BruteBoard {
        pub fn new() -> BruteBoard {
            BruteBoard { 
                size: 5,
                pieces: vec![
                    Queen::new(0, 0),
                    Queen::new(1, 0),
                    Queen::new(2, 0),
                    Queen::new(3, 0),
                    Queen::new(4, 0),
                    // Queen::new(5, 0),
                    // Queen::new(6, 0),
                    // Queen::new(7, 0)
                ]
            }
        }
        pub fn play (&mut self) {
            for _i in 0..self.size {
                for _j in 0..self.size {
                    for _k in 0..self.size {
                        for _l in 0..self.size {
                            for _ in 0..self.size {
                                if !self.attacked() {
                                    // print!("DOBRE -> : ");
                                    self.print_result();
                                }
                                self.move_queen(4);
                            }
                            self.move_queen(3);
                        }
                        self.move_queen(2);
                    }
                    self.move_queen(1);
                }
                self.move_queen(0);
            }
        }
        fn attacked(&self) -> bool {
            for q1 in self.pieces.iter() {
                for q2 in self.pieces.iter() {
                    if q1 == q2 { continue; }
                    if q1.attacked(q2) { return true; }
                }
            }
            false
        }
        fn print_result(&self) {
            for i in 0..self.size {
                print!("{}", self.pieces.get(i).unwrap().y + 1);
            }
            println!();
        }
        fn move_queen(&mut self, i: usize) {
            self.pieces.get_mut(i).expect("Nie istnieje queen w vec, out of range!!")
            .make_move(self.size);
        }
    }
}
