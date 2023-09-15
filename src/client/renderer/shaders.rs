use std::{fs, io};

use wgpu::{ShaderModule, Device, ShaderModuleDescriptor, ShaderSource};

pub struct Shaders {
    pub shader: Shader
}

impl Shaders {
    pub fn new(device: &Device) -> Self {
        Self {
            shader: Shader::new(device, "shader").unwrap()
        }
    }
}

pub struct Shader {
    pub inner: ShaderModule
}

impl Shader {
    pub fn new(device: &Device, name: &str) -> Result<Self, io::Error> {
        let file = fs::read_to_string(
            format!("{}/resources/shaders/{}.wgsl", env!("CARGO_MANIFEST_DIR"), name)
        )?;

        let desc = ShaderModuleDescriptor {
            label: Some(name),
            source: ShaderSource::Wgsl(file.into())
        };
        
        Ok(Self {
            inner: device.create_shader_module(desc)
        })
    }
}