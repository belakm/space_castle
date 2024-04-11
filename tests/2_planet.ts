import * as anchor from '@coral-xyz/anchor'
import { type Program } from '@coral-xyz/anchor'
import { type SpaceCastle } from '../target/types/space_castle'
import { Keypair, PublicKey } from '@solana/web3.js'
import { assert } from 'chai'
import { createAndFundWallet, number_to_bytes_buffer } from './utils'

describe('Space Castle: PLANET', () => {

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>
  const valid_planet = {
    x: number_to_bytes_buffer(1),
    y: number_to_bytes_buffer(3)
  }
  const second_valid_planet = {
    x: number_to_bytes_buffer(2),
    y: number_to_bytes_buffer(6)
  }
  const invalid_planet = {
    x: number_to_bytes_buffer(1),
    y: number_to_bytes_buffer(4)
  }
  let playerWallet: Keypair
  let secondPlayerWallet: Keypair

  before('Prepare wallet and player accounts', async () => {
    playerWallet = await createAndFundWallet()
    secondPlayerWallet = await createAndFundWallet()

    const [playerAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('player'), playerWallet.publicKey.toBuffer()],
      program.programId
    )
    const [secondPlayerAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
      program.programId
    )

    await program.methods
      .registerPlayer('mico')
      .accounts({
        signer: playerWallet.publicKey,
        player: playerAccount
      })
      .signers([playerWallet])
      .rpc()

    await program.methods
      .registerPlayer('mico 2')
      .accounts({
        signer: secondPlayerWallet.publicKey,
        player: secondPlayerAccount
      })
      .signers([secondPlayerWallet])
      .rpc()

  })

  it('Player with no planets can claim the first planet', async () => {
    const [playerInfo] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('player'),
        playerWallet.publicKey.toBuffer()
      ],
      program.programId
    )
    const [planetInfo] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('planet_info'),
        valid_planet.x,
        valid_planet.y
      ],
      program.programId
    )
    const [planetHolding] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('planet_holding'),
        playerWallet.publicKey.toBuffer(),
        valid_planet.x,
        valid_planet.y
      ],
      program.programId
    )
    const [initialShip] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('ship'),
        playerWallet.publicKey.toBuffer(),
        Buffer.from('1')
      ],
      program.programId
    )

    await program.methods
      .settleFirstPlanet(1, 3)
      .accounts({
        signer: playerWallet.publicKey,
        planetInfo,
        planetHolding,
        initialShip,
        playerInfo
      })
      .signers([playerWallet])
      .rpc()
  })

  it('Can\'t claim already claimed planet', async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [Buffer.from('player'), secondPlayerWallet.publicKey.toBuffer()],
        program.programId
      )
      const [planetInfo] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('planet_info'),
          valid_planet.x,
          valid_planet.y
        ],
        program.programId
      )
      const [planetHolding] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('planet_holding'),
          secondPlayerWallet.publicKey.toBuffer(),
          valid_planet.x,
          valid_planet.y
        ],
        program.programId
      )
      const [initialShip] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('ship'),
          secondPlayerWallet.publicKey.toBuffer(),
          Buffer.from('1')
        ],
        program.programId
      )

      await program.methods
        .settleFirstPlanet(1, 3)
        .accounts({
          signer: secondPlayerWallet.publicKey,
          planetInfo,
          planetHolding,
          initialShip,
          playerInfo
        })
        .signers([secondPlayerWallet])
        .rpc()
      assert.fail('Could claim an already claimed planet')
    } catch {
      assert.ok('Can\'t claim an already claimed planet')
    }
  })

  it('Can\'t claim a planet where there is no planet', async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('player'),
          secondPlayerWallet.publicKey.toBuffer()
        ],
        program.programId
      )
      const [planetInfo] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('planet_info'),
          invalid_planet.x,
          invalid_planet.y
        ],
        program.programId
      )
      const [planetHolding] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('planet_holding'),
          secondPlayerWallet.publicKey.toBuffer(),
          invalid_planet.x,
          invalid_planet.y
        ],
        program.programId
      )
      const [initialShip] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('ship'),
          secondPlayerWallet.publicKey.toBuffer(),
          Buffer.from('1')
        ],
        program.programId
      )

      await program.methods
        .settleFirstPlanet(1, 3)
        .accounts({
          signer: secondPlayerWallet.publicKey,
          planetInfo,
          planetHolding,
          initialShip,
          playerInfo
        })
        .signers([secondPlayerWallet])
        .rpc()
      assert.fail('Could settle first planet at invalid position')
    } catch {
      assert.ok('Can\'t settle first planet where there is none')
    }
  })

  it('Can\'t claim another planet as first planet', async () => {
    try {
      const [playerInfo] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('player'),
          playerWallet.publicKey.toBuffer()
        ],
        program.programId
      )
      const [planetInfo] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('planet_info'),
          second_valid_planet.x,
          second_valid_planet.y
        ],
        program.programId
      )
      const [planetHolding] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('planet_holding'),
          playerWallet.publicKey.toBuffer(),
          second_valid_planet.x,
          second_valid_planet.y
        ],
        program.programId
      )
      const [initialShip] = PublicKey.findProgramAddressSync(
        [
          Buffer.from('ship'),
          playerWallet.publicKey.toBuffer(),
          Buffer.from('1')
        ],
        program.programId
      )

      await program.methods
        .settleFirstPlanet(1, 3)
        .accounts({
          signer: playerWallet.publicKey,
          planetInfo,
          planetHolding,
          initialShip,
          playerInfo
        })
        .signers([playerWallet])
        .rpc()
      assert.fail('Could settle second planet as first planet.')
    } catch {
      assert.ok('Couln\'t settle second planet as first planet.')
    }
  })

})
