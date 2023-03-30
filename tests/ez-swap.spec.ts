import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { SolanaConfigService } from "@coin98/solana-support-library/config";
// import { Account, createMint, getAccount, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { TokenProgramService } from "@coin98/solana-support-library";
import { TokenSwapService } from "../services";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import BN from "bn.js";
import { mintTo } from "@solana/spl-token";

describe("ez-swap", () => {

  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  const encoder = new TextEncoder();

  const EZ_SWAP_PROGRAM_ID = new PublicKey("DxsrhkT7S2EqipZoSyQd9bmx5eLurU49cCZ4yDMHDN52");
  const TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');

  // Root will be pool Liquidity provider
  let root: Keypair;
  let tokenARootAta: PublicKey;
  let tokenBRootAta: PublicKey;

  // Owner will be owner of pool token
  let owner: Keypair;

  let poolState: PublicKey;
  let poolBump: number;
  let poolAuthority: PublicKey;
  let poolAuthorityBump: number;
  let tokenAAta: PublicKey;
  let tokenBAta: PublicKey;

  let poolTokenMint: Keypair;
  let tokenAMint: Keypair;
  let tokenBMint: Keypair;

  // User deposit to pool
  let poolTokenRootAta: PublicKey;

  before( async () => {
    root = await SolanaConfigService.getDefaultAccount();

    // New owner of pool token
    owner = Keypair.generate();
    const airdropSignature = await connection.requestAirdrop(
      owner.publicKey,
       2 * LAMPORTS_PER_SOL,
    );
    await connection.confirmTransaction(airdropSignature);

    // Create mint Token A
    tokenAMint = Keypair.generate();
    tokenAMint = await TokenProgramService.createTokenMint(
      connection,
      owner,
      tokenAMint,
      9,
      owner.publicKey,
      owner.publicKey,
    );
    console.log("tokenAMint", tokenAMint.publicKey.toBase58());

    // Create associated token account for Token A in pool
    tokenAAta = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      owner,
      owner.publicKey,
      tokenAMint.publicKey,
    );
    console.log("tokenAAta", tokenAAta.toBase58());

    // Create mint Token B
    tokenBMint = Keypair.generate()
    tokenBMint = await TokenProgramService.createTokenMint(
      connection,
      owner,
      tokenBMint,
      9,
      owner.publicKey,
      owner.publicKey,
    );
    console.log("tokenBMint", tokenBMint.publicKey.toBase58());

    // Create associated token account for Token B in pool
    tokenBAta = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      owner,
      owner.publicKey,
      tokenBMint.publicKey,
    );
    console.log("tokenBAta", tokenBAta.toBase58());

  })

  it("should initialize pool", async () => {

    // Create pool state
    [poolState, poolBump] = findProgramAddressSync(
      [
        encoder.encode("POOL_STATE"),
        tokenAMint.publicKey.toBuffer(),
        tokenBMint.publicKey.toBuffer(),
      ],
      EZ_SWAP_PROGRAM_ID,
    );
    console.log("poolState", poolState.toBase58());

    [poolAuthority, poolAuthorityBump] = findProgramAddressSync(
      [
        Buffer.from("POOL_STATE"),
        poolState.toBuffer(),
        Buffer.from("POOL_AUTHORITY"),
      ],
      EZ_SWAP_PROGRAM_ID,
    );
    console.log("poolAuthority", poolAuthority.toBase58());
    console.log("poolAuthorityBump", poolAuthorityBump);

    // Create pool token mint
    poolTokenMint = Keypair.generate();
    poolTokenMint = await TokenProgramService.createTokenMint(
      connection,
      owner,
      poolTokenMint,
      9,
      poolAuthority,
      poolAuthority,
    );
    console.log("poolTokenMint", poolTokenMint.publicKey.toBase58());

    const tx = await TokenSwapService.initializePool(
      connection,
      owner,
      poolState,
      poolAuthority,
      tokenAAta,
      tokenBAta,
      poolTokenMint.publicKey,
      tokenAMint.publicKey,
      tokenBMint.publicKey,
      SystemProgram.programId,
      EZ_SWAP_PROGRAM_ID,
      poolBump,
      poolAuthorityBump
    );

    console.log("tx", tx);

  });

  it("Deposit liquidity", async () => {

    // Create associated token account for Token B in pool
    poolTokenRootAta = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      root,
      root.publicKey,
      poolTokenMint.publicKey,
    );
    console.log("poolTokenRootAta", poolTokenRootAta.toBase58());

    // Mint token A to root account
    tokenARootAta = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      root,
      root.publicKey,
      tokenAMint.publicKey,
    );
    console.log("tokenARootAta", tokenARootAta.toBase58());

    await mintTo(
      connection,
      root,
      tokenAMint.publicKey,
      tokenARootAta,
      owner,
      10 * LAMPORTS_PER_SOL,
    );

    // Mint token B to root account
    tokenBRootAta = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      root,
      root.publicKey,
      tokenBMint.publicKey,
    );
    console.log("tokenBRootAta", tokenBRootAta.toBase58());

    await mintTo(
      connection,
      root,
      tokenBMint.publicKey,
      tokenBRootAta,
      owner,
      1000 * LAMPORTS_PER_SOL,
    );

    const tx = await TokenSwapService.depositLiquidity(
      connection,
      root,
      poolState,
      poolAuthority,
      tokenARootAta,
      tokenBRootAta,
      poolTokenRootAta,
      tokenAAta,
      tokenBAta,

      poolTokenMint.publicKey,
      tokenAMint.publicKey,
      tokenBMint.publicKey,
      TOKEN_PROGRAM_ID,
      SystemProgram.programId,
      EZ_SWAP_PROGRAM_ID,
      new BN(2 * LAMPORTS_PER_SOL),
      new BN(200 * LAMPORTS_PER_SOL),
    );
    console.log("tx", tx);
  });

  it("Deposit second times", async () => {

      const tx = await TokenSwapService.depositLiquidity(
        connection,
      root,
      poolState,
      poolAuthority,
      tokenARootAta,
      tokenBRootAta,
      poolTokenRootAta,
      tokenAAta,
      tokenBAta,
      poolTokenMint.publicKey,
      tokenAMint.publicKey,
      tokenBMint.publicKey,
      TOKEN_PROGRAM_ID,
      SystemProgram.programId,
      EZ_SWAP_PROGRAM_ID,
      new BN(2 * LAMPORTS_PER_SOL),
      new BN(200 * LAMPORTS_PER_SOL),
      );
      console.log("tx", tx);
  });

})
