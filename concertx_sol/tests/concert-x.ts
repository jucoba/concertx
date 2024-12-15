import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConcertX } from "../target/types/concert_x";
import { expect } from "chai";
import BN from "bn.js";

describe("concert-x", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ConcertX as Program<ConcertX>;
  const [concertXPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("concertX"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const title = "The big concert";
    
    const desc = "The most amazing concert";
    const goalAmount = 1000;
    const startDate = new BN(new Date(2024,12,31,10,0,0).getTime());
    const endDate = startDate.add(new BN(1000));
    const tx = await program.methods.createConcert(title, desc,goalAmount,startDate,endDate).rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.concert.fetch(concertXPda);
    expect(account.title).equals(title);
    expect(account.description).equals(desc);
    expect(account.startDate.eq(startDate)).to.be.true;
    expect(account.endDate.eq(endDate)).to.be.true;
  });
});
