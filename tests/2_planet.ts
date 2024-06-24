import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { assert } from 'chai'
import {
  PlayerBalances,
  PlayerInfo,
  balanceDiff,
  getPlayerBalances,
  getPlayerCache,
  usePlayer,
} from './utils/player'
import { getHoldings, hasBuilding } from './utils/planet'

describe('[Unit]: 🪐 Planet', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: PlayerInfo
  let secondPlayerWallet: PlayerInfo
  let playerBalances: PlayerBalances

  before('Prepare wallet and player accounts', async () => {
    playerWallet = await usePlayer(1, program.programId)
    secondPlayerWallet = await usePlayer(2, program.programId)
    playerBalances = await getPlayerBalances(
      playerWallet.keypair,
      program.programId,
      provider,
    )
  })

  it("Can't claim a planet where there is no planet", async () => {
    try {
      await program.methods
        .planetFirstClaim(1, 4)
        .accounts({
          signer: secondPlayerWallet.keypair.publicKey,
        })
        .signers([secondPlayerWallet.keypair])
        .rpc()
      return assert.fail('Could settle first planet at invalid position')
    } catch (e) {
      return assert.ok("Can't settle first planet where there is none")
    }
  })

  it('Player with no planets can claim the first planet', async () => {
    // Player 1
    await program.methods
      .planetFirstClaim(1, 3)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })

    await program.methods
      .planetFirstClaim(2, 6)
      .accounts({
        signer: secondPlayerWallet.keypair.publicKey,
      })
      .signers([secondPlayerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })

  it("Can't claim already claimed planet", async () => {
    try {
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: secondPlayerWallet.keypair.publicKey,
        })
        .signers([secondPlayerWallet.keypair])
        .rpc()
      return assert.fail('Could claim an already claimed planet')
    } catch {
      return assert.ok("Can't claim an already claimed planet")
    }
  })

  it('Player is awarded a token amount of resources when claiming the first planet', async () => {
    const player_cache = await getPlayerCache(
      playerWallet.keypair.publicKey,
      program,
    )
    if (
      player_cache.resources.igt.toNumber() > 0 && // .gt(new BN(0)) &&
      player_cache.resources.metal.toNumber() > 0 &&
      player_cache.resources.crystal.toNumber() > 0 &&
      player_cache.resources.chemical.toNumber() > 0 &&
      player_cache.resources.fuel.toNumber() > 0
    ) {
      return assert.ok('Player received all resources')
    } else return assert.fail('Player did not receive all resources')
  })

  it('Player can claim Player Cache (it has resources granted by claiming first planet)', async () => {
    try {
      await program.methods
        .playerClaimResourceCache()
        .accounts({
          signer: playerWallet.keypair.publicKey,
        })
        .signers([playerWallet.keypair])
        .rpc()

      // Also claim second player cache so he has some resources later on
      await program.methods
        .playerClaimResourceCache()
        .accounts({
          signer: secondPlayerWallet.keypair.publicKey,
        })
        .signers([secondPlayerWallet.keypair])
        .rpc()

      const oldBalances = { ...playerBalances }
      playerBalances = await getPlayerBalances(
        playerWallet.keypair,
        program.programId,
        provider,
      )
      const diff = balanceDiff(playerBalances, oldBalances)
      if (
        diff.igt > 0 &&
        diff.metal > 0 &&
        diff.crystal > 0 &&
        diff.chemical > 0 &&
        diff.fuel > 0
      ) {
        return assert.ok('Got resources')
      } else {
        return assert.fail('Not all resources were given')
      }
    } catch (e) {
      return assert.fail(e)
    }
  })

  // TODO: Add this test
  it("TODO: Can't harvest planet if enough time hasnt passed", async () => {})

  // TODO: Add this test
  it('TODO: After enough slots passed, you can harvest again', async () => {})

  it('Planet has starting buildings', async () => {
    const holdings = await getHoldings(
      1,
      3,
      playerWallet.keypair.publicKey,
      program,
    )
    const hasAllTheRightBuildings =
      hasBuilding(holdings, 'planetaryCapital') &&
      hasBuilding(holdings, 'shipyard') &&
      (hasBuilding(holdings, 'crystalLabs') ||
        hasBuilding(holdings, 'metalIndustry') ||
        hasBuilding(holdings, 'chemicalRefinery'))
    if (!hasAllTheRightBuildings) {
      return assert.fail(`Missing the three starter buildings`)
    }
  })

  it("Can't claim another planet as first planet", async () => {
    try {
      await program.methods
        .planetFirstClaim(2, 20)
        .accounts({
          signer: playerWallet.keypair.publicKey,
        })
        .signers([playerWallet.keypair])
        .rpc()
      return assert.fail('Could settle second planet as first planet.')
    } catch {
      return assert.ok("Couldn't settle second planet as first planet.")
    }
  })

  // TODO: Add this test
  it("TODO: Player can't harvest another player's planet", async () => {})
})
