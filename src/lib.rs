use std::hash::{Hash, Hasher};

use lccc_siphash::{RawSipHasher, SipHasher};
use proc_macro2::Span;

fn hash_span<H: Hasher>(span: Span, hasher: &mut H) {
    span.byte_range().hash(hasher);
    let st = format!("{span:?}");

    st.hash(hasher);
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

const HASH_SEED_VERSION: &str = env!("HASH_SEED_VERSION");
const HASH_KEY_TARGET: &str = env!("TARGET");

fn seed_token_generator() -> SipHasher<2, 4> {
    let mut seed_gen = SipHasher::<2, 4>::new_with_keys(11717105939243852261, 11816760824499105823);
    VERSION.hash(&mut seed_gen);
    NAME.hash(&mut seed_gen);
    HASH_SEED_VERSION.hash(&mut seed_gen);
    HASH_KEY_TARGET.hash(&mut seed_gen);
    let k0 = seed_gen.finish();
    let vname = std::env::var("CARGO_PKG_NAME").unwrap_or(String::new());
    let vversion = std::env::var("CARGO_PKG_VERSION").unwrap_or(String::new());
    vname.hash(&mut seed_gen);
    vversion.hash(&mut seed_gen);
    let k1 = seed_gen.finish();

    SipHasher::new_with_keys(k0, k1)
}

pub struct RandomSource(SipHasher<2, 4>);

impl RandomSource {
    pub fn new() -> Self {
        Self::with_key_span(Span::mixed_site())
    }

    pub fn with_key_span(span: Span) -> Self {
        let mut hasher = seed_token_generator();
        hash_span(span, &mut hasher);

        Self(hasher)
    }

    pub fn next(&mut self, span: Span) -> u64 {
        hash_span(span, &mut self.0);

        self.0.finish()
    }
}
