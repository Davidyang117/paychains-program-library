const web3 = require('@paychains/web3.js');
const splToken = require('@paychains/spl-token');

(async () => {
  // Connect to cluster
  const connection = new web3.Connection(
    web3.clusterApiUrl('devnet'),
    'confirmed',
  );

  // Generate a new wallet keypair and airdrop PAY
  var fromWallet = web3.Keypair.generate();
  var fromAirdropSignature = await connection.requestAirdrop(
    fromWallet.publicKey,
    web3.LAMPORTS_PER_PAY,
  );
  // Wait for airdrop confirmation
  await connection.confirmTransaction(fromAirdropSignature);

  // Generate a new wallet to receive newly minted token
  const toWallet = web3.Keypair.generate();

  // Create new token mint
  const mint = await splToken.Token.createMint(
    connection,
    fromWallet,
    fromWallet.publicKey,
    null,
    9,
    splToken.TOKEN_PROGRAM_ID,
  );

  // Get the token account of the fromWallet PayChains address, if it does not exist, create it
  const fromTokenAccount = await mint.getOrCreateAssociatedAccountInfo(
    fromWallet.publicKey,
  );

  //get the token account of the toWallet PayChains address, if it does not exist, create it
  const toTokenAccount = await mint.getOrCreateAssociatedAccountInfo(
    toWallet.publicKey,
  );

  // Minting 1 new token to the "fromTokenAccount" account we just returned/created
  await mint.mintTo(
    fromTokenAccount.address,
    fromWallet.publicKey,
    [],
    1000000000,
  );

  // Add token transfer instructions to transaction
  const transaction = new web3.Transaction().add(
    splToken.Token.createTransferInstruction(
      splToken.TOKEN_PROGRAM_ID,
      fromTokenAccount.address,
      toTokenAccount.address,
      fromWallet.publicKey,
      [],
      1,
    ),
  );

  // Sign transaction, broadcast, and confirm
  const signature = await web3.sendAndConfirmTransaction(
    connection,
    transaction,
    [fromWallet],
    {commitment: 'confirmed'},
  );
  console.log('SIGNATURE', signature);
})();
