import * as anchor from '@coral-xyz/anchor'
import { Keypair, PublicKey } from '@solana/web3.js'
import { SpaceCastle } from '../target/types/space_castle'
import { clearPlayers, usePlayer } from './utils/player'
import { assert } from 'chai'

describe('[Unit]: 💰 Mints', () => {
  const provider = anchor.AnchorProvider.env()
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>
  anchor.setProvider(provider)

  // Metadata program id
  const METADATA_PROGRAM_ID = new PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
  )

  // IGT
  const [mintIGT] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_igt')],
    program.programId,
  )
  const [metadataIGTAccountAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      METADATA_PROGRAM_ID.toBuffer(),
      mintIGT.toBuffer(),
    ],
    METADATA_PROGRAM_ID,
  )
  // METAL
  const [mintMetal] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_metal')],
    program.programId,
  )
  const [metadataMetalAccountAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      METADATA_PROGRAM_ID.toBuffer(),
      mintMetal.toBuffer(),
    ],
    METADATA_PROGRAM_ID,
  )
  // CRYSTAL
  const [mintCrystal] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_crystal')],
    program.programId,
  )
  const [metadataCrystalAccountAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      METADATA_PROGRAM_ID.toBuffer(),
      mintCrystal.toBuffer(),
    ],
    METADATA_PROGRAM_ID,
  )
  // CHEMICAL
  const [mintChemical] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_chemical')],
    program.programId,
  )
  const [metadataChemicalAccountAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      METADATA_PROGRAM_ID.toBuffer(),
      mintChemical.toBuffer(),
    ],
    METADATA_PROGRAM_ID,
  )
  // FUEL
  const [mintFuel] = PublicKey.findProgramAddressSync(
    [Buffer.from('mint_fuel')],
    program.programId,
  )
  const [metadataFuelAccountAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      METADATA_PROGRAM_ID.toBuffer(),
      mintFuel.toBuffer(),
    ],
    METADATA_PROGRAM_ID,
  )

  let playerWallet: Keypair
  before('Get player wallet', async () => {
    clearPlayers()
    playerWallet = await usePlayer(1)
  })

  it('Intergalactic Tender: mint and metadata', async () => {
    await program.methods
      .mintInitIgt()
      .accounts({
        metadata: metadataIGTAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('Metal: mint and metadata', async () => {
    await program.methods
      .mintInitMetal()
      .accounts({
        metadata: metadataMetalAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('Crystal: mint and metadata', async () => {
    await program.methods
      .mintInitCrystal()
      .accounts({
        metadata: metadataCrystalAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('Chemical: mint and metadata', async () => {
    await program.methods
      .mintInitChemical()
      .accounts({
        metadata: metadataChemicalAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
  it('Fuel: mint and metadata', async () => {
    await program.methods
      .mintInitFuel()
      .accounts({
        metadata: metadataFuelAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
      .catch((e) => {
        return assert.fail(e)
      })
  })
})
