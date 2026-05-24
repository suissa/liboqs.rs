//! example_sig.rs
//!
//! Tradução idiomática de `tests/example_sig.c` para Rust, mantendo os mesmos
//! nomes de funções/variáveis onde possível.

use oqs::sig::{Algorithm, Sig};
use oqs::Oqs;

const MESSAGE_LEN: usize = 50;

type OQS_STATUS = Result<(), oqs::Error>;

fn cleanup_stack(secret_key: &mut [u8]) {
    secret_key.fill(0);
}

fn cleanup_heap(secret_key: &mut [u8]) {
    // Em Rust, Vec já gerencia o free; aqui fazemos apenas limpeza segura.
    secret_key.fill(0);
}

fn example_stack() -> OQS_STATUS {
    let sigalg = Sig::new(Algorithm::MlDsa65)?;

    let mut public_key = vec![0u8; sigalg.length_public_key()];
    let mut secret_key = vec![0u8; sigalg.length_secret_key()];
    let mut message = [0u8; MESSAGE_LEN];

    getrandom::fill(&mut message)?;

    sigalg.keypair_into(&mut public_key, &mut secret_key)?;
    let signature = sigalg.sign(&message, &secret_key)?;
    sigalg.verify(&message, &signature, &public_key)?;

    println!("[example_stack] OQS_SIG_ml_dsa_65 operations completed.");
    cleanup_stack(&mut secret_key);
    Ok(())
}

fn example_heap() -> OQS_STATUS {
    let sig = Sig::new(Algorithm::MlDsa65)?;

    let mut public_key = vec![0u8; sig.length_public_key()];
    let mut secret_key = vec![0u8; sig.length_secret_key()];
    let mut message = vec![0u8; MESSAGE_LEN];

    getrandom::fill(&mut message)?;

    sig.keypair_into(&mut public_key, &mut secret_key)?;
    let signature = sig.sign(&message, &secret_key)?;
    sig.verify(&message, &signature, &public_key)?;

    println!("[example_heap] OQS_SIG_ml_dsa_65 operations completed.");
    cleanup_heap(&mut secret_key);
    Ok(())
}

fn main() {
    let _oqs = Oqs::init();

    let rc = example_stack().and_then(|_| example_heap());
    if rc.is_ok() {
        std::process::exit(0);
    }
    std::process::exit(1);
}
