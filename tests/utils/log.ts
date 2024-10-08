import { Account as TokenAccount, getMint } from '@solana/spl-token'
import { Connection, Keypair, PublicKey } from '@solana/web3.js'
import { fromBigIntQuantity } from './token'
import { calculateBalances } from './swap'
import { MARKET_RESOURCES } from './resources'
import { getPlayerBalances, resourceKeys } from './player'
import { AnchorProvider } from '@coral-xyz/anchor'

/**
 * Log a line break
 */
export function lineBreak() {
  console.log('--------------------------------------------------------')
}

/**
 *
 * Log information about a newly created asset mint
 *
 * @param name Asset name
 * @param decimals Asset mint decimals
 * @param quantity Quantity of asset minted
 * @param mint Asset mint address
 * @param signature Transaction signature of the minting
 */
export function logNewMint(
  name: string,
  decimals: number,
  quantity: number,
  mint: PublicKey,
  signature: string,
) {
  lineBreak()
  console.log(`Mint: ${name}`)
  console.log(`Address:    ${mint.toBase58()}`)
  console.log(`Decimals:   ${decimals}`)
  console.log(`Quantity:   ${quantity}`)
  console.log(`Transaction Signature: ${signature}`)
  lineBreak()
}

// Logs information about a swap - can be pre- or post-swap

export async function logPreSwap(
  connection: Connection,
  owner: PublicKey,
  pool: PublicKey,
  receive: {
    name: string
    quantity: number
    decimals: number
    address: PublicKey
  },
  pay: {
    name: string
    quantity: number
    decimals: number
    address: PublicKey
  },
  amount: number,
) {
  const [
    receiveUserBalance,
    receivePoolBalance,
    payUserBalance,
    payPoolBalance,
  ] = await calculateBalances(
    connection,
    owner,
    pool,
    receive.address,
    receive.decimals,
    pay.address,
    pay.decimals,
  )
  lineBreak()
  console.log('   PRE-SWAP:')
  console.log()
  console.log(
    `       PAY: ${pay.name.padEnd(
      18,
      ' ',
    )}  RECEIVE: ${receive.name.padEnd(18, ' ')}`,
  )
  console.log(`       OFFERING TO PAY: ${amount}`)
  console.log()
  console.log('   |====================|==============|==============|')
  console.log('   | Asset:             | User:        | Pool:        |')
  console.log('   |====================|==============|==============|')
  console.log(
    `   | ${pay.name.padEnd(18, ' ')} | ${payUserBalance.padStart(
      12,
      ' ',
    )} | ${payPoolBalance.padStart(12, ' ')} |`,
  )
  console.log(
    `   | ${receive.name.padEnd(18, ' ')} | ${receiveUserBalance.padStart(
      12,
      ' ',
    )} | ${receivePoolBalance.padStart(12, ' ')} |`,
  )
  console.log('   |====================|==============|==============|')
  console.log()
}

export async function logPostSwap(
  connection: Connection,
  owner: PublicKey,
  pool: PublicKey,
  receive: {
    name: string
    quantity: number
    decimals: number
    address: PublicKey
  },
  pay: {
    name: string
    quantity: number
    decimals: number
    address: PublicKey
  },
) {
  const [
    receiveUserBalance,
    receivePoolBalance,
    payUserBalance,
    payPoolBalance,
  ] = await calculateBalances(
    connection,
    owner,
    pool,
    receive.address,
    receive.decimals,
    pay.address,
    pay.decimals,
  )
  console.log('   POST-SWAP:')
  console.log()
  console.log('   |====================|==============|==============|')
  console.log('   | Asset:             | User:        | Pool:        |')
  console.log('   |====================|==============|==============|')
  console.log(
    `   | ${pay.name.padEnd(18, ' ')} | ${payUserBalance.padStart(
      12,
      ' ',
    )} | ${payPoolBalance.padStart(12, ' ')} |`,
  )
  console.log(
    `   | ${receive.name.padEnd(18, ' ')} | ${receiveUserBalance.padStart(
      12,
      ' ',
    )} | ${receivePoolBalance.padStart(12, ' ')} |`,
  )
  console.log('   |====================|==============|==============|')
  console.log()
  lineBreak()
}

/**
 *
 * Logs the Liquidity Pool's holdings (assets held in each token account)
 *
 * @param connection Connection to Solana RPC
 * @param programId Program Id
 * @param poolAddress Address of the Liquidity Pool
 * @param tokenAccounts All token accounts owned by the Liquidity Pool
 * @param k The constant-product `K` (Constant-Product Algorithm)
 */
export async function logPool(
  connection: Connection,
  programId: PublicKey,
  poolAddress: PublicKey,
  tokenAccounts: TokenAccount[],
  k: bigint,
) {
  function getHoldings(mint: PublicKey, tokenAccounts: TokenAccount[]): bigint {
    const holding = tokenAccounts.find((account) => account.mint.equals(mint))
    return holding?.amount || BigInt(0)
  }
  const padding = MARKET_RESOURCES.reduce(
    (max, a) => Math.max(max, a.name.length),
    0,
  )
  lineBreak()
  console.log('Liquidity Pool:')
  console.log(`Address:    ${poolAddress.toBase58()}`)
  console.log('Holdings:')
  for (const a of MARKET_RESOURCES) {
    const mint = PublicKey.findProgramAddressSync(
      [Buffer.from('mint_' + a.mintKey)],
      programId,
    )[0]
    const holding = getHoldings(mint, tokenAccounts)
    const mint_data = await getMint(connection, mint)
    const normalizedHolding = fromBigIntQuantity(holding, mint_data.decimals)
    console.log(
      `\t${a.name.padEnd(padding, ' ')} : ${normalizedHolding.padStart(12, ' ')} : ${mint.toBase58()}`,
    )
  }
  logK(k)
  lineBreak()
}

export async function logPlayerBalances(
  playerWallet: Keypair,
  programId: PublicKey,
  provider: AnchorProvider,
  mintKey?: string,
) {
  const holdings = await getPlayerBalances(
    playerWallet,
    programId,
    provider,
    mintKey,
  )
  let format = ' Balances:'
  lineBreak()
  for (const r of resourceKeys) {
    if (mintKey && mintKey !== r) {
      continue
    }
    const holding = holdings[r]
    format = format.concat(` | ${holding} ${r}`)
  }
  console.log(format)
  lineBreak()
}

/**
 *
 * Logs `K`
 *
 * @param k The constant-product `K` (Constant-Product Algorithm)
 */
export function logK(k: bigint) {
  console.log(`** Constant-Product (K): ${k.toString()}`)
}

/**
 *
 * Logs `ΔK` ("delta K", or "change in K")
 *
 * @param changeInK The change in the constant-product `K`
 * (Constant-Product Algorithm)
 */
export function logChangeInK(changeInK: string) {
  console.log(`\n** Δ Change in Constant-Product (K): ${changeInK}\n`)
}
