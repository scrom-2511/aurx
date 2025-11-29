import React, { useEffect, useRef } from "react";
import { IPFSUploader } from "./services/ipfs_uploader";
import { DataEncryptorDecryptor } from "./services/encrypt_decrypt";

const App = () => {

  const CHUNK_SIZE = 1024 * 1024;

  const uploaderRef = useRef(new IPFSUploader("https://rpc.filebase.io/api/v0/add"));
  const encryptorRef = useRef(new DataEncryptorDecryptor());
  const socketRef = useRef<WebSocket | null>(null);

  const handleOnChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    console.log("hi there1");

    let offset = 0;

    while (offset < file.size) {
      const chunk = file.slice(offset, offset + CHUNK_SIZE);
      const arrayBuffer = await chunk.arrayBuffer();

      console.log("Original:", arrayBuffer);

      const encryptedChunk = await encryptorRef.current.encrypt(
        "this",
        new Uint8Array(arrayBuffer)
      );

      console.log("Encrypted:", encryptedChunk);

      const decryptedChunk = await encryptorRef.current.decrypt(
        "this",
        encryptedChunk
      );

      console.log("Decrypted:", decryptedChunk);

      const { Hash } = await uploaderRef.current.uploadChunk(encryptedChunk);

      socketRef.current?.send(Hash);

      console.log("Uploaded chunk hash:", Hash);

      offset += CHUNK_SIZE;
    }

    console.log("Upload complete!");
  };

  useEffect(() => {
    socketRef.current = new WebSocket("ws://localhost:4000/ws");

    socketRef.current.onopen = () => {
      console.log("connection established!");
      socketRef.current?.send("hello from browser");
    };

    socketRef.current.onmessage = (event) => {
      console.log("Message from server:", event.data);
    };
  }, []);

  return (
    <div className="text-black">
      <input type="file" onChange={handleOnChange} />
      <button
        onClick={() => {
          socketRef.current?.send("hi there");
        }}
      >
        send
      </button>
    </div>
  );
};

export default App;
