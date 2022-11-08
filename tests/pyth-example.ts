import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PythV1 } from "../target/types/pyth_v1";
import { PythV2 } from "../target/types/pyth_v2";

describe("pyth example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PythV1 as Program<PythV1>;
  const programV2 = anchor.workspace.PythV2 as Program<PythV2>;

  it("Is printing price of sol in usd in v1", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        solPriceKey: new anchor.web3.PublicKey(
          "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"
        ),
        solProduct: new anchor.web3.PublicKey(
          "3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E"
        ),
      })
      .rpc();
    console.log("Your transaction signature in v1", tx);
  });
  it("Is printing price of sol in usd in v2", async () => {
    const tx = await programV2.methods
      .initialize()
      .accounts({
        solPriceKey: new anchor.web3.PublicKey(
          "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"
        ),
      })
      .rpc({
        skipPreflight: true,
      });
    console.log("Your transaction signature in v2", tx);
  });
});
