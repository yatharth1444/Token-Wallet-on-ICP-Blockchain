 Token Wallet on the ICP Blockchain

Overview

This project is a token wallet built on the **Internet Computer Protocol (ICP)** using **Rust** for the backend and **React** for the frontend. The wallet supports basic functionality like sending, receiving tokens (IRCRC2), and displaying balances.

This solution demonstrates how to create a secure token wallet on ICP and interacts with smart contracts to transfer tokens between users. The frontend is built using React, styled with modular CSS, and is connected to the ICP backend using the DFX SDK.

---

Features

- Token Transfer: Send and receive tokens on the ICP blockchain.
- Balance Display: View the current token balance for a user.
- Secure: Implements secure arithmetic operations for safe token transfer.
- React Frontend: Built with React for easy use and interaction with the wallet.

---

Technologies

- Rust for backend smart contract development.
- React for frontend development.
- ICP (Internet Computer Protocol)** for blockchain infrastructure.
- DFX  for local development and deployment of ICP canisters.
- Candid for interface definition between frontend and backend canisters.

---

 Project Structure

 Backend (`ipc_token_wallet_backend`)

- **Rust Smart Contract**: Handles the logic for sending and receiving tokens, token balance, and arithmetic operations.
- **State Management**: Uses a mutable state to keep track of token balances for each user.
- **Security**: Implements safe arithmetic operations to prevent overflows and underflows.

 Frontend (`ipc_token_wallet_frontend`)

- **React Components**:
  - `Balance`: Displays the user's current token balance.
  - `SendTokens`: Form to send tokens to another user.
  - `ReceiveTokens`: Form to receive tokens from another user.
- Styling: Modular CSS for each component to ensure a clean and responsive UI.

---

 Setup Instructions

 Prerequisites

- Rust: Ensure you have Rust installed. You can install it by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- DFX: The DFX SDK is required to deploy the backend canisters to ICP. Install it by following the instructions at [https://smartcontracts.org/docs/install/dfx](https://smartcontracts.org/docs/install/dfx).
- Node.js and npm: Ensure you have Node.js and npm installed for React and frontend-related tasks.

Setting up the Project

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <project-directory>
   ```

2. Install frontend dependencies:
   ```bash
   cd src/ipc_token_wallet_frontend
   npm install
   ```

3. Build the backend canister:
   ```bash
   cd ../ipc_token_wallet_backend
   dfx build
   ```

4. Start the local ICP network (using DFX):
   ```bash
   dfx start
   ```

5. Deploy the canisters:
   ```bash
   dfx deploy
   ```

6. Access the frontend:
   ```bash
   cd src/ipc_token_wallet_frontend
   npm start
   ```

7. Open your browser and navigate to [http://localhost:3000](http://localhost:3000) to use the wallet.

---

