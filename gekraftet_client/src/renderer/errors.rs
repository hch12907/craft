#[derive(Clone, Debug)]
pub enum RenderError {
    ShaderCompile(String),
}
