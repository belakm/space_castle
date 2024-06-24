import { type Program } from '@coral-xyz/anchor'
import { SpaceCastle } from '../../target/types/space_castle'

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
      [
        ['machineGun', 1],
        ['machineGun', 1],
        ['machineGun', 1],
      ],
      3,
    ],
  ])

export const constructFleet = (
  squadrons: [[ShipModuleName, number][], number][],
) => {
  return squadrons.map(
    ([ship, amount]) =>
      ({
        amount,
        template: constructShip(ship),
      }) as SquadronBlueprint,
  ) as FleetBlueprint
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
