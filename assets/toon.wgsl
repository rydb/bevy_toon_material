// Heavily adapted from https://github.com/tbillington/bevy_toon_shader
// And https://roystan.net/articles/toon-shader/
#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> light_direction: vec3<f32>;
@group(2) @binding(2) var<uniform> light_color: vec4<f32>;
@group(2) @binding(3) var<uniform> camera_position: vec3<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // shading the object
    let normal = normalize(input.world_normal); // make the world_normal of the input mesh have a length of one
    let n_dot_l = dot(light_direction, normal) ;
    var light_intensity = smoothstep(0.0, 0.01, n_dot_l); // smooth out the harsh shadow a little
    // if n_dot_l > 0.0 {
    //     let bands = 1.0;
    //     var x = round(n_dot_l * bands);
    //     light_intensity = x / bands;
    // } else {
    //     light_intensity = 0.0;
    // }

    let ambient_light = vec4<f32>(0.1, 0.1, 0.1, 1.0); // could be anything
    let light = (light_intensity * light_color);

    // specular 
    let view_direction = normalize(camera_position - input.world_position.xyz);
    let half_vector = normalize(light_direction + view_direction);
    let n_dot_h = dot(normal, half_vector);
    let spec_color = vec4<f32>(0.9, 0.9, 0.9, 1.0);
    let glossiness = 32.0;
    let spec_intensity = pow(n_dot_h * light_intensity, glossiness * glossiness);
    let spec_intensity_smooth = smoothstep(0.005, 0.01, spec_intensity);
    let specular = spec_intensity_smooth * spec_color;

    // rim lighting
    let rim_color = vec4<f32>(1.0);
    let rim_amount = 0.716; // float between 0 and 1
    let rim_dot = 1 - dot(view_direction, normal);
    let rim_threshold = 0.1; // float between 0 and 1
    var rim_intensity = rim_dot * pow(n_dot_l, rim_threshold);
    rim_intensity = smoothstep(rim_amount - 0.01, rim_amount + 0.01, rim_intensity);
    let rim = rim_intensity * rim_color;

    return material_color * (ambient_light + light + specular + rim);
}