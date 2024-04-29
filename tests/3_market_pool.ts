import * as anchor from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { PublicKey, Keypair } from '@solana/web3.js'
import { assert } from 'chai'
import { MARKET_RESOURCES } from './utils/resources'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'
import { createAndFundWallet } from './utils/wallet'
import { mintAllResourcesToAddress } from './utils/token'

describe('[Unit] Market pool', () => {
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
      playerWallet = await createAndFundWallet()
      mintAllResourcesToAddress(playerWallet)
    },
  )

  it('Init market pool', async () => {
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

  it('Mint resources to pool', async () => {
    for (const resource of MARKET_RESOURCES) {
      const mint = PublicKey.findProgramAddressSync(
        [Buffer.from('mint_' + resource.mintKey)],
        program.programId,
      )
      const ata = getAssociatedTokenAddressSync(mint[0], poolAddress, true)
      await program.methods
        .marketPoolMintTo(new anchor.BN(resource.quantity), resource.mintKey)
        .accounts({
          mint: mint[0],
          poolTokenAccount: ata,
          payer: payer.publicKey,
        })
        .signers([payer])
        .rpc()
      const balance = await provider.connection.getTokenAccountBalance(ata)
      console.log(
        `\tMinted: ${balance.value.uiAmount} ${resource.symbol} to market pool`,
      )
    }
  })

  it('Swapping resources for IGT', async () => {})

  it('Swapping IGT for resources', async () => {})

  it('Swapping resources for resources', async () => {})
})
