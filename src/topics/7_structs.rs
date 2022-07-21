#[derive(Debug)]
struct Rectangle {
    height: i16,
    width: i16,
}

impl Rectangle {
    fn set_height(&mut self, height: i16) {
        self.height = height;
    }
    fn set_width(&mut self, width: i16) {
        self.width = width;
    }
    fn area(&self) -> i16 {
        return self.width * self.height;
    }
}

fn fun(rect: &mut Rectangle) -> i16 {
    let x: i16 = 10;
    let y: i16 = 10;
    rect.set_height(x);
    rect.set_width(y);
    return rect.area();
}

pub fn learn() {
    let mut rect = Rectangle {
        width: 0,
        height: 0,
    };

    // for format printing
    dbg!(&rect);

    let area: i16 = fun(&mut rect);
    println!("area of rectangle {}", area);
    println!("area of rectangle {} {}", rect.height, rect.width)
}
