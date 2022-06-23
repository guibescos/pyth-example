import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PythExample } from "../target/types/pyth_example";

describe("pyth-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PythExample as Program<PythExample>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
