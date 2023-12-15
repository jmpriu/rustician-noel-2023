/*
Using a second grade ecuation to solve the problem
x * (Time - x) - Distance = 0;
x * Time - x² - Distance = 0;
-x² + Time * x - Distance = 0;
(-b + sqrt(b² - 4ac)) / 2a;
x1 = (-Time + sqrt(Time² - 4 * (-1) * (-Distance))) / 2 * (-1);
x2 = (-Time - sqrt(Time² - 4 * (-1) * (-Distance))) / 2 * (-1);
All values between x1 and x2 are valid
*/
struct Race(u64, u64);
impl Race {
    fn different_ways_to_win(&self) -> usize {
        let a: f64 = - 1.0;
        let b: f64 = self.0 as f64;
        let c: f64 = - (self.1 as f64);
        let x1 = ((-b + (b.powf(2.0) - 4.0 * a * c).sqrt()) / (2.0 * a)).floor();
        let x2 = ((-b - (b.powf(2.0) - 4.0 * a * c).sqrt()) / (2.0 * a)).ceil();
        (x1 - x2 + 1.0).abs() as usize
    }
}

fn main() {
    // let races: Vec<Race> = vec![Race(56, 334), Race(71, 1135), Race(79, 1350),  Race(99, 2430)];
    let races: Vec<Race> = vec![Race(56717999, 334113513502430)];
    let mut mult = 1;
    for x in races {
        mult *= x.different_ways_to_win();
    }
    println!("Result: {}", mult);
}
