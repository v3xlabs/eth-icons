// Blob rejects views over shared memory, so the buffer type is pinned rather than left as ArrayBufferLike.
export type Icon = {
  bytes: Uint8Array<ArrayBuffer>;
  mimeType?: string;
};

export const toBlob = (icon: Icon): Blob =>
  new Blob([icon.bytes], icon.mimeType === undefined ? {} : { type: icon.mimeType });

export const toDataUri = (icon: Icon): string =>
  `data:${icon.mimeType ?? "application/octet-stream"};base64,${toBase64(icon.bytes)}`;

// Spreading a whole icon into String.fromCodePoint overflows the argument limit, so encode in chunks.
const CHUNK_SIZE_BYTES = 0x80_00;

const toBase64 = (bytes: Uint8Array): string => {
  let binary = "";

  for (let offset = 0; offset < bytes.length; offset += CHUNK_SIZE_BYTES) {
    binary += String.fromCodePoint(...bytes.subarray(offset, offset + CHUNK_SIZE_BYTES));
  }

  return btoa(binary);
};
