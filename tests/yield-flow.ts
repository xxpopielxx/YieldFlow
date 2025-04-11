import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { YieldFlow } from "../target/types/yield_flow";

describe("yield-flow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.yieldFlow as Program<YieldFlow>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
