import axios, { type AxiosInstance } from "axios";
import type { ipfsResponse } from "../type";

export class IPFSUploader {
  private client: AxiosInstance;
  private endpoint: string;

  constructor(endpoint: string) {
    this.client = axios.create();
    this.endpoint = endpoint;
  }

  async uploadChunk(chunk: string): Promise<ipfsResponse> {
    const form = new FormData();

    form.append("file", new Blob([chunk], { type: "text/plain" }), "data.txt");

    const response = await this.client.post(this.endpoint, form, {
      headers: {
        Authorization:
          "Bearer NDhERDFCRDM2QkIzMjU4RDA1MzY6TFA5UnFRSzB5Y3lxUG1tOG13ZHJZN1A2Z2ZaeGhBMnY4aVdwRHRqTDp0ZXN0YnVja2Vy",
      },
    });
    
    return response.data;
  }
}
