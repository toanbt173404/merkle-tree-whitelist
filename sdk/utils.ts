import { MerkleTree } from 'merkletreejs';
import { keccak_256 } from '@noble/hashes/sha3';
import { PublicKey } from '@solana/web3.js';

interface MerkleTreeEntry {
  address: PublicKey;
  amount: number;
}

const encodeEntry = (entry: MerkleTreeEntry): Uint8Array => {
  const addressBuffer = entry.address.toBuffer();
  const amountBuffer = Buffer.alloc(8);
  amountBuffer.writeBigUInt64BE(BigInt(entry.amount), 0);
  return Buffer.concat([addressBuffer, amountBuffer]);
};

export const getMerkleTree = (entries: MerkleTreeEntry[]): MerkleTree => {
  const leaves = entries.map((entry) => keccak_256(encodeEntry(entry)));
  return new MerkleTree(leaves, keccak_256, { sortPairs: true });
};

export const getMerkleRoot = (entries: MerkleTreeEntry[]): string => {
  return getMerkleTree(entries).getRoot().toString('hex');
};

export const getMerkleProof = (
  entries: MerkleTreeEntry[],
  leaf: MerkleTreeEntry,
  index?: number
): Uint8Array[] => {
  const leafBuffer = Buffer.from(keccak_256(encodeEntry(leaf)));
  return getMerkleTree(entries)
    .getProof(leafBuffer, index)
    .map((proofItem) => proofItem.data);
};

export const getMerkleProofAtIndex = (
  entries: MerkleTreeEntry[],
  index: number
): Uint8Array[] => {
  if (index < 0 || index >= entries.length) {
    throw new Error('Invalid index');
  }
  return getMerkleProof(entries, entries[index], index);
};
