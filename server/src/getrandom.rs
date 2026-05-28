//! SpacetimeDB modules target `wasm32-unknown-unknown`, which has no OS RNG.
//! `spacetimedb` enables `getrandom`'s `custom` feature so accidental use of
//! `rand::thread_rng()` fails loudly unless we provide this hook.
//!
//! Prefer [`ReducerContext::rng`] for reducer randomness (see `generate_username`).

use core::sync::atomic::{AtomicU64, Ordering};

use getrandom::{register_custom_getrandom, Error};

static SEED: AtomicU64 = AtomicU64::new(0x853c49e6748fea9b);

fn spacetime_getrandom(buf: &mut [u8]) -> Result<(), Error> {
    let mut state = SEED.load(Ordering::Relaxed);
    for byte in buf.iter_mut() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        *byte = state as u8;
    }
    SEED.store(state, Ordering::Relaxed);
    Ok(())
}

register_custom_getrandom!(spacetime_getrandom);
