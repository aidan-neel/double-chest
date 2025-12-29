fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &["../proto/user.proto", "../proto/upload.proto", "../proto/token.proto"], 
            &["../proto"],
        )?;
    Ok(())
}