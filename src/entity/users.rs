use argon2::{ThreadMode, Variant, Config, Version};
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize};
use rand::distributions::{Alphanumeric, DistString};

pub const ARGON2_CONFIG: argon2::Config<'_> = Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 1024,
    time_cost: 10,
    lanes: 4,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 32,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub img_url: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub integrations: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn encrypt(self: &mut Self, password_hash: String) {
        let salt: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let hash: String = argon2::hash_encoded(password_hash.trim().as_bytes(), 
            &salt.as_bytes(), &ARGON2_CONFIG).unwrap();  

        self.salt = Set(salt);
        self.password_hash = Set(hash);
    }
}