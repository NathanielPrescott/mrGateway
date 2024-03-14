use config::Config;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Settings {
    cache_generated_location: String,
    cache_generated_file: String,
    cache_generated_bin: String,
    cache_proto_url: String,
    cache_proto_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_settings();
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    download_proto(&settings, &out_dir)
        .expect(format!("Downloading cache failed from url: {}", settings.cache_proto_url).as_str());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(false)
        .file_descriptor_set_path(out_dir.join(settings.cache_generated_bin))
        .out_dir(settings.cache_generated_location)
        .compile(&[out_dir.join(settings.cache_proto_file)], &[out_dir])
        .expect("Building proto failed");

    Ok(())
}

// Download externally needed proto file
fn download_proto(settings: &Settings, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = path.join(&settings.cache_proto_file);

    match reqwest::blocking::get(&settings.cache_proto_url) {
        Ok(response) => {
            let bytes = response.bytes()?;
            let mut file = File::create(&file_path)?;
            file.write_all(&bytes)?;
        }
        Err(e) => {
            if !file_path.exists() {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn get_settings() -> Settings {
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, HashMap<String, String>>>()
        .unwrap();

    let build = settings.get("build").unwrap();

    let cache_generated_location: String = build.get("cache_generated_location").unwrap().clone();
    let cache_generated_file: String = build.get("cache_generated_file").unwrap().clone();
    let cache_generated_bin: String = build.get("cache_generated_bin").unwrap().clone();
    let cache_proto_url: String = build.get("cache_proto_url").unwrap().clone();
    let cache_proto_file: String = build.get("cache_proto_file").unwrap().clone();

    Settings {
        cache_generated_location,
        cache_generated_file,
        cache_generated_bin,
        cache_proto_url,
        cache_proto_file,
    }
}
