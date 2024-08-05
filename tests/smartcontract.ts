import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartcontract } from "../target/types/smartcontract";
import { LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import * as assert from "assert";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

describe("smartcontract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const systemProgram = SystemProgram.programId;
  const program = anchor.workspace.Smartcontract as Program<Smartcontract>;
  const user = provider.wallet;
  let reusableSurveyPda;

  const surveyTitle: string =
      "Solana vs Ethereum: Blockchain Comparison Survey";
    const surveyDescription: string =
      "We are conducting a survey to compare the usability of Solana and Ethereum. Share your experiences and preferences regarding transaction speed, ease of use, and overall satisfaction with each platform. Your feedback will contribute to a comprehensive comparison of these leading blockchains.";
    const now = new Date().getTime();
    const openTimestamp = new anchor.BN(now);
    const closeTimestamp = new anchor.BN(now + (2 * 24 * 60 * 60 * 1000));
    const targetParticipant = new anchor.BN(10);
    const totalReward = new anchor.BN(0.5 * LAMPORTS_PER_SOL);
    const questionList: string[] = [
      "What do you like most about using Solana ?",
      "What are the main challenges you’ve faced when using Ethereum ?",
      "Describe a feature or aspect of Solana that you believe could be improved.",
      "In your opinion, how does the developer experience on Solana compare to Ethereum? Please provide specific examples.",
      "What factors influenced your preference between Solana and Ethereum ?",
    ];

    const answerList: string[] = [
      "What I like most about using Solana is its high transaction throughput and low fees. The speed of transactions on Solana is impressive compared to other blockchains, which enhances the user experience significantly.",
      "The main challenges I've faced with Ethereum include high gas fees during peak times and slower transaction confirmation times compared to other blockchains. These issues can be frustrating when performing frequent or low-value transactions.",
      "One aspect of Solana that I believe could be improved is its developer tooling and documentation. While the network is fast and efficient, having more comprehensive and user-friendly tools would greatly benefit developers building on Solana.",
      "In my opinion, the developer experience on Solana is generally more streamlined due to its lower transaction fees and faster speeds. However, Ethereum's extensive documentation and established ecosystem offer a lot of support and resources, which can be very helpful for developers.",
      "Factors that influenced my preference between Solana and Ethereum include transaction speed and cost. Solana's high speed and low transaction fees make it more attractive for applications that require frequent interactions, while Ethereum's established ecosystem and extensive developer resources make it a strong choice for projects that need mature tools and community support."
    ];

  it("can create survey!", async () => {
    const id = closeTimestamp.sub(openTimestamp);
    const idToBuffer = id.toBuffer("le", 8);
    const [surveyPda, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("survey"), 
        user.publicKey.toBuffer(),
        idToBuffer
      ],
      program.programId
    );

    await program.methods
      .createSurvey(
        id,
        surveyTitle,
        surveyDescription,
        openTimestamp,
        closeTimestamp,
        targetParticipant,
        totalReward,
        questionList
      )
      .accounts({
        survey: surveyPda,
        user: user.publicKey,
        systemProgram: systemProgram,
      })
      .rpc();

    const account = await program.account.survey.fetch(surveyPda);
    assert.strictEqual(account.title, surveyTitle);
    assert.strictEqual(account.description, surveyDescription);
    assert.strictEqual(
      account.openTimestamp.toString(),
      openTimestamp.toString()
    );
    assert.strictEqual(
      account.closeTimestamp.toString(),
      closeTimestamp.toString()
    );
    assert.strictEqual(
      account.targetParticipant.toString(),
      targetParticipant.toString()
    );
    assert.strictEqual(account.totalReward.toString(), totalReward.toString());
    assert.deepStrictEqual(account.questionList, questionList);
    reusableSurveyPda = surveyPda;
  });

  it("can fill survey!", async () => {
    const [answerPda, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("answer"),
        user.publicKey.toBuffer(),
        reusableSurveyPda.toBuffer()
      ],
      program.programId
    )

    console.log(reusableSurveyPda);

    await program.methods
    .fillSurvey(
      answerList
    )
    .accounts({
      user: user.publicKey,
      answer: answerPda,
      survey: reusableSurveyPda,
      systemProgram: SystemProgram.programId
    })
    .rpc()

    const account = await program.account.answer.fetch(answerPda);
    assert.deepStrictEqual(account.answerList, answerList);
  });

  it("can change survey status!", async () => {});

  it("can't fill survey when survey is closed!", async () => {});

  it("can't create survey because there are still empty input!", async () => {});

  it("can't fill survey because there are still empty field!", async () => {});

  it("can't create survey because survey has invalid time!", async () => {});

  it("can't fill this survey because this survey has insufficient funds!", async () => {});
});
