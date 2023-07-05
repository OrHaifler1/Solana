// create an empty transaction
const transaction = new web3.Transaction();

// add a hello world program instruction to the transaction
transaction.add(
  new web3.TransactionInstruction({
    keys: [],
    programId: new web3.PublicKey(pg.PROGRAM_ID),
  })
);

// send the transaction to the Solana cluster
console.log("Sending transaction...");
const txHash = await web3.sendAndConfirmTransaction(
  pg.connection,
  transaction,
  [pg.wallet.keypair]
);
console.log("Transaction sent with hash:", txHash);
