# 🔐 Encrypted Media Locker

**A decentralized vault for your digital files — built with Rust, React, and Solana.**

---

## 🧠 Overview

**Encrypted Media Locker** is a privacy-focused, decentralized platform that lets you **upload, encrypt, and securely share** any kind of file — audio, video, documents, and more.

Unlike traditional cloud storage, your files are encrypted **locally** and stored **decentrally** on networks like **IPFS**, while metadata and access control are managed on **Solana**.

You stay in full control — no central servers, no intermediaries, no data mining.

---

## 🌍 Key Features

- 🔒 **End-to-End Encryption**  
  Files are encrypted on your device before upload using AES encryption.

- 🌐 **Decentralized Storage**  
  Files are stored across decentralized networks (IPFS / Arweave).

- 🪶 **On-Chain Metadata (Solana)**  
  File metadata and sharing permissions are securely stored on-chain.

- 👥 **Access Control**  
  Grant access to others by encrypting your file’s key with their public key.

- 💻 **Cross-Platform UI**  
  Built with React for a smooth, modern user experience.

- ⚡ **Powered by Rust**  
  The backend leverages Rust’s speed and safety for encryption and storage coordination.

---

## 🏗️ Tech Stack

| Layer | Technology | Description |
|-------|-------------|--------------|
| Backend | 🦀 Rust (Actix-Web) | Handles encryption, uploads, and API logic |
| Frontend | ⚛️ React (Vite) | User interface for uploads, sharing, and access |
| Blockchain | 🪙 Solana | Stores file metadata and permissions |
| Storage | 📦 IPFS | Decentralized storage for encrypted file chunks |
| Encryption | AES + RSA Hybrid | AES for files, RSA for key sharing |

---

## 🔑 How It Works

1. **Upload a File**  
   The file is encrypted locally using AES.

2. **Store Encrypted Data**  
   The encrypted file is uploaded to IPFS or Arweave.

3. **Record Metadata**  
   The file’s CID and ownership details are stored on Solana.

4. **Grant Access**  
   You can encrypt the AES key with another user’s public key to securely share access.

---