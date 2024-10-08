import * as anchor from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { PublicKey, Keypair } from '@solana/web3.js'
import { assert } from 'chai'
import { MARKET_RESOURCES } from './utils/resources'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'
import {
  calculateChangeInK,
  calculateK,
  fetchPool,
  fetchPoolTokenAccounts,
} from './utils/swap'
import { logChangeInK } from './utils/log'
import { usePlayer } from './utils/player'

describe('[Test] 💱 Market pool - a global automated market and market maker', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>
  const payer = (provider.wallet as anchor.Wallet).payer
  const poolAddress = PublicKey.findProgramAddressSync(
    [Buffer.from('market_pool')],
    program.programId,
  )[0]
  let playerWallet: Keypair
  let poolInitialized = false

  /**
   *
   * Calculates the Liquidity Pool's holdings (assets held in each token account)
   *
   * @param log A flag provided telling this function whether or not to print to logs
   * @returns The constant-product `K` (Constant-Product Algorithm)
   */
  async function getPoolData(): Promise<bigint> {
    const pool = await fetchPool(program, poolAddress)
    const poolTokenAccounts = await fetchPoolTokenAccounts(
      provider.connection,
      poolAddress,
      pool,
    )
    const k = calculateK(poolTokenAccounts)
    // await logPool(
    //   provider.connection,
    //   program.programId,
    //   poolAddress,
    //   poolTokenAccounts,
    //   k,
    // )
    return k
  }

  /**
   * Check if the Liquidity Pool exists and set the flag
   */
  before('Check if Pool exists', async () => {
    const poolAccountInfo =
      await provider.connection.getAccountInfo(poolAddress)
    if (poolAccountInfo != undefined && poolAccountInfo.lamports != 0) {
      console.log('Pool already initialized!')
      console.log(`Address: ${poolAddress.toBase58()}`)
      poolInitialized = true
    }
  })

  before(
    'Prepare wallets and fund mock player wallet with resources',
    async () => {
      playerWallet = (await usePlayer(1, program.programId)).keypair
      // mintAllResourcesToAddress(playerWallet)
    },
  )

  it('Market pool initialization', async () => {
    if (poolInitialized) {
      return assert.ok('Pool exists already.')
    }
    await program.methods
      .marketPoolCreate()
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
  })

  it('Market pool can be filled with resources and IGT', async () => {
    for (const resource of MARKET_RESOURCES) {
      const mint = PublicKey.findProgramAddressSync(
        [Buffer.from('mint_' + resource.mintKey)],
        program.programId,
      )
      await program.methods
        .marketPoolMintTo(new anchor.BN(resource.quantity), resource.mintKey)
        .accounts({
          mint: mint[0],
          payer: payer.publicKey,
        })
        .signers([payer])
        .rpc()
      // const balance = await provider.connection.getTokenAccountBalance(ata)
      // console.log(
      //   `\tMinted: ${balance.value.uiAmount} ${resource.symbol} to market pool`,
      // )
    }
  })

  it('All tokens are interchangable on the Market pool (IGT, rMETL, rCRYS, rCHEM, rFUEL)', async () => {
    const initialK = await getPoolData()
    for (const payResource of MARKET_RESOURCES) {
      for (const receiveResource of MARKET_RESOURCES) {
        if (payResource.symbol !== receiveResource.symbol) {
          const quantity =
            Math.floor(Math.random() * 5) +
            100 +
            (payResource.mintKey === 'igt' ? 10000 : 0)
          // console.log(
          //   `\tSwapping ${quantity} ${payResource.symbol} for ${receiveResource.symbol}.`,
          // )
          const [payMint] = PublicKey.findProgramAddressSync(
            [Buffer.from('mint_' + payResource.mintKey)],
            program.programId,
          )
          const [receiveMint] = PublicKey.findProgramAddressSync(
            [Buffer.from('mint_' + receiveResource.mintKey)],
            program.programId,
          )
          const payerPayTokenAccount =
            payResource.mintKey === 'igt'
              ? getAssociatedTokenAddressSync(payMint, playerWallet.publicKey)
              : PublicKey.findProgramAddressSync(
                [
                  Buffer.from('account_' + payResource.mintKey),
                  playerWallet.publicKey.toBuffer(),
                ],
                program.programId,
              )[0]

          const payerReceiveTokenAccount =
            receiveResource.mintKey === 'igt'
              ? getAssociatedTokenAddressSync(
                receiveMint,
                playerWallet.publicKey,
              )
              : PublicKey.findProgramAddressSync(
                [
                  Buffer.from('account_' + receiveResource.mintKey),
                  playerWallet.publicKey.toBuffer(),
                ],
                program.programId,
              )[0]

          await program.methods
            .marketPoolSwap(
              new anchor.BN(quantity),
              payResource.mintKey !== 'igt',
            )
            .accounts({
              payer: playerWallet.publicKey,
              payerPayTokenAccount,
              payerReceiveTokenAccount,
              payMint,
              receiveMint,
            })
            .signers([playerWallet])
            .rpc()
        }
      }
    }
    const resultingK = await getPoolData()
    logChangeInK(calculateChangeInK(initialK, resultingK))
  })
})
