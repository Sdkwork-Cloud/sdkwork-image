import {
  useMemo,
  useSyncExternalStore,
} from "react";
import type { SdkworkImageAsset, SdkworkImageJobStatus, SdkworkImageWorkspaceData } from "./image";
import {
  createSdkworkImageMessages,
  type SdkworkImageMessagesOverrides,
} from "./image-copy";
import { createSdkworkImageService, type SdkworkImageService } from "./image-service";

export interface SdkworkImageControllerState {
  activePreset: string | "all";
  activeStatus: SdkworkImageJobStatus | "all";
  isBootstrapped: boolean;
  isLoading: boolean;
  lastError?: string;
  searchQuery: string;
  visibleImages: SdkworkImageAsset[];
  workspace: SdkworkImageWorkspaceData;
}

export interface CreateSdkworkImageControllerOptions {
  locale?: string | null;
  messages?: SdkworkImageMessagesOverrides;
  service?: Partial<SdkworkImageService>;
}

function normalizeText(value: string): string {
  return value.trim().toLowerCase();
}

function deriveVisibleImages(
  workspace: SdkworkImageWorkspaceData,
  activePreset: string | "all",
  activeStatus: SdkworkImageJobStatus | "all",
  searchQuery: string,
) {
  const query = normalizeText(searchQuery);

  return workspace.images.filter((image) => {
    if (activePreset !== "all" && image.presetId !== activePreset) {
      return false;
    }
    if (activeStatus !== "all" && image.status !== activeStatus) {
      return false;
    }
    if (!query) {
      return true;
    }

    return [image.id, image.title, image.style, image.prompt].some((value) => normalizeText(value).includes(query));
  });
}

function normalizeState(state: SdkworkImageControllerState): SdkworkImageControllerState {
  return {
    ...state,
    visibleImages: deriveVisibleImages(
      state.workspace,
      state.activePreset,
      state.activeStatus,
      state.searchQuery,
    ),
  };
}

export function createSdkworkImageController(options: CreateSdkworkImageControllerOptions = {}) {
  const copy = createSdkworkImageMessages(options.locale, options.messages);
  const service: SdkworkImageService = options.service
    ? {
        ...createSdkworkImageService(),
        ...options.service,
      }
    : createSdkworkImageService();

  const listeners = new Set<() => void>();
  let state = normalizeState({
    activePreset: "all",
    activeStatus: "all",
    isBootstrapped: false,
    isLoading: false,
    searchQuery: "",
    visibleImages: [],
    workspace: service.getEmptyWorkspace(),
  });

  function emit() {
    listeners.forEach((listener) => listener());
  }

  function setState(next: Partial<SdkworkImageControllerState>) {
    state = normalizeState({
      ...state,
      ...next,
    });
    emit();
  }

  return {
    async bootstrap() {
      setState({ isLoading: true, lastError: undefined });
      try {
        const workspace = await service.getWorkspace();
        setState({ isBootstrapped: true, isLoading: false, workspace });
        return state;
      } catch (error) {
        setState({
          isLoading: false,
          lastError: error instanceof Error ? error.message : copy.service.loadWorkspaceFailed,
        });
        throw error;
      }
    },

    getState() {
      return state;
    },

    async refresh() {
      setState({ isLoading: true, lastError: undefined });
      try {
        const workspace = await service.getWorkspace();
        setState({ isBootstrapped: true, isLoading: false, workspace });
        return state;
      } catch (error) {
        setState({
          isLoading: false,
          lastError: error instanceof Error ? error.message : copy.service.loadWorkspaceFailed,
        });
        throw error;
      }
    },

    setPreset(presetId: string | "all") {
      setState({ activePreset: presetId });
    },

    setSearchQuery(query: string) {
      setState({ searchQuery: query });
    },

    setStatus(status: SdkworkImageJobStatus | "all") {
      setState({ activeStatus: status });
    },

    subscribe(listener: () => void) {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
}

export function useSdkworkImageController(
  controller?: ReturnType<typeof createSdkworkImageController>,
  options?: Pick<CreateSdkworkImageControllerOptions, "locale" | "messages" | "service">,
) {
  const locale = options?.locale;
  const messages = options?.messages;
  const service = options?.service;

  return useMemo(
    () => controller ?? createSdkworkImageController({
      ...(locale ? { locale } : {}),
      ...(messages ? { messages } : {}),
      ...(service ? { service } : {}),
    }),
    [controller, locale, messages, service],
  );
}

export function useSdkworkImageControllerState(
  controller: ReturnType<typeof createSdkworkImageController>,
): SdkworkImageControllerState {
  return useSyncExternalStore(controller.subscribe, controller.getState, controller.getState);
}
