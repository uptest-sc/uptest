/// automatically generate
use rand::Rng; // https://rust-random.github.io/book/guide-rngs.html

#[cfg(feature = "subxthelper")]
use subxt_signer::sr25519::Keypair;

#[cfg(feature = "subxthelper")]
use subxt::utils::AccountId32;

/// auto generate inputs for tests
pub struct InputHelper {}

impl InputHelper {
    pub fn get_boolean() -> bool {
        rand::random()
    }
    /// get a random i32
    pub fn get_i32() -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen::<i32>()
    }
    /// get a random u8
    pub fn get_u8() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen::<u8>()
    }
    /// get a random u32
    pub fn get_u32() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen::<u32>()
    }
    /// get a random u64
    pub fn get_u64() -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen::<u64>()
    }
    /// get a random u128
    pub fn get_u128() -> u128 {
        let mut rng = rand::thread_rng();
        rng.gen::<u128>()
    }

    /// get a random f32
    pub fn get_f32() -> f32 {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen();
        x
    }

    /// get a random f64
    pub fn get_f64() -> f64 {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen();
        x
    }
    /// generate a random accountid32 with subxt_signer
    #[cfg(feature = "subxthelper")]
    pub fn get_address() -> AccountId32 {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 32] = rng.gen();
        let seckey = Keypair::from_seed(random_bytes).expect("Could not generate key");
        let key2: AccountId32 = seckey.public_key().to_account_id();
        key2
    }
}
