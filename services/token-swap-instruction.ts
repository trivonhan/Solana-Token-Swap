import { BorshCoder, Idl } from "@project-serum/anchor";
import { AccountMeta, PublicKey, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import EzTokenSwapIdl from "../target/idl/ez_token_swap.json";

const coder = new BorshCoder(EzTokenSwapIdl as Idl)

export interface InitializeSwapRequest {
    bumpSeed: number;
    poolAuthorityBumpSeed: number;
}

export interface DepositTokenRequest {
  amountA: BN;
  amountB: BN;
}

export class TokenSwapInstructionService {
    static initializePool(
      owner: PublicKey,
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
    ) {
      const request: InitializeSwapRequest = {
        bumpSeed,
        poolAuthorityBumpSeed,
      }

      const data = coder.instruction.encode("initializePool", request);

      const keys: AccountMeta[] = [
        <AccountMeta>{ pubkey: owner, isSigner: true, isWritable: true },
        <AccountMeta>{ pubkey: poolState, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolAuthority, isSigner: false, isWritable: false },
        <AccountMeta>{ pubkey: tokenAAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenBAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolTokenMint, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenAMint, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenBMint, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: systemProgramId, isSigner: false, isWritable: false },
      ];

      return new TransactionInstruction({
        keys,
        programId: tokenSwapProgramId,
        data,
      })
    }

    static depositLiquidity(
      liquidityProvider: PublicKey,
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
    ) {
      const request: DepositTokenRequest = {
        amountA,
        amountB,
      }

      const data = coder.instruction.encode("depositLiquidity", request);

      const keys: AccountMeta[] = [
        <AccountMeta>{ pubkey: liquidityProvider, isSigner: true, isWritable: true },
        <AccountMeta>{ pubkey: poolState, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolAuthority, isSigner: false, isWritable: false },
        <AccountMeta>{ pubkey: tokenAAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenBAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolTokenAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolTokenAAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolTokenBAta, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: poolTokenMint, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenAMint, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenBMint, isSigner: false, isWritable: true },
        <AccountMeta>{ pubkey: tokenProgramId, isSigner: false, isWritable: false },
        <AccountMeta>{ pubkey: systemProgramId, isSigner: false, isWritable: false },
      ];

      return new TransactionInstruction({
        keys,
        programId: tokenSwapProgramId,
        data,
      })
    }
}

