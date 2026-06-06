import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkCustomConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { ImageCompatApi, createImageCompatApi } from './api/image-compat';

export class SdkworkCustomClient {
  private httpClient: HttpClient;

  public readonly imageCompat: ImageCompatApi;

  constructor(config: SdkworkCustomConfig) {
    this.httpClient = createHttpClient(config);
    this.imageCompat = createImageCompatApi(this.httpClient);
  }

  setApiKey(apiKey: string): this {
    this.httpClient.setApiKey(apiKey);
    return this;
  }

  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkCustomConfig): SdkworkCustomClient {
  return new SdkworkCustomClient(config);
}

export default SdkworkCustomClient;
