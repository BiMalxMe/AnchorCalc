import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";
import { assert, expect } from "chai";

describe("anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const newAccount =  anchor.web3.Keypair.generate(); 
  const program = anchor.workspace.anchor as Program<Anchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .init(2)
    .accounts({
      signer : anchor.getProvider().wallet.publicKey,
      account : newAccount.publicKey
    })
    .signers([newAccount])
    .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.accountDataShape.fetch(newAccount.publicKey)
    assert(account.num ==2)
  });
  it("Is Doubled!", async () => {
    // Add your test here.
    const tx = await program.methods
    .double()
    .accounts({
      signer : anchor.getProvider().wallet.publicKey,
      account : newAccount.publicKey
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.accountDataShape.fetch(newAccount.publicKey)
    assert(account.num == 4)
  });
  it("is added!", async () => {
    // Add your test here.
    const tx = await program.methods
    .add(20)
    .accounts({
      signer : anchor.getProvider().wallet.publicKey,
      account : newAccount.publicKey
    })
    .signers([])
    .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.accountDataShape.fetch(newAccount.publicKey)
    assert(account.num ==24)
  });
});
