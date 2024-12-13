import React, { useState } from "react";
import { sendTokens } from "../api";
import "../Styles/SendToken.css";

const SendTokens = ({ onUpdate }) => {
  const [to, setTo] = useState("");
  const [amount, setAmount] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    await sendTokens(to, amount);
    setTo("");
    setAmount("");
    onUpdate();
  };

  return (
    <div className="send-tokens-container">
      <h2>Send Tokens</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Recipient Wallet"
          value={to}
          onChange={(e) => setTo(e.target.value)}
          required
        />
        <input
          type="number"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          required
        />
        <button type="submit">Send</button>
      </form>
    </div>
  );
};

export default SendTokens;
