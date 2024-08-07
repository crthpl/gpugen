struct PositionUniform {
    position: vec3<i32>,
};

@group(0) @binding(0)
var<uniform> positionUniform: PositionUniform;

@group(0) @binding(1)
var<storage, read_write> chunk: array<u32>;
// compute shader


@compute
@workgroup_size(1, 1, 1)
fn main(
    @builtin(local_invocation_index) local_invocation_index: u32,
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
    @builtin(global_invocation_id) id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id : vec3<u32>
) {
      let workgroup_index =  
     workgroup_id.x +
     workgroup_id.y * num_workgroups.x +
     workgroup_id.z * num_workgroups.x * num_workgroups.y;

    let global_invocation_index = workgroup_index * 16 + local_invocation_index;
	var result = u32(3);
     if (id.y > 5) {
        result = u32(0);
    } else {
        result = u32(1);
    }
    chunk[global_invocation_index] = result;
}
