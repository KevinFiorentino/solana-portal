const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaPortal;
  const baseCounter = anchor.web3.Keypair.generate();
  let tx = await program.rpc.createCounter({
    accounts: {
      baseCounter: baseCounter.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseCounter],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseCounter.fetch(baseCounter.publicKey);
  console.log('👀 GIF Count', account.total.toString())
	
  await program.rpc.counterAdd({
    accounts: {
      baseCounter: baseCounter.publicKey,
    },
  });
  
  account = await program.account.baseCounter.fetch(baseCounter.publicKey);
  console.log('👀 GIF Count', account.total.toString())
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
