import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PythExample } from "../target/types/pyth_example";
import { PublicKey} from "@solana/web3.js";
import { BN } from "bn.js";

describe("pyth-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PythExample as Program<PythExample>;

  it("Send money", async () => {
    // Add your test here.
    const tx = await program.methods.payUsd(new BN(1)).accounts({solUsdPriceAccount: new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"), to : new PublicKey("3zDcwD1YQjr4YpLNzGRZA4oJL8HsCv34Rpze2qgkFP78")}).rpc();
    console.log("Your transaction signature", tx);
  });
});
