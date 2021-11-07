fn main() {
    let w = 30;
    let h = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(w, h)
    );

    let rect = (w, h);
    println!(
        "The area1 of the rectangle is {} square pixels.",
        area1(rect)
    );

    let rect1 = Rectangle {
        width: w,
        height: h
    };
    println!(
        "The area2 of the rectangle is {} square pixels.",
        area2(&rect1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


struct Rectangle {
    width: u32,
    height: u32,
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
