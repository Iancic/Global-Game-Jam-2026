#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

// CRT screen curvature - more visible
fn curve_uv(uv: vec2<f32>) -> vec2<f32> {
    let curvature = 6.0; // Lower value = more visible curvature
    let uv_centered = uv * 2.0 - 1.0;
    let offset = uv_centered.yx / curvature;
    let curved = uv_centered + uv_centered * offset * offset;
    return curved * 0.5 + 0.5;
}

// Animated scanlines - more visible
fn scanline(uv: vec2<f32>, time: f32) -> f32 {
    let resolution = textureDimensions(screen_texture).y;
    let scaled_y = uv.y * f32(resolution);
    // Animate scanlines slowly
    let scanline_pos = scaled_y + time * 5.0;
    let scanline = sin(scanline_pos * 3.14159);
    // More visible scanline effect
    return scanline * 0.12 + 0.88;
}

// Phosphor mask (simulates RGB subpixel pattern) - more visible
fn phosphor_mask(uv: vec2<f32>) -> vec3<f32> {
    let resolution = textureDimensions(screen_texture);
    let pixel_x = uv.x * f32(resolution.x);
    let mask_pattern = fract(pixel_x / 3.0);
    
    if (mask_pattern < 0.333) {
        return vec3<f32>(1.0, 0.8, 0.8);
    } else if (mask_pattern < 0.666) {
        return vec3<f32>(0.8, 1.0, 0.8);
    } else {
        return vec3<f32>(0.8, 0.8, 1.0);
    }
}

// Simple bloom by sampling neighboring pixels - subtle
fn bloom(uv: vec2<f32>, strength: f32) -> vec3<f32> {
    let resolution = vec2<f32>(textureDimensions(screen_texture));
    let pixel_size = 1.0 / resolution;
    
    var bloom_color = vec3<f32>(0.0);
    let samples = 8;
    let radius = 1.5; // Smaller radius
    
    // Sample in a circle pattern
    for (var i = 0; i < samples; i++) {
        let angle = f32(i) * 3.14159 * 2.0 / f32(samples);
        let offset = vec2<f32>(cos(angle), sin(angle)) * pixel_size * radius;
        let sample_color = textureSample(screen_texture, texture_sampler, uv + offset).rgb;
        // Only accumulate bright areas
        bloom_color += max(sample_color - 0.8, vec3<f32>(0.0)); // Higher threshold
    }
    
    return bloom_color / f32(samples) * strength;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Use intensity as time for animation
    let time = settings.intensity * 100.0;
    
    // Apply screen curvature
    let curved_uv = curve_uv(in.uv);
    
    // Check if we're outside the curved screen bounds
    if (curved_uv.x < 0.0 || curved_uv.x > 1.0 || curved_uv.y < 0.0 || curved_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    
    // Chromatic aberration (RGB separation)
    let aberration = 0.0008;
    let r = textureSample(screen_texture, texture_sampler, curved_uv + vec2<f32>(aberration, 0.0)).r;
    let g = textureSample(screen_texture, texture_sampler, curved_uv).g;
    let b = textureSample(screen_texture, texture_sampler, curved_uv - vec2<f32>(aberration, 0.0)).b;
    var color = vec3<f32>(r, g, b);
    
    // Apply bloom
    let bloom_color = bloom(curved_uv, 1.2);
    color += bloom_color;
    
    // Apply animated scanlines
    let scanline_intensity = scanline(curved_uv, time);
    color *= scanline_intensity;
    
    // Apply phosphor mask
    let mask = phosphor_mask(curved_uv);
    color *= mix(vec3<f32>(1.0), mask, 0.25);
    
    // Slight brightness boost
    color *= 1.05;
    
    return vec4<f32>(color, 1.0);
}
