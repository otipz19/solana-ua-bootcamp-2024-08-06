import { Keypair } from "@solana/web3.js";
import "dotenv/config";

export function getKeypairFromEnv(key: string): Keypair {
    const fromEnv = process.env[key];
    if(!fromEnv) {
        throw new Error(`Environment variable ${key} is not set!`);
    }
    return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fromEnv)));
}