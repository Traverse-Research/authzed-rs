#[cfg(feature = "v0")]
pub mod v0 {
    include!("gen/authzed.api.v0.rs");
}

#[cfg(feature = "v1")]
pub mod v1 {
    include!("gen/authzed.api.v1.rs");
}

#[cfg(feature = "v1alpha1")]
pub mod v1alpha1 {
    include!("gen/authzed.api.v1alpha1.rs");
}
