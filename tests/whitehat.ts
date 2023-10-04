import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { Whitehat } from "../target/types/whitehat";

const commitment: Commitment = "confirmed";

describe("whitehat", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Whitehat as Program<Whitehat>;
  const connection: Connection = anchor.getProvider().connection;

  const owner = new Keypair();
  const signer = new Keypair();
  const payout = new Keypair();

  //  owner
  //  auth
  //  sol_vault
  //  state
  //  protocol
  //  system_program

  const protocol = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol"), owner.publicKey.toBuffer()],
    program.programId
  )[0];

  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), protocol.toBytes()],
    program.programId
  )[0];

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), protocol.toBytes()],
    program.programId
  )[0];

  const percent = new BN(10);
  const seed = new BN(1);

  const message = new Uint8Array([
    4, 123, 82, 201, 77, 181, 1, 94, 165, 47, 19, 121, 237, 205, 155, 44, 151,
    64, 216, 179, 28, 13, 172, 233, 31, 75, 1, 231, 170, 79, 20, 55, 200, 247,
    173, 52, 177, 192, 216, 13, 85, 110, 56, 42, 133, 115, 167, 0, 102, 157, 27,
    153, 226, 131, 210, 205, 167, 54, 172, 18, 124, 137, 4, 221, 93, 223, 197,
    252, 200, 248, 105, 188, 151, 153, 237, 215, 242, 73, 105, 189, 8, 77, 98,
    85, 113, 42, 195, 14, 221, 104, 236, 160, 141, 113, 230, 25, 41, 235, 29,
    199, 13, 246, 142, 217, 177, 174, 9, 130, 131,
  ]);

  const amount = new BN(10 * LAMPORTS_PER_SOL);

  const hack = PublicKey.findProgramAddressSync(
    // b"hack", protocol.key().as_ref(), amount.to_le_bytes().as_ref()
    [Buffer.from("hack"), protocol.toBytes(), amount.toBuffer("le", 8)],
    program.programId
  )[0];

  it("airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        owner.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        signer.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("register protocol", async () => {
    await program.methods
      .registerProtocol("whitehat", percent)
      .accounts({
        owner: owner.publicKey,
        auth,
        vault,
        protocol,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("report vulnerability", async () => {
    const protocolPda = await program.account.protocol.fetch(protocol);

    const vulnerability = PublicKey.findProgramAddressSync(
      // b"vulnerability", protocol.key().as_ref(), protocol.vulnerabilities.to_le_bytes().as_ref(), seed.to_le_bytes().as_ref()
      [
        Buffer.from("vulnerability"),
        protocol.toBytes(),
        new BN(protocolPda.vulnerabilities.toNumber() + 1).toArrayLike(
          Buffer,
          "le",
          8
        ),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    await program.methods
      .reportVulnerability(
        Buffer.from(message),
        new BN(protocolPda.vulnerabilities.toNumber() + 1),
        seed
      )
      .accounts({
        signer: signer.publicKey,
        payout: payout.publicKey,
        protocol,
        vulnerability,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirmTx);
  });

  it("approve vulnerability", async () => {
    const protocolPda = await program.account.protocol.fetch(protocol);

    console.log(
      `protocol have ${protocolPda.vulnerabilities.toNumber()} vulnerabilities`
    );

    const vulnerability = PublicKey.findProgramAddressSync(
      // b"vulnerability", protocol.key().as_ref(), id.to_le_bytes().as_ref(), seed.to_le_bytes().as_ref()
      [
        Buffer.from("vulnerability"),
        protocol.toBytes(),
        protocolPda.vulnerabilities.toArrayLike(Buffer, "le", 8),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    await program.methods
      .approveVulnerability()
      .accounts({
        owner: owner.publicKey,
        protocol,
        vulnerability,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
  });

  it("deposit hacked funds", async () => {
    const protocolPda = await program.account.protocol.fetch(protocol);

    const vulnerability = PublicKey.findProgramAddressSync(
      // b"vulnerability", protocol.key().as_ref(), id.to_le_bytes().as_ref(), seed.to_le_bytes().as_ref()
      [
        Buffer.from("vulnerability"),
        protocol.toBytes(),
        protocolPda.vulnerabilities.toArrayLike(Buffer, "le", 8),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    await program.methods
      .depositSolHack(amount)
      .accounts({
        signer: signer.publicKey,
        payout: payout.publicKey,
        protocol,
        vulnerability,
        hack,
        vault,
        // vulnerability: doxxVulnerability,
        systemProgram: SystemProgram.programId,
      })
      .signers([signer])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        console.log(
          "new vault balance : ",
          (await connection.getBalance(vault)) / LAMPORTS_PER_SOL + " sol"
        );
      });
  });

  it("approve hack", async () => {
    await program.methods
      .approveSolHack()
      .accounts({
        owner: owner.publicKey,
        payout: payout.publicKey,
        protocol,
        hack,
        auth,
        vault,
        // vulnerability,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        console.log(
          "new vault balance : ",
          (await connection.getBalance(vault)) / LAMPORTS_PER_SOL + " sol"
        );
        console.log(
          "new payout balance : ",
          (await connection.getBalance(payout.publicKey)) / LAMPORTS_PER_SOL +
            " sol"
        );
      });
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  );
};
