import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair, PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { getPlayerBalances, resourceKeys, usePlayer } from './utils/player'
import { getHoldings, hasBuilding } from './utils/planet'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'

describe('[Unit]: 🪐 Planet', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: Keypair
  let secondPlayerWallet: Keypair
  const [mintIGT] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_igt')],
    program.programId,
  )

  before('Prepare wallet and player accounts', async () => {
    playerWallet = await usePlayer(1)
    secondPlayerWallet = await usePlayer(2)
  })

  it('Player with no planets can claim the first planet', async () => {
    await program.methods
      .planetFirstClaim(1, 3)
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })

  it('Player is awarded a token amount of resources when harvesting for the first time', async () => {
    const tokenAccount = getAssociatedTokenAddressSync(
      mintIGT,
      secondPlayerWallet.publicKey,
    )
    await program.methods
      .planetHarvest(1, 3)
      .accounts({
        signer: playerWallet.publicKey,
        igtTokenAccount: tokenAccount,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })

    const holdings = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    for (const r of resourceKeys) {
      if (r != 'igt' && holdings[r] <= 0) {
        return assert.fail('Missing resources after claim')
      }
    }
  })

  // TODO: Add this test
  it('TODO: Cant harvest planet if enough time hasnt passed', async () => {})

  // TODO: Add this test
  it('TODO: After N cycles, you can harvest again', async () => {})

  it('Planet has starting buildings', async () => {
    const holdings = await getHoldings(1, 3, playerWallet.publicKey, program)
    const hasAllTheRightBuildings =
      hasBuilding(holdings, 'planetaryCapital') &&
      hasBuilding(holdings, 'shipyard') &&
      (hasBuilding(holdings, 'crystalLabs') ||
        hasBuilding(holdings, 'metalIndustry') ||
        hasBuilding(holdings, 'chemicalRefinery'))
    if (!hasAllTheRightBuildings) {
      assert.fail(`Missing the three starter buildings`)
    }
  })

  it("Can't claim already claimed planet", async () => {
    try {
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: secondPlayerWallet.publicKey,
        })
        .signers([secondPlayerWallet])
        .rpc()
      return assert.fail('Could claim an already claimed planet')
    } catch {
      return assert.ok("Can't claim an already claimed planet")
    }
  })

  it("Can't claim a planet where there is no planet", async () => {
    try {
      await program.methods
        .planetFirstClaim(1, 4)
        .accounts({
          signer: secondPlayerWallet.publicKey,
        })
        .signers([secondPlayerWallet])
        .rpc()
      return assert.fail('Could settle first planet at invalid position')
    } catch (e) {
      return assert.ok("Can't settle first planet where there is none")
    }
  })

  it("Can't claim another planet as first planet", async () => {
    try {
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: playerWallet.publicKey,
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('Could settle second planet as first planet.')
    } catch {
      assert.ok("Couldn't settle second planet as first planet.")
    }
  })

  // TODO: Add this test
  it('TODO: Player cant harvest another players planet', async () => {})
})
