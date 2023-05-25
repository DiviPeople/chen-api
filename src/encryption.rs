use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::{distributions::Alphanumeric, Rng}; // 0.8
use std::env;

pub fn encrypt(mut pass: String) -> (String, String) {
    const CONFIG: argon2::Config<'_> = Config {
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

    let mut salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            pass = args[1].to_owned();
        }
        if args.len() > 2 {
            salt = args[2].to_owned();
        }

    let pass_bytes = pass.trim().as_bytes();
    let salt_bytes = salt.as_bytes();

    (argon2::hash_encoded(pass_bytes, &salt_bytes, &CONFIG).unwrap(), salt)
}
