import { customApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { ImageOperationCommand } from '../types';


export class ImageCompatCompatOpenaiImagesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Compat openai images generate. */
  async generate(body: ImageOperationCommand): Promise<Record<string, unknown>> {
    return this.client.post<Record<string, unknown>>(customApiPath(`/compat/openai/images/generations`), body, undefined, undefined, 'application/json');
  }
}

export class ImageCompatCompatOpenaiApi {
  private client: HttpClient;
  public readonly images: ImageCompatCompatOpenaiImagesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.images = new ImageCompatCompatOpenaiImagesApi(client);
  }

}

export class ImageCompatCompatApi {
  private client: HttpClient;
  public readonly openai: ImageCompatCompatOpenaiApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.openai = new ImageCompatCompatOpenaiApi(client);
  }

}

export class ImageCompatApi {
  private client: HttpClient;
  public readonly compat: ImageCompatCompatApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.compat = new ImageCompatCompatApi(client);
  }

}

export function createImageCompatApi(client: HttpClient): ImageCompatApi {
  return new ImageCompatApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
