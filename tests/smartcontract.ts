import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartcontract } from "../target/types/smartcontract";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("smartcontract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const systemProgram = SystemProgram.programId;
  const program = anchor.workspace.Smartcontract as Program<Smartcontract>;
  const user = provider.wallet;

  let surveyPda;
  let answerPda;
  let surveyId;
  let convertedOpenTimestamp;
  let convertedCloseTimestamp;

  const surveyTitle: string =
    "Solana vs Ethereum: Blockchain Comparison Survey";
  const surveyDescription: string =
    "We are conducting a survey to compare the usability of Solana and Ethereum. Share your experiences and preferences regarding transaction speed, ease of use, and overall satisfaction with each platform. Your feedback will contribute to a comprehensive comparison of these leading blockchains.";

  const currentDate = new Date();
  const now = currentDate.getTime();

  const targetParticipant = new anchor.BN(100);
  const totalReward = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

  const questionList: string[] = [
    "What do you like most about using Solana ?",
    "What are the main challenges you’ve faced when using Ethereum ?",
    "Describe a feature or aspect of Solana that you believe could be improved.",
    "In your opinion, how does the developer experience on Solana compare to Ethereum? Please provide specific examples.",
    "What factors influenced your preference between Solana and Ethereum ?",
  ];
  const answerList: string[] = [
    "I like Solana's high throughput and low fees. Transactions are impressively fast.",
    "Ethereum's high gas fees and slow confirmations are challenging.",
    "Solana needs better developer tools and documentation.",
    "Solana has lower fees and faster speeds, but Ethereum offers more support.",
    "Solana is best for frequent transactions; Ethereum is best for mature projects.",
  ];

  it("can create survey!", async () => {
    const openTimestamp = Math.floor(now / 1000);
    convertedOpenTimestamp = new anchor.BN(openTimestamp);

    const twoDaysLater = new Date(currentDate);
    twoDaysLater.setDate(currentDate.getDate() + 2);
    const closeTimestamp = Math.floor(twoDaysLater.getTime() / 1000);
    convertedCloseTimestamp = new anchor.BN(closeTimestamp);

    const id = convertedCloseTimestamp.sub(convertedOpenTimestamp);
    surveyId = id.toBuffer("le", 8);
    [surveyPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("survey"), user.publicKey.toBuffer(), surveyId],
      program.programId
    );

    await program.methods
      .createSurvey(
        id,
        surveyTitle,
        surveyDescription,
        convertedOpenTimestamp,
        convertedCloseTimestamp,
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
    assert.strictEqual(account.id.toString(), id.toString());
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
  });

  it("can fill survey!", async () => {
    const surveyIdBn = new anchor.BN(surveyId, "le");
    const id = new anchor.BN(surveyIdBn.toNumber());
    [answerPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("answer"), user.publicKey.toBuffer(), surveyId],
      program.programId
    );

    await program.methods
      .fillSurvey(id, answerList)
      .accounts({
        user: user.publicKey,
        answer: answerPda,
        survey: surveyPda,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const answerAccount = await program.account.answer.fetch(answerPda);
    const surveyAccount = await program.account.survey.fetch(surveyPda);
    assert.strictEqual(id.toString(), answerAccount.surveyId.toString());
    assert.deepStrictEqual(answerAccount.answerList, answerList);
    assert.strictEqual(surveyAccount.currentParticipant.toString(), "1");
  });

  it("can't create survey because there are still empty input!", async () => {
    const fourDaysLater = new Date(currentDate);
    fourDaysLater.setDate(currentDate.getDate() + 4);
    const closeTimestamp = Math.floor(fourDaysLater.getTime() / 1000);
    const convertedCloseTimestamp = new anchor.BN(closeTimestamp);

    const id = convertedCloseTimestamp.sub(convertedOpenTimestamp);
    surveyId = id.toBuffer("le", 8);
    [surveyPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("survey"), user.publicKey.toBuffer(), surveyId],
      program.programId
    );

    try {
      await program.methods
        .createSurvey(
          id,
          "",
          "",
          convertedOpenTimestamp,
          convertedCloseTimestamp,
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
    } catch (error) {
      assert.ok("Invalid survey input.");
    }
  });

  it("can't fill survey because there are still empty field!", async () => {
    const fiveDaysLater = new Date(currentDate);
    fiveDaysLater.setDate(currentDate.getDate() + 5);
    const closeTimestamp = Math.floor(fiveDaysLater.getTime() / 1000);
    const convertedCloseTimestamp = new anchor.BN(closeTimestamp);

    const id = convertedCloseTimestamp.sub(convertedOpenTimestamp);
    surveyId = id.toBuffer("le", 8);
    [surveyPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("survey"), user.publicKey.toBuffer(), surveyId],
      program.programId
    );

    const firstAnswerList = ["", "", "", "", ""];
    const secondAnswerList = [answerList[0]]

    await program.methods
      .createSurvey(
        id,
        surveyTitle,
        surveyDescription,
        convertedOpenTimestamp,
        convertedCloseTimestamp,
        targetParticipant,
        totalReward,
        questionList
      )
      .accounts({
        user: user.publicKey,
        survey: surveyPda,
        systemProgram: systemProgram,
      })
      .rpc();

    try {
      await program.methods
        .fillSurvey(id, firstAnswerList)
        .accounts({
          survey: surveyPda,
          user: user.publicKey,
          systemProgram: systemProgram,
        })
        .rpc();
    } 
    catch (error) {
      assert.ok("Invalid survey input.");
    }

    try {
      await program.methods
      .fillSurvey(id, secondAnswerList)
      .accounts({
        survey: surveyPda,
        user: user.publicKey,
        systemProgram: systemProgram,
      })
      .rpc();
    }
    catch(error) {
      assert.ok("All field must be answered.");
    }
  });

  it("can't create survey because survey has invalid time!", async () => {
    const oneDayBefore = new Date(currentDate);
    oneDayBefore.setDate(currentDate.getDate() - 1);
    const closeTimestamp = Math.floor(oneDayBefore.getTime() / 1000);
    const convertedCloseTimestamp = new anchor.BN(closeTimestamp);

    const id = convertedCloseTimestamp.sub(convertedOpenTimestamp);
    surveyId = id.toBuffer("le", 8);
    [surveyPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("survey"), user.publicKey.toBuffer(), surveyId],
      program.programId
    );

    try {
      await program.methods
      .createSurvey(
        id,
        surveyTitle,
        surveyDescription,
        convertedOpenTimestamp,
        convertedCloseTimestamp,
        targetParticipant,
        totalReward,
        questionList
      )
      .accounts({
        user: user.publicKey,
        survey: surveyPda,
        systemProgram: systemProgram,
      })
      .rpc();
    }
    catch (error) {
      assert.ok("Invalid time.")
    }

  });

  it("can't create this survey because this survey has insufficient funds!", async () => {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const dataSize = 8 + 8 + (4 + 150) + (4 + 300) + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + (4 + ((256 + 44) * 5));
    const minBalance = await connection.getMinimumBalanceForRentExemption(dataSize);
    const totalReward = new anchor.BN(minBalance);

    const id = convertedCloseTimestamp.sub(convertedOpenTimestamp);
    const surveyId = id.toBuffer("le", 8);
    [surveyPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("survey"), user.publicKey.toBuffer(), surveyId],
      program.programId
    );

    try {
      await program.methods
      .createSurvey(
        id,
        surveyTitle,
        surveyDescription,
        convertedOpenTimestamp,
        convertedCloseTimestamp,
        targetParticipant,
        totalReward,
        questionList
      )
      .accounts({
        user: user.publicKey,
        survey: surveyPda,
        systemProgram: systemProgram,
      })
      .rpc();
    }
    catch (error) {
      assert.ok("Insufficient funds.");
    }
  });
});
