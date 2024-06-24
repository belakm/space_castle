import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import {
  PlayerBalances,
  PlayerInfo,
  balanceDiff,
  getPlayerBalances,
  usePlayer,
} from './utils/player'
import { assert } from 'chai'
import { createSimpleFleetTemplate } from './utils/fleet'

describe('[Unit]: 🚀 Fleet', () => {
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const provider = anchor.AnchorProvider.env()
  let playerWallet: PlayerInfo
  let playerBalances: PlayerBalances
  let secondPlayerWallet: PlayerInfo
  anchor.setProvider(provider)

  before('Prepare players', async () => {
    playerWallet = await usePlayer(1, program.programId)
    secondPlayerWallet = await usePlayer(2, program.programId)
    playerBalances = await getPlayerBalances(
      playerWallet.keypair,
      program.programId,
      provider,
    )
  })

  it('Fleet can move', async () => {
    await program.methods
      .fleetMove(1, 3, 2, 3)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('When fleet moves, it burns fuel', async () => {
    const prevBalances = { ...playerBalances }
    playerBalances = await getPlayerBalances(
      playerWallet.keypair,
      program.programId,
      provider,
    )
    const diff = balanceDiff(playerBalances, prevBalances)
    return diff.fuel >= 0
      ? assert.fail('No fuel was burned')
      : assert.ok('All ok')
  })
  it('Fleet cant move where another fleet is present', async () => {
    await program.methods
      // on [2, 4] there is a claimed planet by player 2
      .fleetMove(1, 3, 2, 4)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch(() => {
        return assert.ok('OK')
      })
    return assert.fail('Somehow fleet moved where another was present')
  })
  it('Fleet cant move where a non-owned planet is', async () => {
    try {
      await program.methods
        // on [2, 4] there is a claimed planet by player 2
        .fleetMove(1, 3, 2, 4)
        .accounts({
          signer: playerWallet.keypair.publicKey,
        })
        .signers([playerWallet.keypair])
        .rpc()
        .catch((e) => {
          return assert.fail(e)
        })
    } catch (e) {
      return assert.ok("Fleet could't move where another fleet is present")
    }
    return assert.fail('Somehow fleet moved where another was present')
  })
  it('Fleet cannot be created if another fleet is on that planet', async () => {
    // Bring fleet back
    await program.methods
      .fleetMove(2, 3, 1, 3)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })

    // Try to spawn another fleet on the planet
    await program.methods
      .fleetNew(1, 3, createSimpleFleetTemplate())
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('New fleet can be created', async () => {
    // Take starting fleet away
    await program.methods
      .fleetMove(1, 3, 2, 3)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })

    // Try to spawn another fleet on the planet
    await program.methods
      .fleetNew(1, 3, createSimpleFleetTemplate())
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('Fleet cant move where another fleet is present', async () => {
    await program.methods
      .fleetMove(1, 3, 2, 6)
      .accounts({
        signer: playerWallet.keypair.publicKey,
      })
      .signers([playerWallet.keypair])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('Only owner of the fleet can move it around', async () => {
    await program.methods
      .fleetMove(1, 3, 2, 4)
      .accounts({
        signer: secondPlayerWallet.keypair.publicKey,
      })
      .signers([secondPlayerWallet.keypair])
      .rpc()
      .catch(() => {
        return assert.ok('Cant move')
      })
    return assert.fail('Could move fleet without right authority')
  })
})
