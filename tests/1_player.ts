import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair, PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { createAndFundWallet } from './utils'

describe('Space Castle: PLAYER', () => {
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const provider = anchor.AnchorProvider.env()
  let playerWallet: Keypair
  let secondPlayerWallet: Keypair
  anchor.setProvider(provider)

  before('Prepare wallets', async () => {
    playerWallet = await createAndFundWallet()
    secondPlayerWallet = await createAndFundWallet()
  })

  it('New player can be created', async () => {
    const [playerAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('player'), playerWallet.publicKey.toBuffer()],
      program.programId
    )
    await program.methods
      .registerPlayer('mico')
      .accounts({
        signer: playerWallet.publicKey,
        player: playerAccount
      })
      .signers([playerWallet])
      .rpc()
  })

  it('Player can\'t have a name too long', async () => {
    try {
      const [playerAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
        program.programId
      )
      await program.methods
        .registerPlayer('123456789012345678901234567890123')
        .accounts({
          signer: secondPlayerWallet.publicKey,
          player: playerAccount
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('32 max length for name doesnt work >:(')
    } catch {
      assert.ok('32 max length is working ok:)')
    }
  })

})
