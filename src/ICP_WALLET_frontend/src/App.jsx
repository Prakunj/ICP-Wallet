import { useState } from 'react';
import { ICP_WALLET_backend } from 'declarations/ICP_WALLET_backend';

function App() {
  const [account, setAccount] = useState('');
  const [balance, setBalance] = useState(null);
  const [receiver, setReceiver] = useState('');
  const [sendAmount, setSendAmount] = useState('');
  const [sendMessage, setSendMessage] = useState('');
  const [receiveAmount, setReceiveAmount] = useState('');
  const [receiveMessage, setReceiveMessage] = useState('');
  const [sendTo, setSendTo] = useState('');


  const handleGetBalance = async (e) => {
    e.preventDefault();
    try {
      console.log('account-', account);
      ICP_WALLET_backend.get_balance(account).then((res) => {
        setBalance(res.toString());
      });
    } catch (error) {
      console.error(error);
      setBalance('Error fetching balance');
    }
  };

  const handleSendTokens = async (e) => {
    e.preventDefault();
    try {
      ICP_WALLET_backend.send_tokens(sendTo, BigInt(sendAmount)).then((res) => {
        setSendMessage(res);
      });
    } catch (error) {
      console.error(error);
      setSendMessage('Error sending tokens');
    }
  };

  // Function to receive tokens
  const handleReceiveTokens = async (e) => {
    e.preventDefault();
    try {
      ICP_WALLET_backend.receive_tokens(receiver, BigInt(receiveAmount)).then((res) => {
        setReceiveMessage(res);
      });
    } catch (error) {
      console.error(error);
      setReceiveMessage('Error receiving tokens');
    }
  };

  return (
    <main>
      <h1>ICP Wallet</h1>

      {/* Section to get balance */}
      <section>
        <h2>Check Balance</h2>
        <form onSubmit={handleGetBalance}>
          <label htmlFor="account">Account:</label>
          <input
            id="account"
            type="text"
            value={account}
            onChange={(e) => setAccount(e.target.value)}
            required
          />
          <button type="submit">Get Balance</button>
        </form>
        {balance !== null && <p>Balance: {balance}</p>}
      </section>

      {/* Section to send tokens */}
      <section>
        <h2>Send Tokens</h2>
        <form onSubmit={handleSendTokens}>
          <label htmlFor="to">To:</label>
          <input
            id="to"
            type="text"
            value={sendTo}
            onChange={(e) => setSendTo(e.target.value)}
            required
          />
          <label htmlFor="sendAmount">Amount:</label>
          <input
            id="sendAmount"
            type="number"
            value={sendAmount}
            onChange={(e) => setSendAmount(e.target.value)}
            required
          />
          <button type="submit">Send</button>
        </form>
        {sendMessage && <p>{sendMessage}</p>}
      </section>

      {/* Section to receive tokens */}
      <section>
        <h2>Receive Tokens</h2>
        <form onSubmit={handleReceiveTokens}>
        <label htmlFor="receiver">Receiver:</label>
        <input
            id="receiver"
            type="text"
            value={receiver}
            onChange={(e) => setReceiver(e.target.value)}
            required
          />
          <label htmlFor="receiveAmount">Amount:</label>
          <input
            id="receiveAmount"
            type="number"
            value={receiveAmount}
            onChange={(e) => setReceiveAmount(e.target.value)}
            required
          />
          <button type="submit">Receive</button>
        </form>
        {receiveMessage && <p>{receiveMessage}</p>}
      </section>
    </main>
  );

}

export default App;
