import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { SolanaMicroGrants } from "../target/types/Solana_Micro_Grants"; // Replace with your actual program type

describe("DAO Creation Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaMicroGrants as Program<SolanaMicroGrants>;
  const authority = provider.wallet as anchor.Wallet;

  // Test data
  const daoName = "TestDAO";
  const governanceTokenMint = new PublicKey("G9EoH38xXqvXjb4qqnKdmY9m7tSWiL7mb9PwcmEZKsnb");

  // PDAs
  let daoPda: PublicKey;
  let treasuryPda: PublicKey;
  let daoBump: number;
  let treasuryBump: number;

  before(async () => {
    // Derive PDAs
    [daoPda, daoBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("dao"), Buffer.from(daoName)],
      program.programId
    );

    [treasuryPda, treasuryBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), Buffer.from(daoName)],
      program.programId
    );
  });

  describe("Successful DAO Creation", () => {
    it("Should create a DAO with valid parameters", async () => {
      const tx = await program.methods
        .createDao(governanceTokenMint, daoName)
        .accounts({
          dao: daoPda,
          treasury: treasuryPda,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Transaction signature:", tx);

      const daoAccount = await program.account.daoAccount.fetch(daoPda);
      const treasuryAccount = await program.account.treasury.fetch(treasuryPda);

      expect(daoAccount.authority.toString()).to.equal(authority.publicKey.toString());
      expect(daoAccount.treasury.toString()).to.equal(treasuryPda.toString());
      expect(daoAccount.proposalCount.toNumber()).to.equal(0);
      expect(daoAccount.bump).to.equal(daoBump);

      const nameString = Buffer.from(daoAccount.daoName).toString().replace(/\0+$/, '');
      expect(nameString).to.equal(daoName);

      expect(treasuryAccount.treasuryMint.toString()).to.equal(governanceTokenMint.toString());
      expect(treasuryAccount.balance.toNumber()).to.equal(0);
      expect(treasuryAccount.bump).to.equal(treasuryBump);
    });

    it("Should create DAO with maximum length name", async () => {
      const longDaoName = "A".repeat(32); 
      
      const [longDaoPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("dao"), Buffer.from(longDaoName)],
        program.programId
      );

      const [longTreasuryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("treasury"), Buffer.from(longDaoName)],
        program.programId
      );

      await program.methods
        .createDao(governanceTokenMint, longDaoName)
        .accounts({
          dao: longDaoPda,
          treasury: longTreasuryPda,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const daoAccount = await program.account.daoAccount.fetch(longDaoPda);
      const nameString = Buffer.from(daoAccount.daoName).toString().replace(/\0+$/, '');
      expect(nameString).to.equal(longDaoName);
    });

    it("Should create DAO with different governance token mint", async () => {
      const differentMint = Keypair.generate().publicKey;
      const daoName2 = "TestDAO2";
      
      const [dao2Pda] = PublicKey.findProgramAddressSync(
        [Buffer.from("dao"), Buffer.from(daoName2)],
        program.programId
      );

      const [treasury2Pda] = PublicKey.findProgramAddressSync(
        [Buffer.from("treasury"), Buffer.from(daoName2)],
        program.programId
      );

      await program.methods
        .createDao(differentMint, daoName2)
        .accounts({
          dao: dao2Pda,
          treasury: treasury2Pda,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const treasuryAccount = await program.account.treasury.fetch(treasury2Pda);
      expect(treasuryAccount.treasuryMint.toString()).to.equal(differentMint.toString());
    });
  });

  // describe("Error Cases", () => {
  //   it("Should fail with DAO name too long", async () => {
  //     const tooLongName = "A".repeat(33); // Over 32 chars
      
  //     const [longDaoPda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("dao"), Buffer.from(tooLongName)],
  //       program.programId
  //     );

  //     const [longTreasuryPda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("treasury"), Buffer.from(tooLongName)],
  //       program.programId
  //     );

  //     try {
  //       await program.methods
  //         .createDao(governanceTokenMint, tooLongName)
  //         .accounts({
  //           dao: longDaoPda,
  //           treasury: longTreasuryPda,
  //           authority: authority.publicKey,
  //           systemProgram: SystemProgram.programId,
  //         })
  //         .rpc();
        
  //       expect.fail("Should have thrown an error");
  //     } catch (error) {
  //       expect(error.message).to.include("DAO name is too long");
  //     }
  //   });

  //   it("Should fail when trying to create duplicate DAO", async () => {
  //     // Try to create the same DAO again
  //     try {
  //       await program.methods
  //         .createDao(governanceTokenMint, daoName)
  //         .accounts({
  //           dao: daoPda,
  //           treasury: treasuryPda,
  //           authority: authority.publicKey,
  //           systemProgram: SystemProgram.programId,
  //         })
  //         .rpc();
        
  //       expect.fail("Should have thrown an error");
  //     } catch (error) {
  //       // Should fail because account already exists
  //       expect(error.message).to.include("already in use");
  //     }
  //   });

  //   it("Should fail with wrong PDA derivation", async () => {
  //     const wrongDaoPda = Keypair.generate().publicKey;
  //     const daoName3 = "TestDAO3";
      
  //     const [correctTreasuryPda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("treasury"), Buffer.from(daoName3)],
  //       program.programId
  //     );

  //     try {
  //       await program.methods
  //         .createDao(governanceTokenMint, daoName3)
  //         .accounts({
  //           dao: wrongDaoPda, // Wrong PDA
  //           treasury: correctTreasuryPda,
  //           authority: authority.publicKey,
  //           systemProgram: SystemProgram.programId,
  //         })
  //         .rpc();
        
  //       expect.fail("Should have thrown an error");
  //     } catch (error) {
  //       expect(error.message).to.include("Invalid seeds");
  //     }
  //   });
  // });

  // describe("Account Space Verification", () => {
  //   it("Should verify DAO account has correct size", async () => {
  //     const accountInfo = await provider.connection.getAccountInfo(daoPda);
  //     expect(accountInfo).to.not.be.null;
      
  //     // Your DaoAccount::SPACE should be 145 bytes based on optimization
  //     const expectedSize = 8 + 32 + 32 + 32 + 8 + 1 + 32; // 145 bytes
  //     expect(accountInfo!.data.length).to.equal(expectedSize);
  //   });

  //   it("Should verify Treasury account has correct size", async () => {
  //     const accountInfo = await provider.connection.getAccountInfo(treasuryPda);
  //     expect(accountInfo).to.not.be.null;
      
  //     // Treasury::SPACE should be 49 bytes
  //     const expectedSize = 8 + 32 + 8 + 1; // 49 bytes
  //     expect(accountInfo!.data.length).to.equal(expectedSize);
  //   });
  // });

  // describe("PDA Verification", () => {
  //   it("Should verify correct PDA derivation", async () => {
  //     const [derivedDaoPda, derivedBump] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("dao"), Buffer.from(daoName)],
  //       program.programId
  //     );

  //     expect(derivedDaoPda.toString()).to.equal(daoPda.toString());
  //     expect(derivedBump).to.equal(daoBump);
  //   });

  //   it("Should verify treasury PDA relationship", async () => {
  //     const daoAccount = await program.account.daoAccount.fetch(daoPda);
  //     expect(daoAccount.treasury.toString()).to.equal(treasuryPda.toString());
  //   });
  // });

  // describe("Multiple Authority Test", () => {
  //   it("Should allow different authorities to create DAOs", async () => {
  //     const newAuthority = Keypair.generate();
      
  //     // Airdrop SOL to new authority
  //     const signature = await provider.connection.requestAirdrop(
  //       newAuthority.publicKey,
  //       2 * anchor.web3.LAMPORTS_PER_SOL
  //     );
  //     await provider.connection.confirmTransaction(signature);

  //     const daoName4 = "TestDAO4";
  //     const [dao4Pda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("dao"), Buffer.from(daoName4)],
  //       program.programId
  //     );

  //     const [treasury4Pda] = PublicKey.findProgramAddressSync(
  //       [Buffer.from("treasury"), Buffer.from(daoName4)],
  //       program.programId
  //     );

  //     await program.methods
  //       .createDao(governanceTokenMint, daoName4)
  //       .accounts({
  //         dao: dao4Pda,
  //         treasury: treasury4Pda,
  //         authority: newAuthority.publicKey,
  //         systemProgram: SystemProgram.programId,
  //       })
  //       .signers([newAuthority])
  //       .rpc();

  //     const daoAccount = await program.account.daoAccount.fetch(dao4Pda);
  //     expect(daoAccount.authority.toString()).to.equal(newAuthority.publicKey.toString());
  //   });
  // });
});
