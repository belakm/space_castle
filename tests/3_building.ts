import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair } from '@solana/web3.js'
import { assert } from 'chai'
import { balanceDiff, getPlayerBalances, usePlayer } from './utils/player'

describe('[Unit]: Buildings', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: Keypair

  before('Prepare wallet and player account', async () => {
    playerWallet = await usePlayer(1)
  })

  it('Buildings can be upgraded, player pays with resources', async () => {
    const balances1 = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    await program.methods
      .planetUpgradeBuilding(1, 3, { planetaryCapital: {} })
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        console.error(e)
        return assert.fail(e)
      })
    const balances2 = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(balances1, balances2)
    if (diff.fuel === 0) {
      assert.fail('Looks like no resources were used up.')
    }
  })
})
