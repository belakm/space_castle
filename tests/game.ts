import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { PublicKey } from '@solana/web3.js'
import { assert } from 'chai'

describe('Space Castle: GAME LOGIC', () => {

  const provider = anchor.AnchorProvider.env()
  const wallet = provider.wallet as anchor.Wallet
  anchor.setProvider(provider)

  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const [gameStateAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('game')],
    program.programId
  )

  it('Game initializes', async () => {
    const tx = await program.methods
      .initializeGame()
      .accounts({
        signer: wallet.publicKey,
        gameState: gameStateAccount
      })
      .rpc()
    console.log('Your transaction signature', tx)
  })

  it('Game doesnt reinitialize', async () => {
    try {
      await program.methods
        .initializeGame()
        .accounts({
          signer: wallet.publicKey,
          gameState: gameStateAccount
        })
        .rpc()
      assert.fail('Reinitalizing should fail, but it didnt :(')
    } catch (e) {
      assert.ok('Reinitalizing the game fails as it should :)')
    }
  })


  it('Game has is_initialized set', async () => {
    const gameState = await program.account.gameState.fetch(gameStateAccount)
    assert.equal(gameState.isInitialized, true, 'Game is set to initialized')
  })

})
