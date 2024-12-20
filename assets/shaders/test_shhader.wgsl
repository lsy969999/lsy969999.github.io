// shader.wgsl

// Vertex Shader
@vertex
fn vertex_main(
    @location(0) position: vec3<f32>, // 입력 정점 위치
    @builtin(vertex_index) index: u32 // 정점 인덱스 (예제용)
) -> @builtin(position) vec4<f32> {
    // 기본적으로 입력 정점을 출력으로 전달
    return vec4<f32>(position, 1.0);
}

// Fragment Shader
@fragment
fn fragment_main() -> @location(0) vec4<f32> {
    // 단색 (빨간색)으로 렌더링
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}