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
import { mintAllResourcesToAddress } from './utils/token'

describe('[Test]: 🏰 Buildings', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: Keypair
  let latestBalance: PlayerBalances
  let latestHolding: Awaited<ReturnType<typeof getHoldings>>

  before('Prepare wallet and player account', async () => {
    playerWallet = (await usePlayer(1, program.programId)).keypair
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    await mintAllResourcesToAddress(playerWallet)
  })

  it('Planet starts with basic buildings - Planetary Capital, Shipyard and either: Crystal Labs, Metal Industry, Chemical Refinery', async () => {
    const holdings = await getHoldings(1, 3, playerWallet.publicKey, program)
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

  it('New buildings can be built on the planet', async () => {
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

    const holding = await getHoldings(1, 3, playerWallet.publicKey, program)
    if (!holding || !hasBuilding(holding, 'astralNavyHq')) {
      return assert.fail('Building was not built')
    }
  })

  it('New buildings are paid with resources', async () => {
    const lastBalance = { ...latestBalance }
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(lastBalance, latestBalance)
    if (diff.fuel === 0) {
      return assert.fail('Looks like no resources were used up.')
    }
  })

  it('Buildings can be upgraded, this increases their level', async () => {
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

    const holding = await getHoldings(1, 3, playerWallet.publicKey, program)
    if (!holding || !(getBuilding(holding, 'astralNavyHq')?.level === 2)) {
      return assert.fail('Building was not upgraded')
    }
  })

  it('Upgrading a building costs resources', async () => {
    const lastBalance = { ...latestBalance }
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(lastBalance, latestBalance)
    if (diff.fuel === 0) {
      return assert.fail('Looks like no resources were used up.')
    }
  })

  it('Buildings can be changed into another building type', async () => {
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

    const holding = await getHoldings(1, 3, playerWallet.publicKey, program)
    if (
      !holding ||
      !(
        hasBuilding(holding, 'fuelExtractors') &&
        !hasBuilding(holding, 'astralNavyHq')
      )
    ) {
      return assert.fail('Building was not changed')
    }
  })

  it('Changing a building halves its level', async () => {
    const buildings = await getHoldings(1, 3, playerWallet.publicKey, program)
    const oldBuilding = getBuilding(latestHolding, 'astralNavyHq')
    const newBuilding = getBuilding(buildings, 'fuelExtractors')
    if (
      !(oldBuilding && newBuilding && oldBuilding.level > newBuilding.level)
    ) {
      return assert.fail('Building did not lose levels when upgraded')
    }
  })

  it('Changing a building costs resources', async () => {
    const lastBalance = { ...latestBalance }
    latestBalance = await getPlayerBalances(
      playerWallet,
      program.programId,
      provider,
    )
    const diff = balanceDiff(lastBalance, latestBalance)
    if (diff.fuel === 0) {
      return assert.fail('Looks like no resources were used up.')
    }
  })
})
