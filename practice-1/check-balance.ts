import "dotenv/config";
import { Keypair, Connection, PublicKey, LAMPORTS_PER_SOL, clusterApiUrl } from "@solana/web3.js";
import { airdropIfRequired } from "@solana-developers/helpers";

const publicKey = new PublicKey("Dk9TYVQ6Jg7PnsxPGC8m8u7aqKBeGTvvcVGybykLyn6P");
const connection = new Connection(clusterApiUrl("devnet"));
const balance = await connection.getBalance(publicKey);
console.log(`Current balance of ${publicKey.toBase58()} wallet is: ${balance / LAMPORTS_PER_SOL}`);