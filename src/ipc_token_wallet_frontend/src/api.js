import { Actor, HttpAgent } from '@dfinity/agent';
import { idlFactory as backendIDL } from '../../declarations/ipc_token_wallet_backend/ipc_token_wallet_backend.did';
import { canisterId as backendCanisterId } from '../../declarations/ipc_token_wallet_backend';

const agent = new HttpAgent();

// You can remove this in production
agent.fetchRootKey();

// Create an actor to interact with the backend
const backendActor = Actor.createActor(backendIDL, {
  agent,
  canisterId: backendCanisterId,
});

export const getBalance = async () => {
  const balance = await backendActor.getBalance();  // Change this depending on how your backend method is named
  return balance;
};

export const sendTokens = async (to, amount) => {
  const result = await backendActor.transfer(to, amount); // Use correct backend method
  return result;
};

export const receiveTokens = async (amount) => {
  const result = await backendActor.receiveTokens(amount); // Use correct backend method
  return result;
};
