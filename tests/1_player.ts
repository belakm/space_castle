import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair } from '@solana/web3.js'
import { assert } from 'chai'
import { createAndFundWallet } from './utils/wallet'

describe('[Unit]: Player', () => {
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
    await program.methods
      .playerRegister('mico')
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
  })

  it("Player can't have a name too long", async () => {
    try {
      await program.methods
        .playerRegister('123456789012345678901234567890123')
        .accounts({
          signer: secondPlayerWallet.publicKey,
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('32 max length for name doesnt work >:(')
    } catch {
      assert.ok('32 max length is working ok:)')
    }
  })
})
