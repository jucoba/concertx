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
  
  const concert = {
    title: "The big concert",
    desc: "The most amazing concert",
    goalAmount: 1000,
    ticketPrice: 100,
    startDate: new BN(new Date(2024,12,31,10,0,0).getTime()),
    endDate: new BN(new Date(2024,12,31,10,0,0).getTime()).add(new BN(1000)),
  };

  const backerA = anchor.web3.Keypair.generate();

  it("Create concert", async () => {
    const tx = await program.methods.createConcert(concert.title, 
                                                   concert.desc, 
                                                   new BN(concert.goalAmount),
                                                   new BN(concert.ticketPrice),
                                                   concert.startDate, 
                                                   concert.endDate).rpc();
    console.log("Your transaction signature", tx);

    const [concertXPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("concertX"), Buffer.from(concert.title), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    
    const account = await program.account.concert.fetch(concertXPda);
    expect(account.title).equals(concert.title);
    expect(account.shortDescription).equals(concert.desc);
    expect(account.startDate.eq(concert.startDate)).to.be.true;
    expect(account.endDate.eq(concert.endDate)).to.be.true;
    expect(account.ticketPrice.toNumber()).equals(concert.ticketPrice);
  });

  it("Make a contribution", async () => {

    // Get PDA of the concert campaign 
    const [concertXPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("concertX"), Buffer.from(concert.title), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const contributionAmount = 500;

    await program.methods
      .makeContribution(new anchor.BN(contributionAmount))
      .accounts({
        concert: concertXPda,
        backer: backerA.publicKey,
      })
      .signers([backerA])
      .rpc();

      const updatedConcertAccount = await program.account.concert.fetch(concertXPda);
      expect(updatedConcertAccount.currentAmount.toNumber()).to.equal(contributionAmount);
    
    
  });




});
