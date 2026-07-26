use serde::Deserialize;

#[derive(Deserialize)]
pub struct RepositoryRef {
    pub full_name: String,
}
