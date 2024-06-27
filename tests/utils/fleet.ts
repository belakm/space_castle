import { type Program } from '@coral-xyz/anchor'
import { SpaceCastle } from '../../target/types/space_castle'
import * as anchor from '@coral-xyz/anchor'
import { PublicKey } from '@solana/web3.js'

const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>

/**
 * # Fleet composition:
 * `Squadron`
 *    - amount
 *    - template: `Ship`
 *      - amount
 *      - template: `ShipModule`
 *        - name
 *        - level
 */
export type FleetBlueprint = Parameters<typeof program.methods.fleetNew>[2]
/// How one blueprint for ships in squadron with amount is structured
export type SquadronBlueprint = Exclude<FleetBlueprint[0], null>
/// Template for a ship
export type ShipType = SquadronBlueprint['template']
/// Module in a ship
export type ShipModuleType = ShipType[0]
/// Keys of modules in a ship
export type ShipModuleName = keyof ShipModuleType['moduleType']

// Create a smiple fleet with one squadron of 3 ships with 3 machine guns each
export const createSimpleFleetTemplate = () =>
  constructFleet([
    [
      padShipModules([
        ['machineGun', 1],
        ['machineGun', 1],
        ['machineGun', 1],
      ]),
      3,
    ],
  ])

export const padShipModules = (shipModules: Array<[ShipModuleName, number]>) =>
  shipModules.concat(
    Array(6 - shipModules.length).fill(['none' as ShipModuleName, 0]),
  )

export const constructFleet = (
  squadrons: [[ShipModuleName, number][], number][],
) => {
  const full_squadrons = squadrons.map(
    ([ship, amount]) =>
      ({
        amount,
        template: constructShip(ship),
      }) as SquadronBlueprint,
  ) as FleetBlueprint
  const padded_squadrons = full_squadrons.concat(
    Array(9 - full_squadrons.length).fill(null),
  ) as FleetBlueprint
  return padded_squadrons
}

export const constructShip = (modules: [ShipModuleName, number][]) => {
  return modules.map(([module, level]) => constructShipModule(module, level))
}

export const constructShipModule = (type: ShipModuleName, level: number) => {
  const module = {}
  module[type] = {}
  return {
    moduleType: module,
    level,
  }
}

export type Fleet = Awaited<ReturnType<typeof getFleet>>

export async function getFleet(
  x: number,
  y: number,
  program: anchor.Program<SpaceCastle>,
) {
  const xBuffer = Buffer.alloc(2) // Allocate 2 bytes for u16
  const yBuffer = Buffer.alloc(2)
  xBuffer.writeUInt16LE(x, 0) // Little-endian format
  yBuffer.writeUInt16LE(y, 0)
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from('fleet'), xBuffer, yBuffer],
    program.programId,
  )
  try {
    const fleet = await program.account.fleet.fetch(pda)
    return Promise.resolve(fleet)
  } catch (e) {
    return Promise.reject(e)
  }
}

export const printFleet = (fleet: Fleet) => {
  console.log('------------ FLEET -------------')
  console.log(`  Present: ${fleet.isPresent}`)
  console.log(`  Owner: ${fleet.owner}`)
  console.log(`  Composition: `)
  fleet.squadrons.forEach((squadron, index) => {
    if (squadron) {
      const modules = squadron.template.reduce(
        (modules, module) =>
          module.moduleType.none
            ? modules
            : `${modules}${modules === '' ? '' : ' | '}${Object.keys(module.moduleType)[0]} lvl. ${module.level}`,
        '',
      )
      console.log(`    Squadron ${index}`)
      console.log(`      ${modules}`)
    }
  })
  console.log('')
}

export const fleetSufferedLosses = (before: Fleet, after: Fleet) => {
  const amountsBefore = before.squadrons.reduce(
    (amount, squadron) => (amount += squadron?.amount ?? 0),
    0,
  )
  const amountsAfter = after.squadrons.reduce(
    (amount, squadron) => (amount += squadron?.amount ?? 0),
    0,
  )
  return amountsAfter !== amountsBefore
}

export const fleetKey = (x: number, y: number) => {
  const xBuffer2 = Buffer.alloc(2) // Allocate 2 bytes for u16
  const yBuffer2 = Buffer.alloc(2)
  xBuffer2.writeUInt16LE(x, 0) // Little-endian format
  yBuffer2.writeUInt16LE(y, 0)
  const [fleet] = PublicKey.findProgramAddressSync(
    [Buffer.from('fleet'), xBuffer2, yBuffer2],
    program.programId,
  )
  return fleet
}
