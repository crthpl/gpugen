use async_mutex::Mutex;
use std::collections::HashMap;

pub struct Generator {
    min_height: i32,
    height: u32,
    sea_level: i32,
    pallete: HashMap<String, i32>,
    //chunks: HashMap<(i32, i32), ChunkFuture>,
    //adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    //shader: wgpu::ShaderModule,
    chunk_buffer: wgpu::Buffer,
    readback_buffer: wgpu::Buffer,
    readback_mutex: Mutex<()>,
    position_buffer: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
}

const CHUNK_WIDTH: usize = 16; // must be multiple of 256 bc who knows

impl Generator {
    pub async fn new(min_height: i32, height: u32, sea_level: i32) -> Generator {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .unwrap();
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let chunk_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: ((std::mem::size_of::<i32>()*CHUNK_WIDTH*CHUNK_WIDTH) as u64)*(height as u64),
            usage: wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
        });

        let position_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<[i32; 4]>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (4 * (CHUNK_WIDTH * CHUNK_WIDTH) as u32 * height) as wgpu::BufferAddress,
            // Can be read to the CPU, and can be copied from the shader's storage buffer
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: position_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: chunk_buffer.as_entire_binding(),
                },
            ],
        });

        Generator {
            min_height,
            height,
            sea_level,
            pallete: HashMap::new(),

            //chunks: HashMap::new(),
            device,
            queue,
            chunk_buffer,
            bind_group,
            readback_buffer,
            readback_mutex: Mutex::new(()),
            position_buffer,
            compute_pipeline,
            //buffer_slice: None,
            //receiver: flume::unbounded().1,
        }
    }

    // y-coordinate of the bottom of the world
    pub fn get_min_height(&self) -> i32 {
        self.min_height
    }

    // distance from the bottom of the world to the top of the world
    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_sea_level(&self) -> i32 {
        self.sea_level
    }

    pub fn get_debug_text(&self, x: i32, y: i32, z: i32) -> String {
        format!(
            "min_height: {}, height: {}, sea_level: {}, x: {}, y: {}, z: {}",
            self.min_height, self.height, self.sea_level, x, y, z
        )
    }

    // Tells the generator what block ids there are and their *mappings* to ints
    pub fn set_pallete(&mut self, pallete: HashMap<String, i32>) {
        self.pallete = pallete;
    }

    // generates a chunk
   // pub fn generate_chunk(&self, x: i32, z: i32) -> ChunkFuture {


   //     ChunkFuture { encoder }
   // }
    // returns the Chunk
    pub async fn get_chunk(&self, x: i32, z: i32) -> Chunk {
        println!("<01.locking mutex");
        //let guard = self.readback_mutex.lock().await;
        println!("<01.locked mutex");
        self.queue.write_buffer(
            &self.position_buffer,
            0,
            bytemuck::cast_slice(&[[x * CHUNK_WIDTH as i32, z * CHUNK_WIDTH as i32, 0]]),
        );
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                timestamp_writes: None,
                label: None,
            });
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.dispatch_workgroups(
                CHUNK_WIDTH as u32,
                CHUNK_WIDTH as u32,
                self.height,
            );
        }
        encoder.copy_buffer_to_buffer(
            &self.chunk_buffer,
            0,
            &self.readback_buffer,
            0,
            self.chunk_buffer.size()
        );

        let submission_index = self.queue.submit(Some(encoder.finish()));
        let buffer_slice = self.readback_buffer.slice(..);
        let (sender, receiver) = flume::bounded(1);
        println!("<02.Mapping Buffer");
        buffer_slice.map_async(wgpu::MapMode::Read, move |r| {
            println!("buffer mapped!");
            sender.send(r).expect("couldn't send")
        });
        self.device.poll(wgpu::Maintain::WaitForSubmissionIndex(
            submission_index.clone(),
        ));
        let blocks = {
            println!("<03.Receiving Buffer");
receiver
                .recv_async()
                .await
                .expect("couldn't recieve")
                .expect("couldn't map");
            println!("<04.Received Buffer");
            let view = buffer_slice.get_mapped_range();
            slice_to_chunk(&view)
        };
        self.readback_buffer.unmap();
        //drop(guard);
        blocks
        //  self.chunks
        //      .get(&(x, z))
        //      .unwrap()
        //      .buffer_future
        //      .await
        //      .map(|_| Chunk {
        //          blocks: buffer_slice
        //              .get_mapped_range()
        //              .chunks_exact(4)
        //              .map(|b| i32::from_ne_bytes(b.try_into().unwrap()))
        //              .collect::<Vec<_>>(),
        //      })
        //self.chunks.get(&(x, z)).unwrap()
    }
}

fn slice_to_chunk(slice: &[u8]) -> Chunk {
    println!("====slice========");
    for i in 0..16 {
        for j in 0..16 {
            print!("{} ", slice[i+j*16]);
        }
        println!("");
    }
    let dst =  Box::new(vec![0 as u8; slice.len()]);
    let dst = Box::leak(dst);
    dst.copy_from_slice(slice);
    let u32_slice: &[u32] = bytemuck::try_cast_slice(&dst).unwrap();

    //let height = u32_slice.len() / (CHUNK_WIDTH * CHUNK_WIDTH);
    //let mut blocks: Vec<[[u32; CHUNK_WIDTH]; CHUNK_WIDTH]> = Vec::with_capacity(height);
    //for y in 0..height {
    //    let mut plane = [[0; CHUNK_WIDTH]; CHUNK_WIDTH];
    //    for x in 0..CHUNK_WIDTH {
    //        plane[x] = u32_slice[y * (CHUNK_WIDTH * CHUNK_WIDTH) + x * CHUNK_WIDTH
    //            ..y * (CHUNK_WIDTH * CHUNK_WIDTH) + (x + 1) * CHUNK_WIDTH]
    //            .try_into()
    //            .unwrap();
    //    }
    //    blocks.push(plane);
    //}
    Chunk {
        blocks: u32_slice.as_ptr(),
    }
}

pub struct ChunkFuture {
    //buffer_slice: wgpu::BufferSlice<'a>,
    //receiver: flume::Receiver<Result<(), wgpu::BufferAsyncError>>,
    encoder: wgpu::CommandEncoder,
}

#[derive(Debug)]
#[repr(C)]
pub struct Chunk {
    pub blocks: *const u32,
}
