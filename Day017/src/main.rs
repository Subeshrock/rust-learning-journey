struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 67, 23, 89, 12, 45];

    let largest_number = get_largest(number_list);
    println!("The largest number in the list is: {}", largest_number);

    let number_list2 = vec![5, 10, 15, 20, 25];
    let largest_number2 = get_largest(number_list2);
    println!("The largest number in the second list is: {}", largest_number2);

    let chart_list = vec!['a', 'b', 'c', 'd', 'e'];
    let largest_char = get_largest(chart_list);
    println!("The largest character in the list is: {}", largest_char);

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    let p3 = Point { x: "Hello", y: "World" };
    let p4 = Point { x: 3.14, y: 2 };

    let mixed_point = p2.mixup(p3);
    println!("Mixed Point: x = {}, y = {}", mixed_point.x(), mixed_point.y());
    
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
