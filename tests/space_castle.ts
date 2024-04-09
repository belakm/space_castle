import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SpaceCastle } from "../target/types/space_castle";

describe("space_castle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SpaceCastle as Program<SpaceCastle>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initializeGame().rpc();
    console.log("Your transaction signature", tx);
  });
});
