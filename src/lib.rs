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
    pub fn can_attack(&self, target: &Queen) -> bool {
        let dx: i32 = (self.x as i32 - target.x as i32).abs();
        let dy: i32 = (self.y as i32 - target.y as i32).abs();
        if self.x == target.x || self.y == target.y || dx == dy {
            true
        } else {
            false
        }
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

pub mod back {
    use crate::*;
    pub struct BackBoard {
        size: usize,
        pieces: Vec<Queen>,
        solutions_found: u32,
    }

    impl BackBoard {
        pub fn new() -> BackBoard {
            BackBoard {
                size: 8,
                pieces: Vec::new(),
                solutions_found: 0,
            }
        }
        pub fn play(&mut self) {
            self.recur(0);
            println!("Liczba rozwiazan: {}", self.solutions_found);
        }
        fn recur(&mut self, placed: usize) {
            // warunek zerowy
            if placed >= self.size {
                self.print_result();
                self.solutions_found += 1;
                return;
            }
            // warunek normalny
            for i in 0..self.size {
                let suggestion = Queen::new(placed, i);
                if self.attacked(&suggestion) {
                    continue;
                } else {
                    self.pieces.push(suggestion);
                    self.recur(placed + 1);
                    self.pieces.pop();
                }
            }
        }
        fn attacked(&self, source: &Queen) -> bool {
            for target in self.pieces.iter() {
                if source == target {
                    continue;
                }
                if source.can_attack(target) {
                    return true;
                }
            }
            return false;
        }
        fn print_result(&self) {
            for i in 0..self.pieces.len() {
                print!(
                    "{}",
                    self.pieces.get(i).expect("print result, poza zasieg").y + 1
                );
            }
            println!();
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
                size: 8,
                pieces: vec![
                    Queen::new(0, 0),
                    Queen::new(1, 0),
                    Queen::new(2, 0),
                    Queen::new(3, 0),
                    Queen::new(4, 0),
                    Queen::new(5, 0),
                    Queen::new(6, 0),
                    Queen::new(7, 0),
                ],
            }
        }
        pub fn play(&mut self) {
            let mut solutions_found: u32 = 0;
            for _ in 0..self.size {
                for _ in 0..self.size {
                    for _ in 0..self.size {
                        for _ in 0..self.size {
                            for _ in 0..self.size {
                                for _ in 0..self.size {
                                    for _ in 0..self.size {
                                        for _ in 0..self.size {
                                            if !self.attacked() {
                                                solutions_found += 1;
                                                self.print_result();
                                            }
                                            self.move_queen(7);
                                        }
                                        self.move_queen(6);
                                    }
                                    self.move_queen(5);
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
            println!("Liczba rozwiazan: {}", solutions_found);
        }
        fn attacked(&self) -> bool {
            for q1 in self.pieces.iter() {
                for q2 in self.pieces.iter() {
                    if q1 == q2 {
                        continue;
                    }
                    if q1.can_attack(q2) {
                        return true;
                    }
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
            self.pieces
                .get_mut(i)
                .expect("Nie istnieje queen w vec, out of range!!")
                .make_move(self.size);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::back::*;
    use crate::brute::*;
    use std::time;

    #[test]
    fn bruteforce_time() {
        let mut game = BruteBoard::new();
        let start = time::Instant::now();
        game.play();
        let end = time::Instant::now();
        println!("Brute force time: {:?}", end - start)
    }

    #[test]
    fn backtracking_time() {
        let mut game = BackBoard::new();
        let start = time::Instant::now();
        game.play();
        let end = time::Instant::now();
        println!("Backtracking time: {:?}", end - start)
    }
}
