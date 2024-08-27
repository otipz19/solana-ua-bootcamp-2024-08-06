import { clusterApiUrl, Connection, PublicKey, Transaction } from "@solana/web3.js";
import { getKeypairFromEnv } from "./util";
import { createMint, createTransferInstruction, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { getExplorerLink } from "@solana-developers/helpers";

const connection = new Connection(clusterApiUrl("devnet"));

const sender = getKeypairFromEnv("FIRST");
const receiver = new PublicKey("yqqJ3CEu7zikT8Dpks7ueKuY7qAF8uDN1esc15MKTvK");

const MINOR_UNITS_PER_MAJOR_UNITS = 1000;
const mint = await createMint(connection, sender, sender.publicKey, null, Math.log10(1000));
const senderTokenAccount = await getOrCreateAssociatedTokenAccount(connection, sender, mint, sender.publicKey);
const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(connection, sender, mint, receiver);
const mintTx = await mintTo(connection, sender, mint, senderTokenAccount.address, sender, 69 * MINOR_UNITS_PER_MAJOR_UNITS);
console.log(`Minted tokens to sender: ${getExplorerLink("transaction", mintTx, "devnet")}`);

const transaction = new Transaction().add(createTransferInstruction(
    senderTokenAccount.address,
    receiverTokenAccount.address,
    sender.publicKey,
    MINOR_UNITS_PER_MAJOR_UNITS
));

transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
transaction.feePayer = receiver;
transaction.partialSign(sender);
const serializedTransaction = transaction.serialize({requireAllSignatures: false});

const deserializedTransaction = Transaction.from(serializedTransaction);
const receiverKeypair = getKeypairFromEnv("SECOND");
deserializedTransaction.partialSign(receiverKeypair);
const signature = await connection.sendRawTransaction(deserializedTransaction.serialize());

console.log(`Sent token with receiver funding: ${getExplorerLink("transaction", signature, "devnet")}`);