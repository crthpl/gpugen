mod generator;
use rust::generator::Chunk;
use rust::generate_chunk;

extern fn print_done(chunk: *const Chunk) {
    println!("chunk: {:?}", chunk);
}

pub fn main() {
    let generator = rust::new_generator(0, 16, 8);
    generate_chunk(generator, print_done as extern fn(*const Chunk), -4, 3);
    generate_chunk(generator, print_done as extern fn(*const Chunk), -4, 4);
    let generator = rust::new_generator(0, 16, 8);
    generate_chunk(generator, print_done as extern fn(*const Chunk), -4, 3);
    generate_chunk(generator, print_done as extern fn(*const Chunk), -4, 4);
    std::thread::sleep(std::time::Duration::new(100, 0));
}
