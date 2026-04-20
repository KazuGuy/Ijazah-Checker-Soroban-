#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Symbol,
    Address, BytesN, Vec
};

// =====================
// DATA STRUCTURE
// =====================
#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub hash: BytesN<32>,
    pub owner: Address,
    pub issuer: Address,
    pub issued_at: u64,
}

// =====================
// STORAGE KEY
// =====================
const CERTS: Symbol = symbol_short!("CERTS");

// =====================
// CONTRACT
// =====================
#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {

    // =====================
    // ISSUE CERTIFICATE
    // =====================
    pub fn issue_certificate(
        env: Env,
        issuer: Address,
        owner: Address,
        hash: BytesN<32>,
    ) -> Symbol {

        // 🔐 issuer harus sign
        issuer.require_auth();

        let mut certs: Vec<Certificate> = env.storage()
            .instance()
            .get(&CERTS)
            .unwrap_or(Vec::new(&env));

        // mencegah hash yang duplicating
        for i in 0..certs.len() {
            let c = certs.get(i).unwrap();
            if c.hash == hash {
                return symbol_short!("EXISTS");
            }
        }

        let cert = Certificate {
            hash,
            owner,
            issuer: issuer.clone(),
            issued_at: env.ledger().timestamp(),
        };

        certs.push_back(cert);

        env.storage().instance().set(&CERTS, &certs);

        symbol_short!("ISSUED")
    }

    // =====================
    // Verifikasi Certificate
    // =====================
    pub fn verify_certificate(env: Env, hash: BytesN<32>) -> bool {

        let certs: Vec<Certificate> = env.storage()
            .instance()
            .get(&CERTS)
            .unwrap_or(Vec::new(&env));

        for i in 0..certs.len() {
            if certs.get(i).unwrap().hash == hash {
                return true;
            }
        }

        false
    }

    // =====================
    // GET CERTIFICATE DETAIL
    // =====================
    pub fn get_certificate(env: Env, hash: BytesN<32>) -> Option<Certificate> {

        let certs: Vec<Certificate> = env.storage()
            .instance()
            .get(&CERTS)
            .unwrap_or(Vec::new(&env));

        for i in 0..certs.len() {
            let cert = certs.get(i).unwrap();

            if cert.hash == hash {
                return Some(cert);
            }
        }

        None
    }

    // =====================
    // GET CERT BY OWNER
    // =====================
    pub fn get_my_certificates(env: Env, user: Address) -> Vec<Certificate> {

        let certs: Vec<Certificate> = env.storage()
            .instance()
            .get(&CERTS)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..certs.len() {
            let cert = certs.get(i).unwrap();

            if cert.owner == user {
                result.push_back(cert);
            }
        }

        result
    }
}