// JavaScript has JSON but here rust has only structs so carefull to handle
// generic is the reduce code complection and duplication for the code
struct Point<T, U> {
    x: T,
    y: U,
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
    let p1 = Point { x: 3, y: 'a' };
    let p2 = Point { x: "Hello", y: 'b' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
