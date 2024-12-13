import React, { useState } from "react";
import "./styles/App.css";
import Balance from "./components/Balance.jsx";
import SendTokens from "./components/SendTokens.jsx";
import ReceiveTokens from "./components/ReceiveTokens.jsx";

function App() {
  const [balance, setBalance] = useState(0);

  const handleSendTokens = (recipient, amount) => {
    console.log(`Sending ${amount} tokens to ${recipient}`);
    setBalance(balance - Number(amount));
  };

  const handleReceiveTokens = (amount) => {
    console.log(`Receiving ${amount} tokens`);
    setBalance(balance + Number(amount));
  };

  return (
    <div className="App">
      <h1>ICP Token Wallet</h1>
      <div className="container">
        <Balance balance={balance} />
        <SendTokens onSend={handleSendTokens} />
        <ReceiveTokens onReceive={handleReceiveTokens} />
      </div>
    </div>
  );
}

export default App;
