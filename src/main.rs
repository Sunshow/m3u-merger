use std::io;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

mod artifact;
mod test;
mod fetch;
mod parser;
mod playlist;

lazy_static! {
    // 获取执行程序所在目录
    static ref SCRIPT_PATH: String = current_exe_dir().unwrap().to_str().unwrap().to_string();
}

fn current_exe_dir() -> io::Result<PathBuf> {
    let mut dir = std::env::current_exe()?;
    dir.pop();
    Ok(dir)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Script path: {}", SCRIPT_PATH.as_str());
    // 从程序目录的 conf 目录读取 config.yaml 配置文件
    let config_location = format!("{}/conf/config.yaml", SCRIPT_PATH.as_str());
    // 判断文件是否存在
    let config_path = Path::new(&config_location);
    if !config_path.exists() {
        println!("Config file not found: {}", config_location);
        return Ok(());
    }
    println!("Found config file: {:?}", config_path);

    process_config(&config_path).await?;

    Ok(())
}

async fn process_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open(path).unwrap();

    match serde_yaml::from_reader(f) {
        Ok(parsed_config) => {
            let config: artifact::ArtifactConfig = parsed_config;
            println!("Config: {:?}", config);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}