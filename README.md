# 🎓 Web3 Certificate Verifier (Soroban)

A blockchain-based smart contract for **verifying the authenticity of certificates or diplomas** using document hashing.
Built with **Rust + Soroban SDK** on the Stellar network.

---

## 🚀 Overview

This application allows institutions (issuers) to:

* Issue digital certificates securely
* Store certificate hashes on-chain
* Enable public verification of document authenticity

🔐 No sensitive data is stored on the blockchain — only **hashed values**.

---

## 🧠 Core Concept

1. A document (e.g., PDF certificate) is hashed (e.g., SHA-256)
2. The hash is submitted to the smart contract
3. The contract stores the hash along with metadata
4. Verification is done by matching the hash

---

## ✨ Features

* ✅ Issue certificate (wallet-based authentication)
* ✅ Verify certificate (public access)
* ✅ Prevent duplicate certificates
* ✅ Retrieve certificate details
* ✅ Get certificates by owner

---

## 🏗️ Tech Stack

* Rust (`no_std`)
* Soroban SDK
* Stellar Blockchain (Testnet)

---

## 📁 Project Structure

```
contracts/
└── inventory/
    ├── src/
    │   ├── lib.rs
    │   └── test.rs
    └── Cargo.toml
```

---

## 🔐 Security Model

* Only the **issuer (wallet address)** can issue certificates
* Every transaction requires a valid signature (`require_auth()`)
* Certificate hashes must be unique (no duplicates allowed)

---

## 📦 Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

Output:

```
target/wasm32-unknown-unknown/release/*.wasm
```

---

## 🌐 Deploy to Testnet

### 1. Configure network

```bash
soroban config network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 2. Generate identity

```bash
soroban config identity generate user1
```

### 3. Fund account

```bash
soroban config identity fund user1 --network testnet
```

### 4. Deploy contract

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/your_contract.wasm \
  --source user1 \
  --network testnet
```

---

## 🧪 Example Usage

### Issue Certificate

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source user1 \
  --network testnet \
  -- issue_certificate \
  --issuer user1 \
  --owner user1 \
  --hash <HASH>
```

---

### Verify Certificate

```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --network testnet \
  -- verify_certificate \
  --hash <HASH>
```

---

## ⚠️ Limitations

* Currently uses `Vec` (not optimal for large-scale data)
* No indexing or mapping storage yet
* No certificate revocation feature

---

## 🔥 Future Improvements

* [ ] O(1) storage using hash-based mapping
* [ ] Certificate revocation
* [ ] Multi-issuer whitelist system
* [ ] IPFS integration for metadata storage
* [ ] Full frontend dApp (React + wallet integration)

---

## 📌 Best Practices

* Do not store raw documents on-chain
* Perform hashing on the frontend
* Always validate ownership using wallet authentication

---

## 👨‍💻 Author

Developed as a Web3 learning and portfolio project using Soroban smart contracts.

---

## 📜 License

MIT License
