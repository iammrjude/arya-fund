# AryaFund — Frontend

The React frontend for AryaFund, a decentralized crowdfunding dApp built on the Stellar network.

---

## Live Demo

> <https://arya-crowdfund.vercel.app>

---

## Screenshots

> _coming soon_

---

## Tech Stack

| Tool | Purpose |
| ------ | --------- |
| React 19 + Vite | UI framework and build tool |
| React Router v7 | Client-side routing |
| `@stellar/stellar-sdk` | Soroban contract interaction |
| `@creit-tech/stellar-wallets-kit` | Multi-wallet connection (Freighter, Albedo, xBull, Rabet, LOBSTR) |
| CSS Modules | Component-scoped styling |
| Inter + JetBrains Mono | Typography |

---

## Contract

| Property | Value |
| ---------- | ------- |
| **Network** | Stellar Testnet |
| **Contract Address** | `CD5LOATI5SDME7GQXRBVSZIG3DZL4NRYD4663GM7PLPY252L2RGPOFTL` |
| **Deploy TX** | `95478ead278154ae67b279cdce1492715f2e37079d5ed41253710dbc017e2ab6` |

---

## Pages

| Route | Description |
| ------- | ------------- |
| `/` | Browse all campaigns with status filters |
| `/campaign/:id` | Campaign detail, donate, claim refund |
| `/create` | Create a new campaign |
| `/dashboard` | Manage your own campaigns (withdraw, extend, mark failed) |
| `/admin` | Platform owner settings (fee, treasury, action window) |

---

## Getting Started

### Prerequisites

- Node.js `v22+`
- A Stellar wallet extension (Freighter recommended for testnet)

### Install

```bash
npm install
```

### Run locally

```bash
npm run dev
```

Opens at `http://localhost:5173`

### Build for production

```bash
npm run build
```

---

## Environment

No `.env` file needed. All configuration is in `src/contract/config.js`:

```js
export const CONTRACT_ID = 'CD5LOATI5SDME7GQXRBVSZIG3DZL4NRYD4663GM7PLPY252L2RGPOFTL'
export const NATIVE_TOKEN_ID = 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC'
export const NETWORK_PASSPHRASE = 'Test SDF Network ; September 2015'
export const RPC_URL = 'https://soroban-testnet.stellar.org'
export const PLATFORM_OWNER = 'GBLH7QUEY43J3AJSIYPRUQKKUFX577GSYWRRQJVNFOV7MUON3YMM5IJQ'
export const READ_ACCOUNT = 'GBLH7QUEY43J3AJSIYPRUQKKUFX577GSYWRRQJVNFOV7MUON3YMM5IJQ'
```

---

## Project Structure

```text
src/
├── contract/
│   ├── config.js          # Contract address, network config
│   └── client.js          # All contract function calls
│
├── hooks/
│   ├── useWallet.js       # getAddress, signTransaction
│   └── useContract.js     # useCampaigns, useCampaign, usePlatformSettings
│
├── utils/
│   ├── stellar.js         # truncateAddress, explorerUrl
│   ├── format.js          # XLM/stroops conversion, fee calculation
│   └── time.js            # Unix timestamps, countdown, action window
│
├── components/
│   ├── Header/
│   ├── Footer/
│   ├── CampaignCard/
│   ├── ProgressBar/
│   ├── CountdownTimer/
│   ├── StatusBadge/
│   ├── TxStatus/
│   └── ConnectPrompt/
│
└── pages/
    ├── Home/
    ├── Campaign/
    ├── Create/
    ├── Dashboard/
    └── Admin/
```

---

## Features

- Browse campaigns filtered by status (Active, Successful, Failed)
- Create campaigns with goal, deadline and extension days
- Donate XLM to active campaigns
- Auto-refund for failed campaigns — donors claim directly
- Organizer dashboard — withdraw funds, extend deadline, mark as failed
- Platform admin panel — update fee, treasury wallet, action window
- Real-time countdown timers and progress bars
- Transaction status feedback with Stellar Explorer links
- Multi-wallet support via Stellar Wallets Kit

---

## Related

- [Smart Contract README](../contract/README.md)
- [Stellar Wallets Kit](https://github.com/Creit-Tech/Stellar-Wallets-Kit)
- [Soroban Docs](https://soroban.stellar.org)
- [Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet)
