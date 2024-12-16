import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConcertX } from "../target/types/concert_x";
import {fetchConcerts} from "../../web/app/utils/crypto";
import { expect } from "chai";
import BN, { max } from "bn.js";

describe("concert-x", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ConcertX as Program<ConcertX>;
  

  it("Create concert", async () => {
    // Add your test here.
    const title = "The big concert";
    const desc = "The most amazing concert";
    const goalAmount = 1000;
    const maxTokenSupply = 1000;
    const startDate = new BN(new Date(2024,12,31,10,0,0).getTime());
    const endDate = startDate.add(new BN(1000));
    const tx = await program.methods.createConcert(title, desc,goalAmount,startDate,endDate, maxTokenSupply).rpc();
    console.log("Your transaction signature", tx);

    const [concertXPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("concertX"), Buffer.from(title), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    
    const account = await program.account.concert.fetch(concertXPda);
    expect(account.title).equals(title);
    expect(account.shortDescription).equals(desc);
    expect(account.startDate.eq(startDate)).to.be.true;
    expect(account.endDate.eq(endDate)).to.be.true;
    expect(account.maxTokenSupply).equals(maxTokenSupply);
    

    const concerts = await fetchConcerts();
    expect(concerts.length).greaterThan(0);

   
    
  });

  /*it("Make an apportation", async () => {
    // Add your test here.
    const title = "The little concert";
    const desc = "The most amazing concert";
    const goalAmount = 1000;
    const startDate = new BN(new Date(2024,12,31,10,0,0).getTime());
    const endDate = startDate.add(new BN(1000));
    const tx = await program.methods.createConcert(title, desc,goalAmount,startDate,endDate).rpc();
    console.log("Your transaction signature", tx);

    const [concertXPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("concertX"), Buffer.from(title), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const backer = anchor.web3.Keypair.generate();
    console.log("Backer publick key", backer.publicKey);
    // Airdrop some SOL to the backer
    const airdropTxBacker = await provider.connection.requestAirdrop(
      backer.publicKey,
      1_000_000_000 // 1 SOL in lamports
    );
    await provider.connection.confirmTransaction({
      signature: airdropTxBacker,
      type: "confirmed", // Commitment level as the type
    });

    const aportationAmount = 500;

    await program.methods
      .makeAportation(new anchor.BN(aportationAmount))
      .accounts({
        concert: concertXPda,
        backer: backer.publicKey,
      })
      .signers([backer])
      .rpc();

      const updatedConcertAccount = await program.account.concert.fetch(concertXPda);
      //expect(updatedConcertAccount.currentAmount.toNumber()).to.equal(aportationAmount);
    
    
  });*/




});
