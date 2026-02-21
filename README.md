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
| **Contract Address** | `CDX4KDWSCBCD7JU6NVDGEJ47NWCTMG7DM43ATJKQA52UPLGZXXQ6VRRW` |
| **Network** | Stellar Testnet |
| **Platform Fee** | 2.5% on successful withdrawals |
| **Action Window** | 7 days |
| **Deploy TX** | `875794a81462c12c94f312ab8256c030d4c2ef17e300e90d193849d927d531c5` |
| **Init TX** | `c39d4ecea886359c4348142d735d81b33912c551520e2f3aac657bda34887db9` |

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

- [Stellar Expert — Contract](https://stellar.expert/explorer/testnet/contract/CDX4KDWSCBCD7JU6NVDGEJ47NWCTMG7DM43ATJKQA52UPLGZXXQ6VRRW)
- [Contract Explorer — Contract info, Invoke contract](https://lab.stellar.org/r/testnet/contract/CDX4KDWSCBCD7JU6NVDGEJ47NWCTMG7DM43ATJKQA52UPLGZXXQ6VRRW)
- [Stellar Expert — Deploy TX](https://stellar.expert/explorer/testnet/tx/875794a81462c12c94f312ab8256c030d4c2ef17e300e90d193849d927d531c5)
- [Soroban Docs](https://soroban.stellar.org)
- [Stellar Wallets Kit](https://github.com/Creit-Tech/Stellar-Wallets-Kit)
- [Attestation Verification Flow](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0055.md#attestation-verification-flow)
