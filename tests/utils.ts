import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js'

export async function createAndFundWallet(): Promise<Keypair> {
  const connection = new Connection('http://localhost:8899', 'confirmed')
  const wallet = Keypair.generate()
  const airdropSignature = await connection.requestAirdrop(
    wallet.publicKey,
    5 * LAMPORTS_PER_SOL // Adjust the amount as needed
  )
  await connection.confirmTransaction(airdropSignature)
  return wallet
}

export function number_to_bytes_buffer(input: number) {
  return new Uint8Array(new Uint16Array([input]).buffer)
}
