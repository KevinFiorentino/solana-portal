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
  console.log('👀 Total:', account.total.toString())
	
  await program.rpc.counterAdd('https://media3.giphy.com/media/l0MYKMrQnwNvLjYhW/giphy.gif?cid=ecf05e479l2lejvyjgzkalyua6aqr73zmji2xhmwc71kgdez&rid=giphy.gif&ct=g',
    {
      accounts: {
        baseCounter: baseCounter.publicKey,
        user: provider.wallet.publicKey,
      },
    }
  );
  
  account = await program.account.baseCounter.fetch(baseCounter.publicKey);
  console.log('👀 Total', account.total.toString());

  console.log('👀 Images list:', account.imageList);
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
