import * as anchor from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { MARKET_RESOURCES } from './utils/resources'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'

describe('[Unit] Market pool', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>
  const payer = (provider.wallet as anchor.Wallet).payer
  const poolAddress = PublicKey.findProgramAddressSync(
    [Buffer.from('market_pool')],
    program.programId,
  )[0]
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

  it('Init market pool', async () => {
    if (poolInitialized) {
      return assert.ok('Pool exists already.')
    }
    const txSig = await program.methods
      .marketPoolCreate()
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
    console.log(`Transaction Signature: ${txSig}`)
  })

  it('Mint resources to pool', async () => {
    for (const resource of MARKET_RESOURCES) {
      console.log(`\n\tMinting ${resource.mintKey} to the market pool\n`)
      const mint = PublicKey.findProgramAddressSync(
        [Buffer.from('mint_' + resource.mintKey)],
        program.programId,
      )
      const ata = getAssociatedTokenAddressSync(mint[0], poolAddress, true)
      const txSig = await program.methods
        .marketPoolMintTo(new anchor.BN(resource.quantity), resource.mintKey)
        .accounts({
          mint: mint[0],
          poolTokenAccount: ata,
          payer: payer.publicKey,
        })
        .signers([payer])
        .rpc()

      console.log(`Transaction Signature: ${txSig}`)

      const balance = await provider.connection.getTokenAccountBalance(ata)
      console.log(`Balance: ${balance.value.uiAmount} ${resource.symbol}`)
    }
  })

  it('Try swapping resources for IGT', async () => { })

  it('Try swapping IGT for resources', async () => { })

  it('Try swapping resources for resources', async () => { })
})
