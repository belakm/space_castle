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

describe('[Unit]: ⚔️  Battle', () => {
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const provider = anchor.AnchorProvider.env()
  let playerWallet: PlayerInfo
  // let playerBalances: PlayerBalances
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
  it('Fleet cant attack where there is not fleet', async () => {
    await program.methods
      .fleetAttack(1, 3, 1, 1)
      .accounts({
        signer: secondPlayerWallet.keypair.publicKey,
      })
      .signers([secondPlayerWallet.keypair])
      .rpc()
      .catch(() => {
        return assert.ok('Couldnt attack where there is no fleet')
      })
    return assert.fail('Somehow attacked where there is no fleet')
  })
  it('Fleet cant attack a planet as it would a fleet (that action is called planet invasion)', async () => {
    await program.methods
      .fleetAttack(1, 3, 2, 6)
      .accounts({
        signer: secondPlayerWallet.keypair.publicKey,
      })
      .signers([secondPlayerWallet.keypair])
      .rpc()
      .catch(() => {
        return assert.ok('Couldnt attack where there is no fleet')
      })
    return assert.fail('Somehow attacked where there is no fleet')
  })
  it('Fleet can attack another fleet', async () => {
    // Move second player fleet off its planet
    // First player attacks second fleet
  })
  it('Winner of the battle is granted resources', async () => {})
  it('Winner and loser both lost some ships in the conflict', async () => {})
})
