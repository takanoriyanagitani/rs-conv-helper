use std::io;

fn main() -> Result<(), io::Error> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                "convhelper/proto/conv/v1/cvt.proto",
                "convhelper/proto/uuid/v1/u.proto",
                "convhelper/proto/conv/v1/bulk.proto",
            ],
            &["rs-conv-helper-proto"],
        )?;
    Ok(())
}
