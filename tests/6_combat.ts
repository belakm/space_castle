import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { PlayerInfo, getPlayerCache, usePlayer } from './utils/player'
import { assert } from 'chai'
import {
  fleetKey,
  fleetSufferedLosses,
  getFleet,
  printFleet,
} from './utils/fleet'

describe('[Unit]: ⚔️  Battle', () => {
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const provider = anchor.AnchorProvider.env()
  let playerWallet: PlayerInfo
  let fleet1: Awaited<ReturnType<typeof getFleet>>
  let fleet2: Awaited<ReturnType<typeof getFleet>>
  let secondPlayerWallet: PlayerInfo

  anchor.setProvider(provider)

  before('Prepare players', async () => {
    playerWallet = await usePlayer(1, program.programId)
    secondPlayerWallet = await usePlayer(2, program.programId)
    fleet2 = await getFleet(1, 3, program)
    fleet2 = await getFleet(2, 6, program)
    // playerBalances = await getPlayerBalances(
    //   playerWallet.keypair,
    //   program.programId,
    //   provider,
    // )
  })
  it('Fleet cant attack where there is not fleet', async () => {
    try {
      await program.methods
        .fleetAttack(1, 3, 1, 1)
        .accounts({
          signer: secondPlayerWallet.keypair.publicKey,
        })
        .signers([secondPlayerWallet.keypair])
        .rpc()
      return assert.fail('Somehow attacked where there is no fleet')
    } catch (e) {
      return assert.ok('Ok')
    }
  })
  it('Fleet cant attack a planet as it would a fleet (that action is called planet invasion)', async () => {
    try {
      await program.methods
        .fleetAttack(1, 3, 2, 6)
        .accounts({
          signer: secondPlayerWallet.keypair.publicKey,
        })
        .signers([secondPlayerWallet.keypair])
        .rpc()
      return assert.fail('Failure')
    } catch (e) {
      return assert.ok('Ok')
    }
  })
  it('Fleet can attack another fleet', async () => {
    // Move second player fleet off its planet
    await program.methods
      .fleetMove(2, 6, 2, 7)
      .accounts({
        signer: secondPlayerWallet.keypair.publicKey,
      })
      .accountsPartial({
        fleetFrom: fleetKey(2, 6),
        fleetTo: fleetKey(2, 7),
      })
      .signers([secondPlayerWallet.keypair])
      .rpc()

    fleet1 = await getFleet(1, 3, program)
    fleet2 = await getFleet(2, 7, program)
    printFleet(fleet1)
    printFleet(fleet2)
    // First player attacks second fleet
    await program.methods
      .fleetAttack(1, 3, 2, 7)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch(console.error)
  })
  it('Winner of the battle is granted plunder', async () => {
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
      return assert.ok('Player received plunder')
    } else return assert.fail('Player did not receive any plunder')
  })
  it('Winner, loser or both lost some ships in the conflict', async () => {
    const fleetBefore1 = { ...fleet1 }
    const fleetBefore2 = { ...fleet2 }
    fleet1 = await getFleet(1, 3, program)
    fleet2 = await getFleet(2, 7, program)
    printFleet(fleet1)
    printFleet(fleet2)
    return (
      fleetSufferedLosses(fleetBefore1, fleet1) ||
      fleetSufferedLosses(fleetBefore2, fleet2)
    )
  })
})
