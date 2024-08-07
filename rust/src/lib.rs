use env_logger::Target;
use smol::future;
use smol::Executor;
use std::fs::File;
use std::sync::mpsc;
use std::thread;
use async_mutex::Mutex;
pub mod generator;

pub struct Generator<'a> {
    generator: generator::Generator,
    spawner: mpsc::Sender<(
        &'a generator::Generator,
        extern "C" fn(* const generator::Chunk),
        i32,
        i32,
    )>, //executor: Executor<'a>,
}


#[no_mangle]
pub extern "C" fn new_generator(
    min_height: i32,
    height: u32,
    sea_level: i32,
) -> *const Generator<'static> {
    static LOGGER_INIT: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

    let file = File::create("logs/gpugen.log").unwrap();
    let mut logger_init = LOGGER_INIT.lock().unwrap();
    if !*logger_init {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_nanos()
        .target(Target::Pipe(Box::new(file)))
        .init();
        *logger_init = true
    }


    println!("min height before: {}", min_height);
    let generator = smol::block_on(generator::Generator::new(min_height, height, sea_level));
    let (sender, receiver) = mpsc::channel();

    let generator = Box::leak(Box::new(Generator {
        generator,
        spawner: sender,
    })) as *const Generator;

    let executor: &'static Executor = Box::leak(Box::new(Executor::new()));

    thread::spawn(move || {
        easy_parallel::Parallel::new()
            .each(0..16, |i| {
                println!("going to tick... {}", i);
                future::block_on(async {
                    loop {
                        println!("ticking! {}", i);
                        executor.tick().await
                    }
                })
            })
            .run()
    });
    thread::spawn(move || loop {
        println!("recieving.....");
        match receiver.recv() {
            Ok((generator, callback, x, z)) => {
                println!("recieved!!!, {}, {}", x, z);
                executor.spawn(callbacker(generator, callback, x, z)).detach();
            }
            Err(err) => {
                eprintln!("sending ended: {}", err);
                break;
            }
        };
    });

    println!(
        "min height now: {}, addar: {:x}",
        unsafe {
            generator
                .as_ref()
                .expect("wat how nul")
                .generator
                .get_min_height()
        },
        generator as usize
    );
    generator
}

async fn callbacker(
    generator: &generator::Generator,
    callback: extern "C" fn(* const generator::Chunk),
    x: i32,
    z: i32,
) {
    static mutex: async_mutex::Mutex<()> = Mutex::new(());

    let guard = mutex.lock().await;
    //println!(">0.generating chunk {}, {}", x, z);
    //let chunk_future = generator.generate_chunk(x, z);
    println!(">1.generating chunk {}, {}", x, z);
    let chunk = generator.get_chunk(x, z).await;
    println!(">2.generated chunk {}, {}", x, z);
    for i in 0..16 {
        for j in 0..16 {
                print!("{:?} ", unsafe {*chunk.blocks.wrapping_add(i+j*16)});
        }
        println!("");
    }
    callback(&chunk);
    drop(guard);
    println!(">3.returned chunk {}, {}", x, z);
}

#[no_mangle]
pub extern "C" fn generate_chunk(
    generator: *const Generator,
    callback: extern "C" fn(*const generator::Chunk),
    x: i32,
    z: i32,
) {
    println!(
        "min height after: {}, addr: {:x}",
        unsafe {
            generator
                .as_ref()
                .expect("wat how nul")
                .generator
                .get_min_height()
        },
        generator as usize
    );
    let generator = unsafe { &*generator };
    println!(
        "min height afftter: {}",
        generator.generator.get_min_height()
    );
    let _ = generator
        .spawner
        .send((&generator.generator, callback, x, z));
}

//#[no_mangle]
//pub extern "C" fn get_chunk(
//    generator: *const Generator,
//    chunk_future: *const ChunkFuture,
//) -> Chunk {
//    let generator = unsafe { &*generator };
//    let chunk_future = unsafe { &*chunk_future };
//    pollster::block_on(generator.get_chunk(chunk_future))
//}

// height, min height, sea level, debug text

#[no_mangle]
pub extern "C" fn get_height(generator: *const Generator) -> u32 {
    let generator = unsafe { &*generator };
    generator.generator.get_height()
}

#[no_mangle]
pub extern "C" fn get_min_height(generator: *const Generator) -> i32 {
    let generator = unsafe { &*generator };
    generator.generator.get_min_height()
}

#[no_mangle]
pub extern "C" fn get_sea_level(generator: *const Generator) -> i32 {
    let generator = unsafe { &*generator };
    generator.generator.get_sea_level()
}

#[no_mangle]
pub extern "C" fn get_debug_text(
    generator: *const Generator,
    x: i32,
    y: i32,
    z: i32,
) -> *const std::os::raw::c_char {
    let generator = unsafe { &*generator };
    let debug_text = generator.generator.get_debug_text(x, y, z);
    std::ffi::CString::new(debug_text).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_generator(generator: *const Generator) {
    let _ = unsafe { Box::from_raw(generator as *mut Generator) };
}

//#[no_mangle]
//pub extern "C" fn free_chunk_future(chunk_future: *const ChunkFuture) {
//    let _ = unsafe { Box::from_raw(chunk_future as *mut ChunkFuture) };
//}

#[no_mangle]
pub extern "C" fn free_chunk(chunk: *const generator::Chunk) {
    let _ = unsafe { Box::from_raw(chunk as *mut generator::Chunk) };
}

#[no_mangle]
pub extern "C" fn free_string(string: *const std::os::raw::c_char) {
    let _ = unsafe { std::ffi::CString::from_raw(string as *mut std::os::raw::c_char) };
}
