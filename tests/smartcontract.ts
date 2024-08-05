import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartcontract } from "../target/types/smartcontract";
import { LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";

describe("smartcontract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const userPubKey = provider.wallet.publicKey;
  const surveyPubKey = 
  const systemProgram = SystemProgram.programId;
  const program = anchor.workspace.Smartcontract as Program<Smartcontract>;

  it("can create survey!", async () => {
    const surveyTitle : string = "Solana vs Ethereum: Blockchain Comparison Survey";
    const surveyDescription : string =
      "We are conducting a survey to compare the usability of Solana and Ethereum. Share your experiences and preferences regarding transaction speed, ease of use, and overall satisfaction with each platform. Your feedback will contribute to a comprehensive comparison of these leading blockchains.";
    const now = new Date().getTime();
    const openTimestamp = new anchor.BN(now);
    const closeTimestamp = new anchor.BN(now + 2 * 24 * 60 * 60 * 1000);
    const targetParticipant = new anchor.BN(10);
    const totalReward = new anchor.BN(0.5 * LAMPORTS_PER_SOL);
    const questionList : string[] = [
      "What do you like most about using Solana ?",
      "What are the main challenges you’ve faced when using Ethereum ?",
      "Describe a feature or aspect of Solana that you believe could be improved.",
      "In your opinion, how does the developer experience on Solana compare to Ethereum? Please provide specific examples.",
      "What factors influenced your preference between Solana and Ethereum ?"
    ];

    const transaction = await program.methods
    .createSurvey(
      surveyTitle,
      surveyDescription,
      openTimestamp,
      closeTimestamp,
      targetParticipant,
      totalReward,
      questionList
    )
    .accounts({
      survey: ,
      user: userPubKey,
      systemProgram: systemProgram
    });

    console.log(`Transaction signature : ${transaction}`);
  });

  it("can fill survey!", async () => {});

  it("can change survey status!", async () => {});

  it("can't fill survey when survey is closed!", async () => {});

  it("can't create survey because there are still empty input!", async () => {});

  it("can't fill survey because there are still empty field!", async () => {});

  it("can't create survey because survey has invalid time!", async () => {});

  it("can't fill this survey because this survey has insufficient funds!", async () => {});
});
