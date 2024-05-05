import * as anchor from '@coral-xyz/anchor'
import { Keypair, PublicKey } from '@solana/web3.js'
import { SpaceCastle } from '../target/types/space_castle'
import { clearPlayers, usePlayer } from './utils/player'

describe('[Unit]: Mints', () => {
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

  let playerWallet: Keypair
  before('Get player wallet', async () => {
    clearPlayers()
    playerWallet = await usePlayer(1)
  })

  it('Creates IGT Mint and metadata', async () => {
    await program.methods
      .mintInitIgt()
      .accounts({
        metadata: metadataIGTAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
  })
  it('Creates Metal Mint and metadata + Metal authority', async () => {
    await program.methods
      .mintInitMetal()
      .accounts({
        metadata: metadataMetalAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
  })
  it('Creates Chemical Mint and metadata + Chemical authority', async () => {
    await program.methods
      .mintInitChemical()
      .accounts({
        metadata: metadataChemicalAccountAddress,
        payer: playerWallet.publicKey,
      })
      .signers([playerWallet])
      .rpc()
  })
})
