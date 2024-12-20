use imshow::imshow;

fn main() {
    let img = image::open("examples/cat.jpg").unwrap();
    imshow(&img);
}
