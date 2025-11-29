export class DataEncryptorDecryptor {
  async encrypt(password: string, text: Uint8Array): Promise<string> {
    const encoder = new TextEncoder();

    const rawKey = encoder.encode(password);
    const keyMaterial = await crypto.subtle.importKey(
      "raw",
      rawKey,
      { name: "PBKDF2" },
      false,
      ["deriveKey"]
    );

    const salt = crypto.getRandomValues(new Uint8Array(16));

    const key = await crypto.subtle.deriveKey(
      {
        name: "PBKDF2",
        salt,
        iterations: 100_000,
        hash: "SHA-256",
      },
      keyMaterial,
      { name: "AES-GCM", length: 256 },
      false,
      ["encrypt"]
    );

    const iv = crypto.getRandomValues(new Uint8Array(12));

    const ciphertext = await crypto.subtle.encrypt(
      { name: "AES-GCM", iv },
      key,
      new Uint8Array(text)
    );

    const b64Salt = this.arrayBufferToBase64(salt);
    const b64Iv = this.arrayBufferToBase64(iv);
    const b64Cipher = this.arrayBufferToBase64(ciphertext);

    return `${b64Salt}:${b64Iv}:${b64Cipher}`;
  }

  async decrypt(password: string, packed: string): Promise<ArrayBuffer> {
    const encoder = new TextEncoder();

    const [b64Salt, b64Iv, b64Cipher] = packed.split(":");
    const salt = this.base64ToUint8Array(b64Salt);
    const iv = this.base64ToUint8Array(b64Iv);
    const ciphertext = this.base64ToUint8Array(b64Cipher);

    const rawKey = encoder.encode(password);
    const keyMaterial = await crypto.subtle.importKey(
      "raw",
      rawKey,
      { name: "PBKDF2" },
      false,
      ["deriveKey"]
    );

    const key = await crypto.subtle.deriveKey(
      {
        name: "PBKDF2",
        salt,
        iterations: 100_000,
        hash: "SHA-256",
      },
      keyMaterial,
      { name: "AES-GCM", length: 256 },
      false,
      ["decrypt"]
    );

    const decrypted = await crypto.subtle.decrypt(
      { name: "AES-GCM", iv },
      key,
      ciphertext
    );

    return decrypted;
  }

  private arrayBufferToBase64(buffer: ArrayBuffer | Uint8Array) {
    const bytes =
      buffer instanceof Uint8Array ? buffer : new Uint8Array(buffer);
    let binary = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  }

  private base64ToUint8Array(base64: string) {
    const binary = atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) {
      bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
  }
}
