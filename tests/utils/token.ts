import { PublicKey, Signer } from '@solana/web3.js'
import { MARKET_RESOURCES } from './resources'
import * as anchor from '@coral-xyz/anchor'
import { SpaceCastle } from '../../target/types/space_castle'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'

/*
 * Returns the real quantity of a `quantity` parameter by
 * increasing the number using the mint's decimal places
 *
 * @param quantity The provided quantity argument
 * @param decimals The decimals of the associated mint
 * @returns The real quantity of a `quantity` parameter
 */
export function toBigIntQuantity(quantity: number, decimals: number): bigint {
  return BigInt(quantity) * BigInt(10) ** BigInt(decimals)
}

/**
 *
 * Returns the nominal quantity of a `quantity` parameter by
 * decreasing the number using the mint's decimal places
 *
 * @param quantity The real quantity of a `quantity` parameter
 * @param decimals The decimals of the associated mint
 * @returns The nominal quantity of a `quantity` parameter
 */
export function fromBigIntQuantity(quantity: bigint, decimals: number): string {
  return (Number(quantity) / 10 ** decimals).toFixed(6)
}

export async function mintAllResourcesToAddress(signer: Signer) {
  const provider = anchor.AnchorProvider.env()
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>

  const methods = {
    igt: program.methods.mintIgt,
    metal: program.methods.mintMetal,
    chemical: program.methods.mintChemicals,
  }

  for (const resource of MARKET_RESOURCES) {
    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from('mint_' + resource.mintKey)],
      program.programId,
    )
    if (resource.symbol === 'IGT') {
      const associatedTokenAccount = getAssociatedTokenAddressSync(
        mint,
        signer.publicKey,
      )
      await program.methods
        .mintIgt(new anchor.BN(1000))
        .accounts({
          tokenAccount: associatedTokenAccount,
          payer: signer.publicKey,
        })
        .signers([signer])
        .rpc()
      const balance = await provider.connection.getTokenAccountBalance(
        associatedTokenAccount,
      )
      console.log(
        `\tPlayer balance: ${balance.value.uiAmount} ${resource.symbol}`,
      )
    } else {
      const [signer_pda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('account_' + resource.mintKey),
          signer.publicKey.toBuffer(),
        ],
        program.programId,
      )
      await methods[resource.mintKey]({
        payer: signer.publicKey,
      })
        .signers([signer])
        .rpc()
      const balance =
        await provider.connection.getTokenAccountBalance(signer_pda)
      console.log(
        `\tPlayer balance of ${resource.name}: ${balance.value.uiAmount} ${resource.symbol}`,
      )
    }
  }
}
