import BN from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import type { RewardNft } from "../target/types/reward_nft";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.RewardNft as anchor.Program<RewardNft>;


(async () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.RewardNft;

  // Crear mint
  const mint = await createMint(
    provider.connection,
    provider.wallet.payer,
    provider.wallet.publicKey,
    null,
    9
  );

  // Crear cuenta asociada
  const userAta = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    provider.wallet.payer,
    mint,
    provider.wallet.publicKey
  );

  // Emitir tokens
  await mintTo(
    provider.connection,
    provider.wallet.payer,
    mint,
    userAta.address,
    provider.wallet.publicKey,
    500
  );

  console.log("Mint:", mint.toBase58());
  console.log("User ATA:", userAta.address.toBase58());

  // Llamar al programa reward_user
  await program.methods
    .rewardUser(new anchor.BN(50))
    .accounts({
      from: userAta.address,
      to: userAta.address,
      authority: provider.wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();

  console.log("Reward ejecutado y tokens transferidos");
})();
