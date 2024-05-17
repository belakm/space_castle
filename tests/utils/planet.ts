import * as anchor from '@coral-xyz/anchor'
import { PublicKey } from '@solana/web3.js'
import { SpaceCastle } from '../../target/types/space_castle'

// prettier-ignore
type AllPossibleKeys<T> =
  T extends Array<infer U>
  ? keyof U | (U extends { buildingType: infer V } ? keyof V : never)
  : never

export type BuildingType = Exclude<
  AllPossibleKeys<Awaited<ReturnType<typeof getHoldings>>['buildings']>,
  'level'
>

export type PlanetHolding = Awaited<ReturnType<typeof getHoldings>>

export async function getHoldings(
  x: number,
  y: number,
  publicKey: PublicKey,
  program: anchor.Program<SpaceCastle>,
) {
  const xBuffer = Buffer.alloc(2) // Allocate 2 bytes for u16
  const yBuffer = Buffer.alloc(2) // Allocate 2 bytes for u16
  xBuffer.writeUInt16LE(x, 0) // Little-endian format
  yBuffer.writeUInt16LE(y, 0) // Little-endian format
  const [planetHoldingPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from('planet_holding'), publicKey.toBuffer(), xBuffer, yBuffer],
    program.programId,
  )
  const holding = await program.account.planetHolding.fetch(planetHoldingPDA)
  return holding
}

export function getBuilding(
  holding: PlanetHolding,
  buildingType: BuildingType,
) {
  return holding.buildings.find((b) => b.buildingType[buildingType] != null)
}

export function hasBuilding(
  holding: PlanetHolding,
  buildingType: BuildingType,
) {
  return holding.buildings.find((b) => b.buildingType[buildingType]) != null
}
