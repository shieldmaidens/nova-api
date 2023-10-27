// @generated
pub mod google {
    #[cfg(feature = "google-rpc")]
    // @@protoc_insertion_point(attribute:google.rpc)
    pub mod rpc {
        include!("google.rpc.rs");
        // @@protoc_insertion_point(google.rpc)
    }
}
pub mod keystore {
    #[cfg(feature = "keystore-v1")]
    // @@protoc_insertion_point(attribute:keystore.v1)
    pub mod v1 {
        include!("keystore.v1.rs");
        // @@protoc_insertion_point(keystore.v1)
    }
}
pub mod raft {
    #[cfg(feature = "raft-v1")]
    // @@protoc_insertion_point(attribute:raft.v1)
    pub mod v1 {
        include!("raft.v1.rs");
        // @@protoc_insertion_point(raft.v1)
    }
}