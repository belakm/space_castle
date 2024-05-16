import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair } from '@solana/web3.js'
import { assert } from 'chai'
import {
  PlayerBalances,
  balanceDiff,
  getPlayerBalances,
  usePlayer,
} from './utils/player'
import { getBuilding, getHoldings, hasBuilding } from './utils/planet'

describe('[Unit]: Buildings', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: Keypair
  let latestBalance: PlayerBalances
  let latestHolding: Awaited<ReturnType<typeof getHoldings>>

  before('Prepare wallet and player account', async () => {
    playerWallet = await usePlayer(1)
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
  })

  it('Player can build a new building on the planet', async () => {
    await program.methods
      .planetBuildingNew(1, 3, { astralNavyHq: {} })
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        console.error(e)
        return assert.fail(e)
      })

    const buildings = await getHoldings(1, 3, playerWallet.publicKey, program)
    if (!buildings || !hasBuilding(buildings, 'astralNavyHq')) {
      return assert.fail('Building not on the planet')
    }
  })

  it('Player paid resources for the new building', async () => {
    const lastBalance = { ...latestBalance }
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(lastBalance, latestBalance)
    if (diff.fuel === 0) {
      assert.fail('Looks like no resources were used up.')
    }
  })

  it('Player can upgrade a building on the planet', async () => {
    await program.methods
      .planetBuildingUpgrade(1, 3, { astralNavyHq: {} })
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        console.error(e)
        return assert.fail(e)
      })

    const buildings = await getHoldings(1, 3, playerWallet.publicKey, program)
    if (!buildings || !(getBuilding(buildings, 'astralNavyHq')?.level === 2)) {
      return assert.fail('Building was not upgraded')
    }
  })

  it('Player paid resources for the upgrade of the building', async () => {
    const lastBalance = { ...latestBalance }
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(lastBalance, latestBalance)
    if (diff.fuel === 0) {
      assert.fail('Looks like no resources were used up.')
    }
  })

  it('Player can change a building into another building', async () => {
    latestHolding = await getHoldings(1, 3, playerWallet.publicKey, program)
    await program.methods
      .planetBuildingChange(1, 3, { astralNavyHq: {} }, { fuelExtractors: {} })
      .accounts({
        signer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        console.error(e)
        return assert.fail(e)
      })

    const buildings = await getHoldings(1, 3, playerWallet.publicKey, program)
    if (
      !buildings ||
      !(
        hasBuilding(buildings, 'fuelExtractors') &&
        hasBuilding(buildings, 'astralNavyHq')
      )
    ) {
      return assert.fail('Building was not changed')
    }
  })

  it('New changed building has half its levels when upgraded', async () => {
    const buildings = await getHoldings(1, 3, playerWallet.publicKey, program)
    const oldBuilding = getBuilding(latestHolding, 'astralNavyHq')
    const newBuilding = getBuilding(buildings, 'fuelExtractors')
    if (
      !(oldBuilding && newBuilding && oldBuilding.level > newBuilding.level)
    ) {
      return assert.fail('Building was not upgraded')
    }
  })

  it('Player paid resources for the change of the building', async () => {
    const lastBalance = { ...latestBalance }
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(lastBalance, latestBalance)
    if (diff.fuel === 0) {
      assert.fail('Looks like no resources were used up.')
    }
  })
})
