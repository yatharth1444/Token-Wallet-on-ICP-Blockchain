import React, { useState } from "react";
import { receiveTokens } from "../api";
import "../Styles/ReceiveToken.css";

const ReceiveTokens = ({ onUpdate }) => {
  const [amount, setAmount] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    await receiveTokens(amount);
    setAmount("");
    onUpdate();
  };

  return (
    <div className="receive-tokens-container">
      <h2>Receive Tokens</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="number"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          required
        />
        <button type="submit">Receive</button>
      </form>
    </div>
  );
};

export default ReceiveTokens;
