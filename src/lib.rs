pub mod rpc {
    pub mod convhelper {
        pub mod proto {
            pub mod uuid {
                pub mod v1 {
                    tonic::include_proto!("convhelper.proto.uuid.v1");
                }
            }

            pub mod conv {
                pub mod v1 {
                    tonic::include_proto!("convhelper.proto.conv.v1");
                }
            }
        }
    }
}
