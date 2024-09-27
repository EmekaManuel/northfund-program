import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NorthfundProgram } from "../target/types/northfund_program";
import { expect } from "chai";
import { BN } from "bn.js"; // Import BN for handling big numbers

describe("northfund-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .NorthfundProgram as Program<NorthfundProgram>;

  it("should create a student fund campaign with the provided details", async () => {
    // Generate a user keypair
    const user = anchor.web3.Keypair.generate();

    // Generate a keypair for the student fund campaign
    const studentFundCampaign = anchor.web3.Keypair.generate();

    // Create a campaign list keypair (you might need to initialize this in your program)
    const campaignList = anchor.web3.Keypair.generate();

    // Call create function with mock context and arguments using the new method
    await program.methods
      .createStudentFundCampaign(
        "Student Name",
        "2024/123456",
        "Computer Science",
        2023,
        new BN(1000),
        "Funding for tuition",
        "https://example.com/student_image.jpg"
      )
      .accounts({
        //@ts-ignore
        studentFundCampaign: studentFundCampaign.publicKey, // Use correct casing
        user: user.publicKey,
        campaignList: campaignList.publicKey, // Generate or retrieve the campaign list's public key
      })
      .signers([studentFundCampaign, user]) // Signers are the accounts being used
      .rpc(); // Call the rpc method

    // Fetch the account details after creation to verify
    const campaignAccount = await program.account.studentFundCampaign.fetch(
      studentFundCampaign.publicKey
    );

    // Assertions
    expect(campaignAccount.studentName).to.equal("Student Name");
    expect(campaignAccount.matricNumber).to.equal("2024/123456");
    expect(campaignAccount.courseOfStudy).to.equal("Computer Science");
    expect(campaignAccount.yearOfEntry).to.equal(2023);
    expect(campaignAccount.targetAmount.toNumber()).to.equal(1000);
    expect(campaignAccount.fundingReason).to.equal("Funding for tuition");
    expect(campaignAccount.studentImageUrl).to.equal(
      "https://example.com/student_image.jpg"
    );
    expect(campaignAccount.amountDonated.toNumber()).to.equal(0);
    expect(campaignAccount.amountWithdrawn.toNumber()).to.equal(0);
    expect(campaignAccount.admin.toBase58()).to.equal(
      user.publicKey.toBase58()
    );
  });
});
