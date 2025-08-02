use std::thread;
use std::time::Duration;

fn simulated_expensive_computation(intensity: u32) -> u32 {
    // Simulate a computation that takes some time
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_intensity,
        simulated_random_number,
    );

    let x = 4;

    let equal_to_x = |z| z == x;
    // let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // };
    // let y = vec![1, 2, 3];
    let y = 4;
    assert!(equal_to_x(y), "y should be equal to x!"); // This will panic if y is not equal to x
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });  
    // let expensive_result = simulated_expensive_computation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity));
        }
    }
}