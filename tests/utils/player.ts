import * as anchor from '@coral-xyz/anchor'
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from '@solana/web3.js'
import { writeFileSync, readFileSync, existsSync, unlinkSync } from 'fs'
import { ResourceKey } from './resources'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'

const path = 'tests/.wallets'

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

export function clearPlayers() {
  for (const index of [1, 2]) {
    if (existsSync(path_to_file(index))) {
      unlinkSync(path_to_file(index))
    }
  }
}

export async function usePlayer(index: number): Promise<Keypair> {
  let keypair: Keypair
  if (!existsSync(path_to_file(index))) {
    keypair = await createAndFundWallet()
    writeFileSync(
      `${path}/wallet-${index}.json`,
      JSON.stringify(Array.from(keypair.secretKey)),
    )
  } else {
    keypair = parseWallet(index)
  }
  // console.log(`\tUsing player ${index}: ${keypair.publicKey}`)
  return keypair
}

function path_to_file(index: number) {
  return `${path}/wallet-${index}.json`
}

function parseWallet(index: number): Keypair {
  const secretKeyArray = JSON.parse(readFileSync(path_to_file(index), 'utf-8'))
  const keypair = Keypair.fromSecretKey(new Uint8Array(secretKeyArray))
  return keypair
}

async function getPlayerHolding(
  mintKey: string,
  playerWallet: Keypair,
  programId: PublicKey,
  provider: anchor.AnchorProvider,
) {
  let ata: PublicKey
  if (mintKey === 'igt') {
    const [mintIGT] = PublicKey.findProgramAddressSync(
      [Buffer.from('mint_igt')],
      programId,
    )
    ata = getAssociatedTokenAddressSync(mintIGT, playerWallet.publicKey)
  } else {
    ata = PublicKey.findProgramAddressSync(
      [Buffer.from('account_' + mintKey), playerWallet.publicKey.toBuffer()],
      programId,
    )[0]
  }
  const balance = await provider.connection.getTokenAccountBalance(ata)
  return balance.value.uiAmount || 0
}

export type PlayerBalances = { [K in ResourceKey]: number }
export const resourceKeys = ['igt', 'metal', 'crystal', 'chemical', 'fuel']

export async function getPlayerBalances(
  playerWallet: Keypair,
  programId: PublicKey,
  provider: anchor.AnchorProvider,
  mintKey?: string,
) {
  const balances: PlayerBalances = {
    igt: 0,
    metal: 0,
    crystal: 0,
    chemical: 0,
    fuel: 0,
  }
  for (const r of resourceKeys) {
    if (mintKey && mintKey !== r) {
      continue
    }
    balances[r] = await getPlayerHolding(r, playerWallet, programId, provider)
  }
  return balances
}

export function balanceDiff(a: PlayerBalances, b: PlayerBalances) {
  const diff = {} as PlayerBalances
  for (const r of resourceKeys) {
    diff[r] = (a[r] || 0) - (b[r] || 0)
  }
  return diff
}
