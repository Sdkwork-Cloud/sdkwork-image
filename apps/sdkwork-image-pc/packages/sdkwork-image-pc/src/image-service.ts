import { readPcReactRuntimeSession } from "@sdkwork/core-pc-react";
import {
  createDefaultSdkworkImagePresets,
  createEmptySdkworkImageWorkspace,
  type SdkworkImageAsset,
  type SdkworkImagePreset,
  type SdkworkImageWorkspaceData,
} from "./image";

export interface CreateSdkworkImageServiceOptions {
  getSessionTokens?: () => { authToken?: string };
  images?: readonly SdkworkImageAsset[];
  listImages?: () => Promise<readonly SdkworkImageAsset[]>;
  presets?: readonly SdkworkImagePreset[];
}

export interface SdkworkImageService {
  getEmptyWorkspace(): SdkworkImageWorkspaceData;
  getWorkspace(): Promise<SdkworkImageWorkspaceData>;
}

function normalizeText(value: string | undefined): string {
  return (value ?? "").trim().toLowerCase();
}

export function createSdkworkImageService(options: CreateSdkworkImageServiceOptions = {}): SdkworkImageService {
  const getSessionTokens = options.getSessionTokens ?? (() => readPcReactRuntimeSession());

  return {
    getEmptyWorkspace() {
      return createEmptySdkworkImageWorkspace({
        images: options.images,
        isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
        presets: options.presets ?? createDefaultSdkworkImagePresets(),
      });
    },

    async getWorkspace() {
      try {
        const images = options.listImages ? await options.listImages() : options.images;
        return createEmptySdkworkImageWorkspace({
          images,
          isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
          presets: options.presets ?? createDefaultSdkworkImagePresets(),
        });
      } catch {
        return createEmptySdkworkImageWorkspace({
          images: options.images,
          isAuthenticated: Boolean(normalizeText(getSessionTokens().authToken)),
          presets: options.presets ?? createDefaultSdkworkImagePresets(),
        });
      }
    },
  };
}

export const sdkworkImageService = createSdkworkImageService();
