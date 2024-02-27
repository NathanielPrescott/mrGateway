use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url_proto_file = "https://raw.githubusercontent.com/NathanielPrescott/mrCache/main/proto/mr_cache.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    download_proto(url_proto_file, &out_dir).expect(format!("Downloading cache failed from url: {}", url_proto_file).as_str());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(false)
        .file_descriptor_set_path(out_dir.join("mr_cache.bin"))
        .out_dir("./src/generated")
        .compile(&[out_dir.join("mr_cache.proto")], &[out_dir])
        .expect("Building proto failed");

    Ok(())
}

// Download externally needed proto file
fn download_proto(url: &str, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(path.join("mr_cache.proto"))?;

    file.write_all(&response)?;

    Ok(())
}