use std::collections::hash_map::HashMap;
use std::fs;

pub const DEFAULT_FRAGMENT: &str = "Shaders/F_UnlitSprite.shader";

pub const DEFAULT_VERTEX: &str = "Shaders/V_Standard.shader";

/// Loads all shaders stored in the Shader folder and allows query for them.
/// Avoids the needs for loading files on runtime.
pub struct ShaderModule
{
    _programLookUp: HashMap<String, String>
}


impl ShaderModule
{
    /// Creates this ShaderModule and Loads all shaders in the Shaders folder.
    pub fn Create() -> Self
    {
        let mut shaderTable = HashMap::<String,String>::new();

        let shaderFileNames: Vec<String> = fs::read_dir("Shaders")
            .unwrap()
            .map(|x| x.unwrap().path())
            .filter(|x| x.is_file())
            .filter(|x| x.extension().unwrap() == "shader")
            .map(|x| String::from(x.to_str().unwrap()))
            .collect();

        for shaderFile in &shaderFileNames 
        {
            let fileContents = fs::read_to_string(shaderFile).unwrap();
            let name = String::from(shaderFile);

            println!("Loaded Shader {}", name);

            shaderTable.insert(name, fileContents);
        }

        Self
        {
            _programLookUp: shaderTable
        }
    }

    pub fn GetShader(&self, shaderName: &str) -> String
    {
        let option = self._programLookUp.get(shaderName);

        if option.is_none()
        {
            panic!("Could not find shader {}", shaderName);
        }

        // I would prefer avoiding copying the shader program.
        String::from(option.unwrap())
    }
}





