import * as anchor from '@coral-xyz/anchor'
import { PublicKey } from '@solana/web3.js'
import { SpaceCastle } from '../../target/types/space_castle'

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
  holding: Awaited<ReturnType<typeof getHoldings>>,
  buildingType: string,
) {
  return holding.buildings.find((b) => b[buildingType])
}

export function hasBuilding(
  buildings: Record<string, unknown>,
  buildingType: string,
) {
  return Object.keys(buildings).includes(buildingType)
}
