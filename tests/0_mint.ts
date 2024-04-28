import * as anchor from '@coral-xyz/anchor'
import { getAssociatedTokenAddressSync } from '@solana/spl-token'
import { PublicKey } from '@solana/web3.js'
import { SpaceCastle } from '../target/types/space_castle'
import { assert } from 'chai'

describe('[Unit]: Mints', () => {
  const provider = anchor.AnchorProvider.env()
  const program = anchor.workspace.SpaceCastle as anchor.Program<SpaceCastle>
  const payer = (provider.wallet as anchor.Wallet).payer
  anchor.setProvider(provider)

  // Metadata program id
  const METADATA_PROGRAM_ID = new PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
  )

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

  // Associated token account address
  const associatedTokenAccount = getAssociatedTokenAddressSync(
    mintIGT,
    payer.publicKey,
  )

  it('Creates IGT Mint and metadata', async () => {
    const txSig = await program.methods
      .mintInitIgt()
      .accounts({
        metadata: metadataIGTAccountAddress,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
    console.log(`Transaction Signature: ${txSig}`)
  })
  it('Creates Metal Mint and metadata + Metal authority', async () => {
    const txSig = await program.methods
      .mintInitMetal()
      .accounts({
        metadata: metadataMetalAccountAddress,
        payer: payer.publicKey,
      })
      .signers([payer])
      .rpc()
    console.log(`Transaction Signature: ${txSig}`)
  })
  it('Mint some IGT to a player', async () => {
    try {
      const amount = new anchor.BN(1000)
      const txSig = await program.methods
        .mintIgt(amount)
        .accounts({
          tokenAccount: associatedTokenAccount,
          payer: payer.publicKey,
        })
        .signers([payer])
        .rpc()

      console.log(`Transaction Signature: ${txSig}`)

      const balance = await provider.connection.getTokenAccountBalance(
        associatedTokenAccount,
      )
      console.log(`Balance: ${balance.value.uiAmount}`)
    } catch (e) {
      console.error(e)
      assert.fail('Failed')
    }
  })
})
