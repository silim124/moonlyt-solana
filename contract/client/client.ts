import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { getMint, getAssociatedTokenAddress } from "@solana/spl-token";
import { Metaplex } from "@metaplex-foundation/metaplex";
import type { MoonpassPool } from "../target/types/moonpass_pool";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.MoonpassPool as anchor.Program<MoonpassPool>;


// metaplex token metadata program ID
const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// metaplex setup
const metaplex = Metaplex.load(program.provider.connection);

// token metadata
const metadata = {
  uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
  name: "Solana Gold",
  symbol: "GOLDSOL",
};

// reward token mint PDA
const rewardTokenMintPda = await PublicKey.findProgramAddress(
  [Buffer.from("reward")],
  programId
);

// player data account PDA
const playerPDA = await PublicKey.findProgramAddress(
  [Buffer.from("player"), program.provider.publicKey.toBuffer()],
  programId
);

// reward token mint metadata account address
const rewardTokenMintMetadataPDA = await metaplex.getProgramAddress(
  "metadata",
  {
    mint: rewardTokenMintPda,
  }
);

// player token account address
const playerTokenAccount = await getAssociatedTokenAddress(
  rewardTokenMintPda,
  program.provider.publicKey
);

async function logTransaction(txHash) {
  const { blockhash, lastValidBlockHeight } =
    await program.provider.connection.getRecentBlockhash();

  await program.provider.connection.confirmTransaction(txHash, "confirmed");

  console.log(`Transaction confirmed with signature: ${txHash}`);
}

let txHash;

try {
  const mintData = await getMint(program.provider.connection, rewardTokenMintPda);
  console.log("Mint Already Exists");
} catch {
  txHash = await program.rpc.createMint(
    metadata.uri,
    metadata.name,
    metadata.symbol,
    {
      accounts: {
        admin: program.provider.publicKey,
        moonPassTokenMint: rewardTokenMintPda,
        metadataAccount: rewardTokenMintMetadataPDA,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: web3.SYSVAR_RENT_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
      },
    }
  );

  await logTransaction(txHash);
}

console.log("Token Mint: ", rewardTokenMintPda.toString());
