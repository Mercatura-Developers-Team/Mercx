# `MercX Platform Overview `

# Introduction
MercX is an innovative blockchain platform designed for the issuance and management of Real World Assets (RWA) tokens, focusing on shares in the EGX30 index. Operating on the Internet Computer Protocol (ICP) using the Rust SDK, MercX integrates the robustness of real-world finance with the flexibility of blockchain technology, reshaping asset trading and management.
Key Features
# Real-Time Settlement
MercX streamlines the settlement process, reducing the traditional 24-72 hour timeframe to instantaneous transactions. This acceleration supports capital efficiency and allows for continuous trading around the clock, empowering participants to leverage market opportunities as they arise.
# Enhanced Accessibility
The platform democratizes financial participation by enabling fractional ownership, which allows small savers to engage in investment opportunities traditionally reserved for larger investors through manageable investment sizes and diversified portfolios.
# Dual Utility of MercX Tokens
-	Transaction Fees: Tokens are used to facilitate various transaction fees, ensuring efficient operational flows within the platform.
-	Medium for Asset Exchange: Tokens may also be used as a medium of exchange, enhancing their utility and value within the MercX ecosystem.
-	24-Hour Redemption: Features a robust infrastructure that supports 24-hour redemption, providing token holders with exceptional liquidity and flexibility.
Regulatory Compliance
MercX adheres to regulations set by the Financial Regulatory Authority of Egypt, ensuring a secure, transparent, and trustworthy environment for all users.
Fund Issuances
-	Venture Capital Fund: The initial issuance, focused on Web3 and digital assets, is projected at $15 million USD. This venture capital fund targets emerging technologies and represents a pioneering effort to bridge innovative startups with traditional capital markets.
  -Exchange-Traded Fund (ETF): Following the venture capital fund, an ETF with a target of $50 million USD is planned within the first year. This fund aims to broaden investment opportunities and enhance market accessibility.
Target Audience
-	Fund Managers: Seek innovative asset management and distribution technologies.
-	End Users: Individuals looking to invest, trade, or leverage their assets in a regulated and dynamic marketplace.
Technical Architecture
Built on the Internet Computer Protocol with the Rust SDK, MercX provides a secure, scalable, and decentralized platform capable of handling high transaction volumes with advanced security measures essential for financial operations.
Business Impact
MercX revolutionizes asset trading by facilitating faster transactions, providing access to a wider range of investors through tokenization, and integrating cutting-edge fund management technologies into the financial sector.


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
