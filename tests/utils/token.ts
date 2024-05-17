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
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>
  const methods = {
    metal: program.methods.mintMetal,
    crystal: program.methods.mintCrystal,
    chemical: program.methods.mintChemical,
    fuel: program.methods.mintFuel,
  }
  for (const resource of MARKET_RESOURCES) {
    const [mint] = PublicKey.findProgramAddressSync(
      [Buffer.from('mint_' + resource.mintKey)],
      program.programId,
    )
    if (resource.symbol === 'iGT') {
      const associatedTokenAccount = getAssociatedTokenAddressSync(
        mint,
        signer.publicKey,
      )
      await program.methods
        .mintIgt(new anchor.BN(10000000))
        .accounts({
          tokenAccount: associatedTokenAccount,
          payer: signer.publicKey,
        })
        .signers([signer])
        .rpc()
    } else {
      await methods[resource.mintKey](new anchor.BN(1000000))
        .accounts({
          payer: signer.publicKey,
        })
        .signers([signer])
        .rpc()
    }
  }
}
