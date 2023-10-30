use std::io;

fn main() -> Result<(), io::Error> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "convhelper/proto/conv/v1/cvt.proto",
                "convhelper/proto/uuid/v1/u.proto",
            ],
            &["rs-conv-helper-proto"],
        )?;
    Ok(())
}
