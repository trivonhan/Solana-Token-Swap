import { sendTransaction2, TransactionLog } from "@coin98/solana-support-library";
import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import BN from "bn.js";
import { TokenSwapInstructionService } from "./token-swap-instruction";

export class TokenSwapService {
  static async initializePool(
    connection: Connection,
    owner: Keypair,
    poolState: PublicKey,
    poolAuthority: PublicKey,
    tokenAAta: PublicKey,
    tokenBAta: PublicKey,
    poolTokenMint: PublicKey,
    tokenAMint: PublicKey,
    tokenBMint: PublicKey,
    systemProgramId: PublicKey,
    tokenSwapProgramId: PublicKey,
    bumpSeed: number,
    poolAuthorityBumpSeed: number,
  ){
    const transaction = new Transaction();
    const initializePoolInstruction = TokenSwapInstructionService.initializePool(
      owner.publicKey,
      poolState,
      poolAuthority,
      tokenAAta,
      tokenBAta,
      poolTokenMint,
      tokenAMint,
      tokenBMint,
      systemProgramId,
      tokenSwapProgramId,
      bumpSeed,
      poolAuthorityBumpSeed,
    );

    transaction.add(initializePoolInstruction);

    const initializePoolTx = await sendTransaction2(connection, transaction, [owner]);

    return initializePoolTx;
  }

  static async depositLiquidity(
    connection: Connection,
    liquidityProvider: Keypair,
    poolState: PublicKey,
    poolAuthority: PublicKey,
    tokenAAta: PublicKey,
    tokenBAta: PublicKey,
    poolTokenAta: PublicKey,
    poolTokenAAta: PublicKey,
    poolTokenBAta: PublicKey,
    poolTokenMint: PublicKey,
    tokenAMint: PublicKey,
    tokenBMint: PublicKey,
    tokenProgramId: PublicKey,
    systemProgramId: PublicKey,
    tokenSwapProgramId: PublicKey,
    amountA: BN,
    amountB: BN,
  ){//}: Promise<[string, TransactionLog]> {
    console.log("depositLiquidity", liquidityProvider.publicKey);

    const transaction = new Transaction();
    const depositLiquidityInstruction = TokenSwapInstructionService.depositLiquidity(
      liquidityProvider.publicKey,
      poolState,
      poolAuthority,
      tokenAAta,
      tokenBAta,
      poolTokenAta,
      poolTokenAAta,
      poolTokenBAta,
      poolTokenMint,
      tokenAMint,
      tokenBMint,
      tokenProgramId,
      systemProgramId,
      tokenSwapProgramId,
      amountA,
      amountB,
    );

    transaction.add(depositLiquidityInstruction);

    // const depositLiquidityTx = await sendAndConfirmTransaction(
    //   connection,
    //   transaction,
    //   [liquidityProvider],
    // );

    const depositLiquidityTx = await sendTransaction2(connection, transaction, [liquidityProvider]);

    return depositLiquidityTx;
  }

}
