//! Shader handling stuff
use std::{fs, path};

use shaderc::{ShaderKind, Error, Compiler};

pub fn load(name: &str, entry: &str, kind: ShaderKind) -> Result<Vec<u8>, Error> {
    let mut compiler = Compiler::new()
        .ok_or(Error::NullResultObject("Can't create compiler.".to_owned()))?;

    let filepath = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("shaders")
        .join(name);

    let contents = fs::read_to_string(&filepath)
        .map_err(|e| Error::NullResultObject(format!("{}", &e)))?;

    let artifact = compiler.compile_into_spirv(&contents, kind, name, entry, None)?;
    
    Ok(artifact.as_binary_u8().to_owned())
}

pub fn load_vert(name: &str, entry: &str) -> Result<Vec<u8>, Error> {
    load(name, entry, ShaderKind::Vertex)
}

pub fn load_frag(name: &str, entry: &str) -> Result<Vec<u8>, Error> {
    load(name, entry, ShaderKind::Fragment)
}

/// Encapsulated shaders.
pub trait CompiledShaders {
    fn fragment(&self) -> &[u8];
    fn vertex(&self) -> &[u8];
}

/// Basic flat shader.
#[derive(Debug, Clone)]
pub struct FlatShaders {
    fragment: Vec<u8>,
    vertex: Vec<u8>,
}

impl FlatShaders {
    fn new(fragment: Vec<u8>, vertex: Vec<u8>) -> Self {
        FlatShaders { fragment, vertex }
    }
}

impl CompiledShaders for FlatShaders {   
    fn fragment(&self) -> &[u8] {
        self.fragment.as_slice()
    }
    
    fn vertex(&self) -> &[u8] {
        self.vertex.as_slice()
    }
}

pub fn load_flat_shaders() -> Result<impl CompiledShaders, Error> {
    let vert = load_vert("flat.vert", "main")?;
    let frag = load_frag("flat.frag", "main")?;

    Ok(FlatShaders::new(frag, vert))
}
