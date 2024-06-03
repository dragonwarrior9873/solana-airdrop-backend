import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Airdrop } from "../target1/types/airdrop";
import { Keypair } from "@solana/web3.js";
import { PublicKey, Connection, Transaction } from "@solana/web3.js";
// import { Program, web3, BN } from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";


describe("airdrop", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Airdrop as Program<Airdrop>;
  const PROGRAM_ID = program.programId;

  const USER_SEED = "USER_SEED";
  const AIRDROP_SEED = "AIRDROP_SEED";

  const myWallet = anchor.AnchorProvider.env().wallet;
  const payer = anchor.AnchorProvider.env().wallet as anchor.Wallet;
  const myPubkey = myWallet.publicKey;

  const pubkey1 = anchor.web3.Keypair.generate().publicKey;
  const pubkey2 = anchor.web3.Keypair.generate().publicKey;

  const airdrop_end_timestamp = (new Date("2024-09-29")).getTime();

  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const airdropTokenPubkey = new PublicKey("CGGVmKPGpDerhsftnC6MpQoDpXa6df8bFeBuM4FtbMon");
  const MINT_DECIMALS = 10 ** 9;

  const recipientWallet: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const airdropAuthorityPubkey = myPubkey; //new PublicKey("HFsdexGjCMJ6XBhQfbKvEUBDdBQ3P1BwR4Nnj4KSb8Yr");

  const getWalletPDA = async (identifier: number) => {
    return (
      await PublicKey.findProgramAddressSync(
        [Buffer.from(USER_SEED), airdropAuthorityPubkey.toBuffer(), payer.publicKey.toBuffer(), Uint8Array.from([identifier])],
        PROGRAM_ID
      )
    )[0];
  };

  const getUserPDA = async (userKey, identifier: number) => {
    return (
      await PublicKey.findProgramAddressSync(
        [Buffer.from(USER_SEED), userKey.toBuffer(), Uint8Array.from([identifier])],
        PROGRAM_ID
      )
    )[0];
  };

  const getAirdropPDA = async (airdropIdentifier: number) => {
    return (
      await PublicKey.findProgramAddressSync(
        [Buffer.from(AIRDROP_SEED), airdropAuthorityPubkey.toBuffer(), Uint8Array.from([airdropIdentifier])],
        PROGRAM_ID
      )
    )[0];
  };

  console.log(`My pubkey: ${myPubkey}`);
  console.log(`pubkey1: ${pubkey1}`);
  console.log(`pubkey2: ${pubkey2}`);

  // Generate a new keypair to use as the address the airdrop account
  const airdropAccount = new Keypair();

  xit("Create an Airdrop", async () => {
    const airdropPDA = await getAirdropPDA(1);
    console.log(`Create address: ${airdropPDA}`);

    // Invoke the increment instruction
    const tx = await program.methods
      .createAirdrop(
        airdropTokenPubkey,
        new anchor.BN(1000),
        new anchor.BN(airdrop_end_timestamp),
        1
      )
      .accounts({
        airdropInfo: airdropPDA,
        authority: airdropAuthorityPubkey,
        systemProgram: anchor.web3.SystemProgram.programId,
      });

    // simulation code
    let simRes = await simulateTxn(
      program.provider.connection,
      await tx.transaction(),
      airdropAuthorityPubkey
    );
    console.log("simulation res >> ", simRes);

    tx.rpc();
  });

  xit("Deposite to Airdrop", async () => {
    const airdropPDA = await getAirdropPDA(1);
    const userPDA = await getWalletPDA(1);

    console.log(`Deposite address: ${airdropPDA}`);

    console.log(`Mint: ${airdropTokenPubkey}`)

    const fromAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: airdropTokenPubkey,
      owner: payer.publicKey,
    });

    console.log(`From: ${fromAssociatedTokenAccountAddress}`);

    const toAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: airdropTokenPubkey,
      owner: airdropPDA,
    });

    console.log(`To: ${toAssociatedTokenAccountAddress}`);

    // Invoke the increment instruction
    const tx = await program.methods
      .depositToken(
        new anchor.BN(10000),
        1
      )
      .accounts({
        mintAccount: airdropTokenPubkey,
        airdropAuthority: airdropAuthorityPubkey,
        fromAssociatedTokenAccount: fromAssociatedTokenAccountAddress,
        toAssociatedTokenAccount: toAssociatedTokenAccountAddress,
        airdropInfo: airdropPDA,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

    // simulation code
    let simRes = await simulateTxn(
      program.provider.connection,
      await tx.transaction(),
      payer.publicKey
    );
    console.log("simulation res >> ", simRes);

    tx.rpc();
  });

  xit("Claim Airdrop", async () => {
    const airdropPDA = await getAirdropPDA(1);

    const claimerPublicKey = new PublicKey("7L5eDM1KxTJhkA2Ucr67Rn4mnPDV6udMEVgGg7ui43M6");

    const claimer = anchor.web3.Keypair.generate();
    const userPDA = await getUserPDA(claimerPublicKey, 1);
    // const userPDA = await getWalletPDA(1);

    console.log(`User address: ${userPDA}`);

    const fromAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: airdropTokenPubkey,
      owner: airdropPDA,
    });

    console.log(`From: ${fromAssociatedTokenAccountAddress}`);

    const toAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: airdropTokenPubkey,
      owner: claimerPublicKey,
    });

    console.log(`To pubKey: ${claimerPublicKey}`);
    console.log(`To ATA: ${toAssociatedTokenAccountAddress}`);

    // Invoke the increment instruction
    const tx = await program.methods
      .claimToken(
        1
      )
      .accounts({
        mintAccount: airdropTokenPubkey,
        airdropAuthority: airdropAuthorityPubkey,
        depositedTokenAta: fromAssociatedTokenAccountAddress,
        claimerAta: toAssociatedTokenAccountAddress,
        userInfo: userPDA,
        airdropInfo: airdropPDA,
        claimer: claimerPublicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

    // simulation code
    let simRes = await simulateTxn(
      program.provider.connection,
      await tx.transaction(),
      payer.publicKey
    );
    console.log("simulation res >> ", simRes);

    tx.rpc();
  });

  it("Withdraw from Airdrop", async () => {
    const airdropPDA = await getAirdropPDA(1);

    console.log(`Deposite address: ${airdropPDA}`);

    console.log(`Mint: ${airdropTokenPubkey}`)

    const fromAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: airdropTokenPubkey,
      owner: airdropPDA,
    });

    console.log(`From: ${fromAssociatedTokenAccountAddress}`);

    const toAssociatedTokenAccountAddress = await anchor.utils.token.associatedAddress({
      mint: airdropTokenPubkey,
      owner: payer.publicKey,
    });

    console.log(`To: ${toAssociatedTokenAccountAddress}`);

    // Invoke the increment instruction
    const tx = await program.methods
      .withdrawToken(
        new anchor.BN(1000),
        1
      )
      .accounts({
        mintAccount: airdropTokenPubkey,
        airdropAuthority: airdropAuthorityPubkey,
        fromAssociatedTokenAccount: fromAssociatedTokenAccountAddress,
        toAssociatedTokenAccount: toAssociatedTokenAccountAddress,
        airdropInfo: airdropPDA,
        payer: payer.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      });

    // simulation code
    let simRes = await simulateTxn(
      program.provider.connection,
      await tx.transaction(),
      payer.publicKey
    );
    console.log("simulation res >> ", simRes);

    tx.rpc();
  });
});

export const simulateTxn = async (
  connection: Connection,
  tx: Transaction,
  userPublicKey: PublicKey
) => {
  const blockhashInfo = await connection.getLatestBlockhash();
  tx.recentBlockhash = blockhashInfo.blockhash;
  tx.feePayer = userPublicKey;
  const simulationResult = await connection.simulateTransaction(
    tx.compileMessage()
  );
  return simulationResult;
};