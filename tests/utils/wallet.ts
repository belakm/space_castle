import * as anchor from '@coral-xyz/anchor'
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js'

export async function createAndFundWallet(): Promise<Keypair> {
  const provider = anchor.AnchorProvider.env()
  const connection = new Connection(
    provider.connection.rpcEndpoint,
    'confirmed',
  )
  const wallet = Keypair.generate()
  const airdropSignature = await connection.requestAirdrop(
    wallet.publicKey,
    2 * LAMPORTS_PER_SOL, // Adjust the amount as needed
  )
  await connection.confirmTransaction(airdropSignature)
  return wallet
}
