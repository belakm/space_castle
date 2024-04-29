import * as anchor from '@coral-xyz/anchor'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'
import { PublicKey } from '@solana/web3.js'
import { SpaceCastle } from '../target/types/space_castle'

describe('[Unit]: Mints', () => {
  const provider = anchor.AnchorProvider.env()
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>
  const payer = (provider.wallet as anchor.Wallet).payer
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

  // Associated token account address
  const associatedTokenAccount = getAssociatedTokenAddressSync(
    mintIGT,
    payer.publicKey,
  )

  it('Creates IGT Mint and metadata', async () => {
    await program.methods
      .mintInitIgt()
      .accounts({
        metadata: metadataIGTAccountAddress,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
  })
  it('Creates Metal Mint and metadata + Metal authority', async () => {
    await program.methods
      .mintInitMetal()
      .accounts({
        metadata: metadataMetalAccountAddress,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
  })
  it('Creates Chemical Mint and metadata + Chemical authority', async () => {
    await program.methods
      .mintInitChemical()
      .accounts({
        metadata: metadataChemicalAccountAddress,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
  })
  it('Mint some IGT to a player', async () => {
    await program.methods
      .mintIgt(new anchor.BN(1000))
      .accounts({
        tokenAccount: associatedTokenAccount,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
    const balance = await provider.connection.getTokenAccountBalance(
      associatedTokenAccount,
    )
    console.log(`\n\tPlayer balance: ${balance.value.uiAmount} IGT`)
  })
  it('Mint some metal to a player', async () => {
    const [signer_pda] = PublicKey.findProgramAddressSync(
      [Buffer.from('account_metal'), payer.publicKey.toBuffer()],
      program.programId,
    )
    // const ata = getAssociatedTokenAddressSync(mintMetal, signer_pda, true)
    await program.methods
      .mintMetal(new anchor.BN(1000))
      .accounts({
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
    const balance = await provider.connection.getTokenAccountBalance(signer_pda)
    console.log(`\n\tPlayer balance: ${balance.value.uiAmount} rMET`)
  })
})
