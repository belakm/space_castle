import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair, PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { getPlayerBalances, usePlayer } from './utils/player'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'

describe('[Unit]: 👨 Player', () => {
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const provider = anchor.AnchorProvider.env()
  let playerWallet: Keypair
  let secondPlayerWallet: Keypair
  anchor.setProvider(provider)

  const [mintIGT] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_igt')],
    program.programId,
  )

  before('Prepare players', async () => {
    playerWallet = await usePlayer(1)
    secondPlayerWallet = await usePlayer(2)
  })

  it('New player can be created', async () => {
    const tokenAccount = getAssociatedTokenAddressSync(
      mintIGT,
      playerWallet.publicKey,
    )
    await program.methods
      .playerRegister('mico')
      .accounts({
        signer: playerWallet.publicKey,
        tokenAccount,
      })
      .signers([playerWallet])
      .rpc()
  })

  it('New player gets a token amount of iGT', async () => {
    const holdings = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
      'igt',
    )
    if (!holdings.igt || holdings.igt <= 0) {
      return assert.fail("Player wasn't credited")
    }
  })

  it('Activate resource accounts for player', async () => {
    await program.methods
      .playerCreateResourceAccountMetal()
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountCrystal()
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountChemical()
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountFuel()
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
  })

  it("Player can't have a name too long", async () => {
    const tokenAccount = getAssociatedTokenAddressSync(
      mintIGT,
      secondPlayerWallet.publicKey,
    )
    try {
      await program.methods
        .playerRegister('123456789012345678901234567890123')
        .accounts({
          signer: secondPlayerWallet.publicKey,
          tokenAccount,
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('32 max length for name doesnt work >:(')
    } catch {
      assert.ok('32 max length is working ok:)')
    }
  })

  it('Another Player', async () => {
    const tokenAccount = getAssociatedTokenAddressSync(
      mintIGT,
      secondPlayerWallet.publicKey,
    )
    await program.methods
      .playerRegister('mico 2')
      .accounts({
        signer: secondPlayerWallet.publicKey,
        tokenAccount,
      })
      .signers([secondPlayerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountMetal()
      .accounts({
        signer: secondPlayerWallet.publicKey,
      })
      .signers([secondPlayerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountCrystal()
      .accounts({
        signer: secondPlayerWallet.publicKey,
      })
      .signers([secondPlayerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountChemical()
      .accounts({
        signer: secondPlayerWallet.publicKey,
      })
      .signers([secondPlayerWallet])
      .rpc()
    await program.methods
      .playerCreateResourceAccountFuel()
      .accounts({
        signer: secondPlayerWallet.publicKey,
      })
      .signers([secondPlayerWallet])
      .rpc()
  })
})
