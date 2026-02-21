# AryaFund

A decentralized crowdfunding dApp built on the Stellar network. Anyone can create campaigns, collect XLM donations, and receive automatic refunds if goals are not met — all enforced by smart contracts with no middleman.

- **Network:** Stellar Testnet
- **Live Demo:** <https://arya-crowdfund.vercel.app>

---

## Screenshots

> _coming soon_

---

## How It Works

1. **Create** a campaign with a goal, deadline, and optional extension days
2. **Donate** XLM to any active campaign
3. **Goal met** → organizer withdraws funds (2.5% platform fee deducted)
4. **Goal not met** → campaign fails, donors claim full refunds automatically

### The 70% Rule

If a campaign raises 70%+ but misses its deadline, the organizer gets a 7-day action window to either extend the deadline (one time) or mark the campaign as failed. If they do nothing, it auto-fails and donors can claim refunds.

---

## Repository Structure

```text
arya-fund/
├── contract/     # Soroban smart contract (Rust)
│   ├── contracts/
│   │   └── arya_fund/
│   │       ├── src/
│   │       │   ├── lib.rs      # 17 exported functions
│   │       │   └── test.rs     # 12 unit tests
│   │       └── Cargo.toml
│   ├── Cargo.toml
│   └── README.md
│
└── frontend/     # React frontend
    ├── src/
    │   ├── contract/       # Contract client and config
    │   ├── hooks/          # useWallet, useContract
    │   ├── utils/          # Formatting, time, stellar helpers
    │   ├── components/     # Reusable UI components
    │   └── pages/          # Home, Campaign, Create, Dashboard, Admin
    ├── package.json
    └── README.md
```

---

## Contract

| Property | Value |
| ---------- | ------- |
| **Contract Address** | `CD5LOATI5SDME7GQXRBVSZIG3DZL4NRYD4663GM7PLPY252L2RGPOFTL` |
| **Network** | Stellar Testnet |
| **Platform Fee** | 2.5% on successful withdrawals |
| **Action Window** | 7 days |
| **Deploy TX** | `95478ead278154ae67b279cdce1492715f2e37079d5ed41253710dbc017e2ab6` |
| **Init TX** | `8e29274c189a60e436da4d2c8aa807a3472ecc7584ad69a6891286312b69e64b` |

---

## Tech Stack

### Smart Contract

- **Rust** + Soroban SDK
- Stellar CLI v25.1.0
- `wasm32v1-none` target

### Frontend

- React 19 + Vite
- React Router v7
- `@stellar/stellar-sdk` v14
- `@creit-tech/stellar-wallets-kit` v2 (Freighter, Albedo, xBull, Rabet, LOBSTR)
- CSS Modules

---

## Getting Started

### Run the Smart Contract

```bash
cd contract

# Build
stellar contract build

# Test
cargo test --manifest-path=contracts/arya_fund/Cargo.toml
```

See [contract/README.md](contract/README.md) for full deployment instructions.

### Run the Frontend

```bash
cd frontend

# Install
npm install

# Run locally
npm run dev
```

Opens at `http://localhost:5173`

See [frontend/README.md](frontend/README.md) for full setup details.

---

## Features

- [x] Create trustless crowdfunding campaigns on Stellar
- [x] Donate XLM to any active campaign
- [x] Smart contract enforced refunds on failed campaigns
- [x] Organizer dashboard — withdraw, extend deadline, mark failed
- [x] Platform admin panel — fee, treasury, action window management
- [x] Multi-wallet support via Stellar Wallets Kit
- [x] Real-time countdown timers and progress bars
- [x] Transaction status feedback with Stellar Explorer links

---

## Security Design

- Funds held by contract, never by the organizer
- Pull refunds — donors claim themselves, no double claiming possible
- Platform owner and treasury are separate wallets
- Every write function requires `.require_auth()` from the appropriate party
- All funding rules enforced on-chain

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

To report a bug or request a feature, [open an issue](../../issues/new/choose).

---

## Links

- [Stellar Expert — Contract](https://stellar.expert/explorer/testnet/contract/CD5LOATI5SDME7GQXRBVSZIG3DZL4NRYD4663GM7PLPY252L2RGPOFTL)
- [Contract Explorer — Contract info, Invoke contract](https://lab.stellar.org/r/testnet/contract/CD5LOATI5SDME7GQXRBVSZIG3DZL4NRYD4663GM7PLPY252L2RGPOFTL)
- [Stellar Expert — Deploy TX](https://stellar.expert/explorer/testnet/tx/95478ead278154ae67b279cdce1492715f2e37079d5ed41253710dbc017e2ab6)
- [Stellar Expert — Init TX](https://stellar.expert/explorer/testnet/tx/8e29274c189a60e436da4d2c8aa807a3472ecc7584ad69a6891286312b69e64b)
- [Soroban Docs](https://soroban.stellar.org)
- [Stellar Wallets Kit](https://github.com/Creit-Tech/Stellar-Wallets-Kit)
- [Attestation Verification Flow](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0055.md#attestation-verification-flow)
