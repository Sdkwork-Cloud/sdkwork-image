import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { ImageApiResult, ImageOperationCommand } from '../types';


export interface ImagePresetsListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  sort?: string;
  q?: string;
}

export class ImagePresetsApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Presets list. */
  async list(params?: ImagePresetsListParams): Promise<ImageApiResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ImageApiResult>(appendQueryString(backendApiPath(`/image/presets`), query));
  }

/** Presets create. */
  async create(body: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.post<ImageApiResult>(backendApiPath(`/image/presets`), body, undefined, undefined, 'application/json');
  }

/** Presets delete. */
  async delete(presetId: string): Promise<ImageApiResult> {
    return this.client.delete<ImageApiResult>(backendApiPath(`/image/presets/${serializePathParameter(presetId, { name: 'presetId', style: 'simple', explode: false })}`));
  }

/** Presets retrieve. */
  async retrieve(presetId: string): Promise<ImageApiResult> {
    return this.client.get<ImageApiResult>(backendApiPath(`/image/presets/${serializePathParameter(presetId, { name: 'presetId', style: 'simple', explode: false })}`));
  }

/** Presets update. */
  async update(presetId: string, body?: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.patch<ImageApiResult>(backendApiPath(`/image/presets/${serializePathParameter(presetId, { name: 'presetId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface ImageGenerationJobsListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  sort?: string;
  q?: string;
}

export class ImageGenerationJobsApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Generation Jobs list. */
  async list(params?: ImageGenerationJobsListParams): Promise<ImageApiResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ImageApiResult>(appendQueryString(backendApiPath(`/image/generation_jobs`), query));
  }

/** Generation Jobs retrieve. */
  async retrieve(jobId: string): Promise<ImageApiResult> {
    return this.client.get<ImageApiResult>(backendApiPath(`/image/generation_jobs/${serializePathParameter(jobId, { name: 'jobId', style: 'simple', explode: false })}`));
  }

/** Generation Jobs cancel. */
  async cancel(jobId: string, body: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.post<ImageApiResult>(backendApiPath(`/image/generation_jobs/${serializePathParameter(jobId, { name: 'jobId', style: 'simple', explode: false })}/cancel`), body, undefined, undefined, 'application/json');
  }
}

export interface ImageGalleriesItemsListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  sort?: string;
  q?: string;
}

export class ImageGalleriesItemsApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Galleries items list. */
  async list(galleryId: string, params?: ImageGalleriesItemsListParams): Promise<ImageApiResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ImageApiResult>(appendQueryString(backendApiPath(`/image/galleries/${serializePathParameter(galleryId, { name: 'galleryId', style: 'simple', explode: false })}/items`), query));
  }

/** Galleries items create. */
  async create(galleryId: string, body: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.post<ImageApiResult>(backendApiPath(`/image/galleries/${serializePathParameter(galleryId, { name: 'galleryId', style: 'simple', explode: false })}/items`), body, undefined, undefined, 'application/json');
  }

/** Galleries items delete. */
  async delete(galleryId: string, itemId: string): Promise<ImageApiResult> {
    return this.client.delete<ImageApiResult>(backendApiPath(`/image/galleries/${serializePathParameter(galleryId, { name: 'galleryId', style: 'simple', explode: false })}/items/${serializePathParameter(itemId, { name: 'itemId', style: 'simple', explode: false })}`));
  }
}

export interface ImageGalleriesListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  sort?: string;
  q?: string;
}

export class ImageGalleriesApi {
  private client: HttpClient;
  public readonly items: ImageGalleriesItemsApi;
  
  constructor(client: HttpClient) { 
    this.client = client;
    this.items = new ImageGalleriesItemsApi(client); 
  }


/** Galleries list. */
  async list(params?: ImageGalleriesListParams): Promise<ImageApiResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ImageApiResult>(appendQueryString(backendApiPath(`/image/galleries`), query));
  }

/** Galleries create. */
  async create(body: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.post<ImageApiResult>(backendApiPath(`/image/galleries`), body, undefined, undefined, 'application/json');
  }

/** Galleries delete. */
  async delete(galleryId: string): Promise<ImageApiResult> {
    return this.client.delete<ImageApiResult>(backendApiPath(`/image/galleries/${serializePathParameter(galleryId, { name: 'galleryId', style: 'simple', explode: false })}`));
  }

/** Galleries retrieve. */
  async retrieve(galleryId: string): Promise<ImageApiResult> {
    return this.client.get<ImageApiResult>(backendApiPath(`/image/galleries/${serializePathParameter(galleryId, { name: 'galleryId', style: 'simple', explode: false })}`));
  }

/** Galleries update. */
  async update(galleryId: string, body?: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.patch<ImageApiResult>(backendApiPath(`/image/galleries/${serializePathParameter(galleryId, { name: 'galleryId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface ImageEditTasksListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  sort?: string;
  q?: string;
}

export class ImageEditTasksApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Edit Tasks list. */
  async list(params?: ImageEditTasksListParams): Promise<ImageApiResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ImageApiResult>(appendQueryString(backendApiPath(`/image/edit_tasks`), query));
  }

/** Edit Tasks retrieve. */
  async retrieve(taskId: string): Promise<ImageApiResult> {
    return this.client.get<ImageApiResult>(backendApiPath(`/image/edit_tasks/${serializePathParameter(taskId, { name: 'taskId', style: 'simple', explode: false })}`));
  }
}

export interface ImageAssetsListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  sort?: string;
  q?: string;
}

export class ImageAssetsApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Assets list. */
  async list(params?: ImageAssetsListParams): Promise<ImageApiResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'sort', value: params?.sort, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<ImageApiResult>(appendQueryString(backendApiPath(`/image/assets`), query));
  }

/** Assets delete. */
  async delete(assetId: string): Promise<ImageApiResult> {
    return this.client.delete<ImageApiResult>(backendApiPath(`/image/assets/${serializePathParameter(assetId, { name: 'assetId', style: 'simple', explode: false })}`));
  }

/** Assets retrieve. */
  async retrieve(assetId: string): Promise<ImageApiResult> {
    return this.client.get<ImageApiResult>(backendApiPath(`/image/assets/${serializePathParameter(assetId, { name: 'assetId', style: 'simple', explode: false })}`));
  }

/** Assets update. */
  async update(assetId: string, body?: ImageOperationCommand): Promise<ImageApiResult> {
    return this.client.patch<ImageApiResult>(backendApiPath(`/image/assets/${serializePathParameter(assetId, { name: 'assetId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class ImageApi {
  private client: HttpClient;
  public readonly assets: ImageAssetsApi;
  public readonly editTasks: ImageEditTasksApi;
  public readonly galleries: ImageGalleriesApi;
  public readonly generationJobs: ImageGenerationJobsApi;
  public readonly presets: ImagePresetsApi;
  
  constructor(client: HttpClient) { 
    this.client = client;
    this.assets = new ImageAssetsApi(client);
    this.editTasks = new ImageEditTasksApi(client);
    this.galleries = new ImageGalleriesApi(client);
    this.generationJobs = new ImageGenerationJobsApi(client);
    this.presets = new ImagePresetsApi(client); 
  }

}

export function createImageApi(client: HttpClient): ImageApi {
  return new ImageApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
