import React, { useState, useEffect } from "react";
import { getBalance } from "../api";
import "../Styles/Balance.css";

const Balance = () => {
  const [balance, setBalance] = useState(0);

  useEffect(() => {
    async function fetchBalance() {
      const fetchedBalance = await getBalance();
      setBalance(fetchedBalance);
    }

    fetchBalance();
  }, []);

  return (
    <div className="balance-container">
      <h2>Your Balance</h2>
      <p>{balance} Tokens</p>
    </div>
  );
};

export default Balance;
