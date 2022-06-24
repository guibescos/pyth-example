import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PythExample } from "../target/types/pyth_example";
import { PublicKey} from "@solana/web3.js";

describe("pyth-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PythExample as Program<PythExample>;

  it("Get BTC price", async () => {
    // Add your test here.
    const tx = await program.methods.getPrice().accounts({price: new PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J")}).rpc();
    console.log("Your transaction signature", tx);
  });
});
