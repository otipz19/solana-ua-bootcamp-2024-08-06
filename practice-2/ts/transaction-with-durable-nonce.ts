import {
  clusterApiUrl,
  Connection,
  Keypair,
  NONCE_ACCOUNT_LENGTH,
  NonceAccount,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { getKeypairFromEnv } from "./util";
import {
  createMint,
  createMintToInstruction,
  createMultisig,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { getExplorerLink } from "@solana-developers/helpers";

const connection = new Connection(clusterApiUrl("devnet"));

const firstSigner = getKeypairFromEnv("FIRST");
const secondSigner = getKeypairFromEnv("SECOND");

const multisig = await createMultisig(
  connection,
  firstSigner,
  [firstSigner.publicKey, secondSigner.publicKey],
  2
);
console.log(`Created multisig account: ${getExplorerLink("address", multisig.toString(), "devnet")}`);

const MINOR_UNITS_PER_MAJOR_UNITS = 1000;
const mint = await createMint(
  connection,
  firstSigner,
  multisig,
  multisig,
  Math.log10(MINOR_UNITS_PER_MAJOR_UNITS)
);
console.log(`Created mint: ${getExplorerLink("address", mint.toString(), "devnet")}`);

const receiver = new PublicKey("J4cYqCez4DeXqD3omAoyNuazwr6EPqohZJVMjDzmZp2o");
const receiverTokenPda = await getOrCreateAssociatedTokenAccount(
  connection,
  firstSigner,
  mint,
  receiver
);
console.log(`Created token account for receiver: ${getExplorerLink("address", receiverTokenPda.address.toString(), "devnet")}`);

const nonceKeypair = Keypair.generate();
const nonceRent = await connection.getMinimumBalanceForRentExemption(NONCE_ACCOUNT_LENGTH);
await sendAndConfirmTransaction(
  connection,
  new Transaction().add(
    SystemProgram.createNonceAccount({
      fromPubkey: firstSigner.publicKey,
      noncePubkey: nonceKeypair.publicKey,
      authorizedPubkey: firstSigner.publicKey,
      lamports: nonceRent,
    })
  ),
  [firstSigner, nonceKeypair]
);
console.log(`Created nonce account: ${getExplorerLink("address", nonceKeypair.publicKey.toBase58(), "devnet")}`);

const nonceAccountInfo = await connection.getAccountInfo(nonceKeypair.publicKey, "confirmed");
if (!nonceAccountInfo) {
  throw new Error("Failed to fetch nonce account info!");
}
const nonceAccount = NonceAccount.fromAccountData(nonceAccountInfo.data);

const mintToTx = new Transaction({
  feePayer: firstSigner.publicKey,
  nonceInfo: {
    nonce: nonceAccount.nonce,
    nonceInstruction: SystemProgram.nonceAdvance({
      authorizedPubkey: firstSigner.publicKey,
      noncePubkey: nonceKeypair.publicKey,
    }),
  },
}).add(createMintToInstruction(
    mint,
    receiverTokenPda.address,
    multisig,
    MINOR_UNITS_PER_MAJOR_UNITS,
    [firstSigner, secondSigner.publicKey]
  )
);
mintToTx.partialSign(firstSigner);

const mintToTxSerialized = mintToTx.serialize({ requireAllSignatures: false });

console.log(`[${getCurTime()}] Created and partially signed transaction. Waiting 3 minutes before passing transaction to second signer!`);
await new Promise((resolve) => setTimeout(resolve, 3 * 60 * 1000));
console.log("3 minutes of waiting have elapsed!");

const mintToTxDeserialized = Transaction.from(mintToTxSerialized);
mintToTxDeserialized.partialSign(secondSigner);
const mintToSignature = await connection.sendRawTransaction(mintToTxDeserialized.serialize());

console.log(`[${getCurTime()}] Sent mint to transaction: ${getExplorerLink("transaction", mintToSignature, "devnet")}`);

function getCurTime(): string {
  const date = new Date();
  return `${date.getHours()}:${date.getMinutes()}`;
}
