# MercX Platform Overview

## Introduction

**MercX** is a decentralized tokenization platform built on the Internet Computer Protocol (ICP), designed to bring Real World Assets (RWA) to the blockchain. Starting with tokenized exposure to Egypt's top 30 companies via the EGX30 index, MercX bridges traditional finance and DeFi by enabling permissioned, compliant, and fractional asset ownership.

Built with the Rust SDK, MercX integrates multiple ICRC-compliant canisters to manage token issuance, swap logic, and real-time trading across various pools, with seamless frontend integration. The platform is aligned with regulatory standards defined by the Egyptian Financial Regulatory Authority (FRA).

---

## 🧩 Problem Statement

Egypt and the broader MENA region suffer from outdated capital market infrastructure that limits investor participation, fund liquidity, and product innovation. Retail and institutional investors alike face high entry barriers, manual onboarding, slow fund redemption, and no access to real-time price data or digital trading functions.

Fund managers spend up to half their management fees on intermediaries just to handle subscriptions and redemptions. There are no tools to borrow against fund shares, no digital liquidity pools, and no integration with smart analytics or decision-making tools. As a result, capital efficiency is low, operational costs are high, and investor experience is broken.

---

## 🚀 MercX: A Digital Infrastructure to Revitalize MENA Capital Markets

MercX offers a fully compliant platform that tokenizes regulated financial assets such as mutual funds, treasury instruments, and equities. It introduces modern digital financial infrastructure tailored to the needs of Egypt and MENA, including:

- **Real-time Price Feeds** to track tokenized asset performance  
- **Swap Functions** to enable instant, peer-to-peer exchange of fund shares  
- **Built-in Liquidity Pools** to absorb redemptions and reduce pressure on fund managers  
- **AI Agent Advisors** to assist users in understanding investment options and market conditions  
- **Business Analytics Dashboards** for both investors and fund managers to make informed decisions  
- **Borrowing and Lending Systems** that unlock capital efficiency by enabling collateralized loans on top of fund shares  

MercX makes institutional-grade products accessible to retail investors starting from $100. With automated onboarding, instant trading, and deep integration with Egypt’s regulatory framework, MercX is not just digitizing capital markets; it is rebuilding them for the future.

> This is the infrastructure upgrade MENA has been waiting for.

---

## Key Features

### Real-Time Settlement

Transactions on the MercX platform are settled instantaneously, allowing for seamless asset management and trading without traditional banking delays.

### Enhanced Accessibility

By tokenizing assets, MercX enables fractional ownership, making it possible for small investors to participate in investment opportunities traditionally reserved for larger capital bases.

### Regulatory Compliance

MercX adheres to regulations set by the Financial Regulatory Authority of Egypt, ensuring a secure, transparent, and trustworthy environment for all users.

## Business Impact

MercX revolutionizes asset trading by facilitating faster transactions, providing access to a wider range of investors through tokenization, and integrating cutting-edge fund management technologies into the financial sector.

## Technical Architecture

Built on the Internet Computer Protocol with the Rust SDK, MercX provides a secure, scalable, and decentralized platform capable of handling high transaction volumes with advanced security measures essential for financial operations.

### MercX Backend Canister
- **Candid Path**: `src/mercx_backend/mercx_backend.did`
- **Type**: Rust
- **Description**:  Serves as the core business logic hub for the MercX platform. It is responsible for managing the lifecycle of asset tokenization and ensures the secure and efficient processing of transactions within the platform. Key functionalities include handling buy,sell,token swaps, maintaining ledger balances, and interfacing with other canisters such as the ICRC1 Ledger and ICRC1 Index canisters for comprehensive asset management.

### XRC Canister
- **Candid Path**: `xrc/xrc.did`
- **Description**:  Exchange Rate Canister calculates exchange rates using a unique aggregation method and it's  essential for executing transactions on the platform, ensuring that all trades are conducted at fair and accurate market rates.

### ICRC1 Ledger Canister
- **Candid Path**: Remote URL to Candid interface
- **Description**: Implements  BELLA token standard, managing the lifecycle of tokens including issuance, transfer, and balance tracking. This canister is crucial for ensuring compliance with the ICRC1 standard across the MercX platform.

### ICRC1 Index Canister
- **Candid Path**: Remote URL to Candid interface
- **Description**: Provides indexing and query functionalities for BELLA token transactions, enhancing the accessibility and auditability of token operations within the MercX ecosystem.

### Tommy ICRC1 Ledger and Index Canisters
- **Description**: Similar to the ICRC1 ledger and index canisters but specific to the "Tommy" token, providing dedicated management and indexing for transactions involving this particular token.

### ICP Ledger Canister
- **Candid Path**: Remote URL to Candid interface
- **Description**: Manages transactions involving ICP tokens. It facilitates operations like token transfers, balance checks, and transaction logging on the network.

### ICP Index Canister
- **Description**: Provides indexing and querying capabilities for transactions involving ICP tokens, which is essential for operational transparency and efficiency in handling ICP-related activities on the platform.

### Internet Identity
- **Candid Path**: Remote URL to Candid interface
- **Description**: Provides decentralized identity verification services, allowing users to authenticate securely without relying on traditional centralized identity providers. This canister supports user privacy and security across the MercX platform.

### MercX Frontend
- **Source**: Location of the frontend distribution files
- **Type**: Assets
- **Description**: Delivers the user interface directly from the Internet Computer, ensuring that users interact with a responsive and secure frontend. It connects seamlessly to the backend and other canisters to provide a comprehensive user experience.



To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with `mercx`, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd mercx/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor
