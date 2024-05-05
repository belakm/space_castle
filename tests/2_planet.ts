import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair, PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { getPlayerHoldings, usePlayer } from './utils/player'
import { MARKET_RESOURCES } from './utils/resources'
import { lineBreak, logPlayerHoldings } from './utils/log'

describe('[Unit]: Planet', () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  let playerWallet: Keypair
  let secondPlayerWallet: Keypair

  before('Prepare wallet and player accounts', async () => {
    playerWallet = await usePlayer(1)
    secondPlayerWallet = await usePlayer(2)
  })

  it('Player with no planets can claim the first planet', async () => {
    const [playerInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from('player'), playerWallet.publicKey.toBuffer()],
      program.programId,
    )
    const tx = await program.methods
      .planetFirstClaim(1, 3)
      .accounts({
        signer: playerWallet.publicKey,
        playerInfo,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        console.error(e)
        return assert.fail(e)
      })

    console.log(tx)
  })

  it('Player is awarded a token amount of resources when claiming the first planet', async () => {
    const holdings = await getPlayerHoldings(
      playerWallet,
      program.programId,
      provider,
    )
    for (const r of MARKET_RESOURCES) {
      if (
        ['metal', 'chemical'].includes(r.mintKey) &&
        holdings[r.mintKey] <= 0
      ) {
        return assert.fail('Missing resources after claim')
      }
    }
    await logPlayerHoldings(playerWallet, program.programId, provider)
    return assert.ok('Has resources')
  })

  it('Planet has starting buildings', async () => {
    const xBuffer = Buffer.alloc(2) // Allocate 2 bytes for u16
    const yBuffer = Buffer.alloc(2) // Allocate 2 bytes for u16
    xBuffer.writeUInt16LE(1, 0) // Little-endian format
    yBuffer.writeUInt16LE(3, 0) // Little-endian format
    const [planetHoldingPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('planet_holding'),
        playerWallet.publicKey.toBuffer(),
        xBuffer,
        yBuffer,
      ],
      program.programId,
    )
    const accountInfo =
      await program.account.planetHolding.fetch(planetHoldingPDA)

    let hasBuildings = false
    let format = '  Buildings: '
    for (const b of accountInfo.buildings) {
      const building = Object.keys(b.buildingType)[0]
      if (building === 'planetaryCapital') {
        hasBuildings = true
      }
      if (building !== 'none') {
        format += `| ${building} lvl. ${b.level} `
      }
    }
    lineBreak()
    console.log(format)
    lineBreak()
    return hasBuildings
      ? assert.ok('It does')
      : assert.fail('No buildings were created')
  })

  it("Can't claim already claimed planet", async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
        program.programId,
      )
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: secondPlayerWallet.publicKey,
          playerInfo,
        })
        .signers([secondPlayerWallet])
        .rpc()
      return assert.fail('Could claim an already claimed planet')
    } catch {
      return assert.ok("Can't claim an already claimed planet")
    }
  })

  it("Can't claim a planet where there is no planet", async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
        program.programId,
      )
      await program.methods
        .planetFirstClaim(1, 4)
        .accounts({
          signer: secondPlayerWallet.publicKey,
          playerInfo,
        })
        .signers([secondPlayerWallet])
        .rpc()
      return assert.fail('Could settle first planet at invalid position')
    } catch (e) {
      return assert.ok("Can't settle first planet where there is none")
    }
  })

  it("Can't claim another planet as first planet", async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), playerWallet.publicKey.toBuffer()],
        program.programId,
      )
      await program.methods
        .planetFirstClaim(1, 3)
        .accounts({
          signer: playerWallet.publicKey,
          playerInfo,
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('Could settle second planet as first planet.')
    } catch {
      assert.ok("Couldn't settle second planet as first planet.")
    }
  })
})
