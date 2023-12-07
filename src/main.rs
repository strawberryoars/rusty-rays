
use std::io;
use std::io::Write;

fn main() {

    // IMAGE
    let image_width = 256;
    let image_height =  256;

    // Render
    for i in 0..image_height {
        for j in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = i as f64 / (image_height - 1) as f64;
            let b = 0 as f64;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
            writeln!(&mut io::stdout(),"{} {} {}", ir, ig, ib);
        }
    }
}
