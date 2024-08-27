import { clusterApiUrl, Connection, PublicKey } from "@solana/web3.js";
import { getKeypairFromEnv } from "./util";
import { createMint, createMultisig, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { getExplorerLink } from '@solana-developers/helpers';

const connection = new Connection(clusterApiUrl("devnet"));

const firstSigner = getKeypairFromEnv("FIRST");
const secondSigner = getKeypairFromEnv("SECOND");

const multisig = await createMultisig(connection, firstSigner, [firstSigner.publicKey, secondSigner.publicKey], 2);
console.log(`Created multisig account: ${getExplorerLink("address", multisig.toString(), "devnet")}`);

const MINOR_UNITS_PER_MAJOR_UNITS = 1000;
const mint = await createMint(connection, firstSigner, multisig, multisig, Math.log10(MINOR_UNITS_PER_MAJOR_UNITS));
console.log(`Created mint: ${getExplorerLink("address", mint.toString(), "devnet")}`);

const receiver = new PublicKey("J4cYqCez4DeXqD3omAoyNuazwr6EPqohZJVMjDzmZp2o");
const receiverTokenPda = await getOrCreateAssociatedTokenAccount(connection, firstSigner, mint, receiver);
console.log(`Created token account for receiver: ${getExplorerLink("address", receiverTokenPda.address.toString(), "devnet")}`);

const signature = await mintTo(
    connection,
    firstSigner,
    mint,
    receiverTokenPda.address,
    multisig,
    69 * MINOR_UNITS_PER_MAJOR_UNITS,
    [firstSigner, secondSigner]
);
console.log(`Minted multisig token: ${getExplorerLink("transaction", signature, "devnet")}`);