# 🚀 Influencer Marketplace (Soroban Smart Contract)

## 📌 Project Description

Influencer Marketplace is a decentralized platform built on the **Stellar Soroban smart contract framework** that connects brands with influencers in a trustless and transparent environment.

It eliminates intermediaries by allowing brands to create campaigns and influencers to directly apply, ensuring secure collaboration using blockchain technology.

---

## ⚙️ What it does

* Allows influencers to register with their niche/category
* Enables brands to create marketing campaigns with budgets
* Lets influencers apply to campaigns
* Allows brands to select influencers for collaboration
* Stores all interactions on-chain for transparency

---

## ✨ Features

* 🔐 **Decentralized System** – No middlemen, fully trustless
* 👤 **Influencer Registration** – Register with niche (tech, fitness, etc.)
* 📢 **Campaign Creation** – Brands can launch campaigns
* 📥 **Application System** – Influencers apply easily
* ✅ **Selection Mechanism** – Brands choose influencers
* 📊 **On-chain Storage** – Transparent & immutable data
* ⚡ **Fast Execution** – Powered by Soroban smart contracts

---

## 🛠 Tech Stack

* **Blockchain:** Stellar (Soroban)
* **Smart Contract:** Rust (soroban-sdk)
* **Backend Logic:** On-chain storage
* **Frontend (optional):** React / Next.js

---

## 🧠 How It Works

1. Influencer registers with niche
2. Brand creates a campaign with budget
3. Influencers apply to the campaign
4. Brand selects an influencer
5. Selection is stored on-chain

---

## 🧪 Getting Started

### Prerequisites

* Rust installed
* Soroban CLI installed
* Stellar testnet account

### Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deploy Contract

contract address : CBV7TQJY53AFTY3XGIQDPIC4XK2DN65FYVFAFE5H4X4ZQJHGFWX3QTYB
<img width="1916" height="904" alt="Screenshot 2026-03-27 133659" src="https://github.com/user-attachments/assets/610e0689-82a9-4c99-9835-77137d72723a" />


```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/your_contract.wasm \
  --source your-account
```

---

## 📂 Project Structure

```
├── src/
│   └── lib.rs        # Smart contract logic
├── Cargo.toml        # Dependencies
└── README.md         # Documentation
```

---

## 🔮 Future Improvements

* 💰 Escrow payment system
* ⭐ Influencer rating & reviews
* 📅 Campaign deadlines
* 🎯 AI-based influencer matching
* 🌐 Full frontend dashboard

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork this repo and submit a pull request.

---

## 📄 License

MIT License

---

## 💡 Inspiration

Built to solve inefficiencies in traditional influencer marketing by leveraging decentralization, transparency, and automation.
