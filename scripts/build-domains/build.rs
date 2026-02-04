use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/proto/extensions.proto", "src/proto/common.proto"],
        &["src/proto/"],
    )?;
    Ok(())
}
