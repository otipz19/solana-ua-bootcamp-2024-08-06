import { Keypair } from "@solana/web3.js";

const prefix = "anza";

let triesCount = 0;
let lastNotifiedTry = 0;
const startTime = new Date().getTime();

let keypair: Keypair;
do {
  keypair = Keypair.generate();
  triesCount++;
  if (triesCount - lastNotifiedTry >= 1_000_000) {
    console.log(
      `Attempt #${triesCount}. ${getElapsedTimeMsg()}`
    );
    lastNotifiedTry = triesCount;
  }
} while (!keypair.publicKey.toBase58().startsWith(prefix));

console.log(
  `Generated public key that starts with ${prefix}: ${keypair.publicKey.toBase58()}\nAttempts: ${triesCount}\n${getElapsedTimeMsg()}`
);

function getElapsedTimeMsg(): string {
  return `Elapsed time since start: ${Math.round(
    (new Date().getTime() - startTime) / 1000
  )} seconds`;
}
