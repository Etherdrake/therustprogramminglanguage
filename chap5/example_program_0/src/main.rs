fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn refactored_main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_updated(rect1)
    );
}

fn area_updated(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}