import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair, PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { createAndFundWallet } from './utils/wallet'

describe('[Unit] Planet', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: Keypair
  let secondPlayerWallet: Keypair

  before('Prepare wallet and player accounts', async () => {
    playerWallet = await createAndFundWallet()
    secondPlayerWallet = await createAndFundWallet()

    await program.methods
      .playerRegister('mico')
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()

    await program.methods
      .playerRegister('mico 2')
      .accounts({
        signer: secondPlayerWallet.publicKey,
      })
      .signers([secondPlayerWallet])
      .rpc()
  })

  it('Player with no planets can claim the first planet', async () => {
    const [playerInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from('player'), playerWallet.publicKey.toBuffer()],
      program.programId,
    )
    await program.methods
      .planetFirstClaim(1, 3)
      .accounts({
        signer: playerWallet.publicKey,
        playerInfo,
      })
      .signers([playerWallet])
      .rpc()
  })

  it("Can't claim already claimed planet", async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
        program.programId,
      )
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: secondPlayerWallet.publicKey,
          playerInfo,
        })
        .signers([secondPlayerWallet])
        .rpc()
      assert.fail('Could claim an already claimed planet')
    } catch {
      assert.ok("Can't claim an already claimed planet")
    }
  })

  it("Can't claim a planet where there is no planet", async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
        program.programId,
      )
      await program.methods
        .planetFirstClaim(1, 4)
        .accounts({
          signer: secondPlayerWallet.publicKey,
          playerInfo,
        })
        .signers([secondPlayerWallet])
        .rpc()
      assert.fail('Could settle first planet at invalid position')
    } catch (e) {
      assert.ok("Can't settle first planet where there is none")
    }
  })

  it("Can't claim another planet as first planet", async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), playerWallet.publicKey.toBuffer()],
        program.programId,
      )
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: playerWallet.publicKey,
          playerInfo,
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('Could settle second planet as first planet.')
    } catch {
      assert.ok("Couldn't settle second planet as first planet.")
    }
  })
})
