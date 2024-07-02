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

describe('[Test]: 🪐 Planet', () => {
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

  it('First planet can be claimed for free', async () => {
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
  })

  it('First free planet be claimed only once', async () => {
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

  it('Only planet without an owner can be claimed as free', async () => {
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

  it('Claims a planet with another user (required for tests that follow)', async () => {
    // Player 2
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

  it('Claiming a planet adds a token amount of IGT and resources to Player Cache', async () => {
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

  it('Player Cache can be claimed - IGT is deposited into wallet and resources into token accounts', async () => {
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

  it('Planet harvesting, this grants player IGT and resources', async () => {
    await program.methods
      .planetHarvest(1, 3)
      .accounts({ signer: playerWallet.keypair.publicKey })
      .signers([playerWallet.keypair])
      .rpc()
      .catch(console.log)

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
  })

  it('Planet can only be harvested by its owner', async () => {
    try {
      await program.methods
        .planetHarvest(1, 3)
        .accounts({ signer: secondPlayerWallet.keypair.publicKey })
        .signers([secondPlayerWallet.keypair])
        .rpc()
      return assert.fail('Failure')
    } catch (e) {
      return assert.ok('Ok')
    }
  })
})
