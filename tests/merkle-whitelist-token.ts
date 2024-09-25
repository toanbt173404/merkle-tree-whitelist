import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MerkleWhitelistToken } from "../target/types/merkle_whitelist_token";

describe("merkle-whitelist-token", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MerkleWhitelistToken as Program<MerkleWhitelistToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
