Decentralized Expense Tracker
Project Title
Decentralized Expense Tracker (DApp Quản Lý Chi Tiêu Hằng Ngày)

Project Description
The Decentralized Expense Tracker is a smart contract platform designed to help users log, categorize, and manage their daily expenses. Built using Soroban on the Stellar blockchain, it provides a transparent, immutable, and secure way to track personal finances without relying on centralized databases or third-party servers.

Project Vision
Our vision is to empower individuals to take full control of their personal finances through a trustless and decentralized ledger. By recording spending habits on-chain, users are guaranteed that their financial data is securely stored, easily auditable, and completely immutable.

Key Features
Expense Logging: Users can securely record daily expenses, specifying categories (e.g., Food, Transport) and exact amounts.

Data Retrieval: Quickly query the most recent expense categories or access historical spending data.

Immutable Records: All financial entries are permanently recorded on the Stellar blockchain, preventing any data tampering or loss.

Decentralized Ownership: Users retain complete ownership and control over their financial data through their own cryptographic wallets.

Usage Instructions
Deploy: Initialize the smart contract on the Stellar Testnet.

Add Expense: Users interact with the add_expense function, providing a category (String) and amount (u32) to record a new transaction.

Query Data: Call the get_latest_category function to retrieve the most recently logged expense category for quick review.

Wallet Integration: Transactions are securely signed and authorized using the Freighter wallet.

Future Scope
Cross-Platform Mobile Interface: Build a seamless mobile frontend using Flutter to allow users to log expenses easily on the go.

Persistent On-Chain Storage: Upgrade the contract to map multiple expenses to specific user addresses using Soroban's persistent storage.

Budget Limits & Alerts: Implement logic to set monthly spending limits and trigger on-chain events when budgets are exceeded.

Data Visualization: Create user dashboards to display categorized charts and monthly spending reports.

Multi-currency Support: Allow users to log expenses in various tokenized assets or fiat equivalents available on the Stellar network.

Technology Stack
Rust & Soroban SDK (v25): For highly secure and efficient smart contract development.

Stellar Blockchain: For fast, low-cost, and immutable state management.

Flutter: For building a responsive, multi-platform mobile user interface.

Freighter Wallet: For secure cryptographic signing and user authentication.

Contribution
Community contributions are welcomed from blockchain developers and personal finance enthusiasts. Fork the repository and submit pull requests to assist in further development.

License
This project is licensed under the MIT License.

Contract Detail
ID: CDM7DT633DRVG5O4ZH36E2DBFCXWV5NPJB5Y3WPRR7SEEWSIUVEKDIGL
(Deployed on Stellar Testnet)