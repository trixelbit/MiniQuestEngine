use std::env;
use std::path::{self, Path, PathBuf};


/// Gets the abolute path from a path relative to an exe
///
pub fn GetAbsolutePathRelativeToEXE(path: &str) -> String
{
    let mut exe = env::current_exe().unwrap();
    exe.pop();

    let path = 
        Path::join(
            exe.as_path(),
            PathBuf::from(path));

    let config_path = path::absolute(path).unwrap();
    
    String::from(config_path.to_str().unwrap())
}
