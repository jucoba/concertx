import crypto from 'crypto'

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ConcertX } from "../../../anchor/target/types/concert_x";

const charset =
  '0123456789ABCDEFGHIJKLMNOPQRSTUVXYZabcdefghijklmnopqrstuvwxyz+/'

export function randomString(length: number) {
  let result = ''
  while (length > 0) {
    const bytes = new Uint8Array(16)
    const random = crypto.webcrypto.getRandomValues(bytes)

    random.forEach((c) => {
      if (length == 0) {
        return
      }
      if (c < charset.length) {
        result += charset[c]
        length--
      }
    })
  }
  return result
}

export async function fetchConcerts() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ConcertX as Program<ConcertX>;
  try {
    const concerts = await program.account.concert.all();
    console.log("Found concerts:", concerts);
    return concerts;
  } catch (error) {
    console.error("Error fetching concerts:", error);
    return null;
  }
}
