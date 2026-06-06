import {
  useMemo,
  useSyncExternalStore,
} from "react";
import {
  sortSdkworkGenerationRuns,
  type SdkworkGenerationRun,
  type SdkworkGenerationSortBy,
  type SdkworkGenerationStatus,
  type SdkworkGenerationWorkspaceData,
} from "./generation.ts";
import {
  createSdkworkGenerationMessages,
  type SdkworkGenerationMessagesOverrides,
} from "./generation-copy.ts";
import {
  createSdkworkGenerationService,
  type SdkworkGenerationService,
} from "./generation-service.ts";

export interface SdkworkGenerationControllerState {
  activeStatus: SdkworkGenerationStatus | "all";
  isBootstrapped: boolean;
  isLoading: boolean;
  lastError?: string;
  searchQuery: string;
  selectedRun: SdkworkGenerationRun | null;
  selectedRunId: string | null;
  sortBy: SdkworkGenerationSortBy;
  visibleRuns: SdkworkGenerationRun[];
  workspace: SdkworkGenerationWorkspaceData;
}

export interface SdkworkGenerationController {
  bootstrap(): Promise<SdkworkGenerationControllerState>;
  getState(): SdkworkGenerationControllerState;
  refresh(): Promise<SdkworkGenerationControllerState>;
  selectRun(runId: string | null): void;
  service: SdkworkGenerationService;
  setSearchQuery(query: string): void;
  setSortBy(sortBy: SdkworkGenerationSortBy): void;
  setStatus(status: SdkworkGenerationStatus | "all"): void;
  subscribe(listener: () => void): () => void;
}

export interface CreateSdkworkGenerationControllerOptions {
  initialState?: Partial<SdkworkGenerationControllerState>;
  locale?: string | null;
  messages?: SdkworkGenerationMessagesOverrides;
  service?: Partial<SdkworkGenerationService>;
}

function normalizeText(value: string | undefined): string {
  return (value ?? "").trim().toLowerCase();
}

function filterRuns(
  runs: readonly SdkworkGenerationRun[],
  state: Pick<SdkworkGenerationControllerState, "activeStatus" | "searchQuery" | "sortBy">,
): SdkworkGenerationRun[] {
  const query = normalizeText(state.searchQuery);
  const filtered = runs.filter((run) => {
    if (state.activeStatus !== "all" && run.status !== state.activeStatus) {
      return false;
    }
    if (!query) {
      return true;
    }

    return [
      run.id,
      run.title,
      run.promptPreview,
      run.model,
      run.status,
    ].some((value) => normalizeText(value).includes(query));
  });

  return sortSdkworkGenerationRuns(filtered, state.sortBy);
}

function resolveSelectedRunId(
  runs: readonly SdkworkGenerationRun[],
  selectedRunId: string | null,
): string | null {
  if (selectedRunId && runs.some((run) => run.id === selectedRunId)) {
    return selectedRunId;
  }

  return runs.find((run) => run.status === "running")?.id
    ?? runs[0]?.id
    ?? null;
}

function normalizeState(
  state: SdkworkGenerationControllerState,
): SdkworkGenerationControllerState {
  const visibleRuns = filterRuns(state.workspace.runs, {
    activeStatus: state.activeStatus,
    searchQuery: state.searchQuery,
    sortBy: state.sortBy,
  });
  const selectedRunId = resolveSelectedRunId(visibleRuns, state.selectedRunId);

  return {
    ...state,
    selectedRun: visibleRuns.find((run) => run.id === selectedRunId) ?? null,
    selectedRunId,
    visibleRuns,
  };
}

export function createSdkworkGenerationController(
  options: CreateSdkworkGenerationControllerOptions = {},
): SdkworkGenerationController {
  const copy = createSdkworkGenerationMessages(options.locale, options.messages);
  const service: SdkworkGenerationService = options.service
    ? {
        ...createSdkworkGenerationService(),
        ...options.service,
      }
    : createSdkworkGenerationService();
  const fallbackWorkspace = service.getEmptyWorkspace();
  const listeners = new Set<() => void>();
  let state = normalizeState({
    activeStatus: "all",
    isBootstrapped: false,
    isLoading: false,
    searchQuery: "",
    selectedRun: null,
    selectedRunId: null,
    sortBy: "recent",
    visibleRuns: fallbackWorkspace.runs,
    workspace: fallbackWorkspace,
    ...options.initialState,
  });

  function emit(): void {
    listeners.forEach((listener) => listener());
  }

  function setState(
    next:
      | Partial<SdkworkGenerationControllerState>
      | ((currentState: SdkworkGenerationControllerState) => Partial<SdkworkGenerationControllerState>),
  ): void {
    const partial = typeof next === "function" ? next(state) : next;
    state = normalizeState({
      ...state,
      ...partial,
    });
    emit();
  }

  return {
    async bootstrap() {
      setState({
        isLoading: true,
        lastError: undefined,
      });

      try {
        const workspace = await service.getWorkspace();
        setState({
          isBootstrapped: true,
          isLoading: false,
          workspace,
        });
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
      setState({
        isLoading: true,
        lastError: undefined,
      });

      try {
        const workspace = await service.getWorkspace();
        setState({
          isBootstrapped: true,
          isLoading: false,
          workspace,
        });
        return state;
      } catch (error) {
        setState({
          isLoading: false,
          lastError: error instanceof Error ? error.message : copy.service.loadWorkspaceFailed,
        });
        throw error;
      }
    },

    selectRun(runId) {
      setState({
        selectedRunId: runId,
      });
    },

    service,

    setSearchQuery(query) {
      setState({
        searchQuery: query,
      });
    },

    setSortBy(sortBy) {
      setState({
        sortBy,
      });
    },

    setStatus(status) {
      setState({
        activeStatus: status,
      });
    },

    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}

export function useSdkworkGenerationController(
  controller?: SdkworkGenerationController,
  options?: Pick<CreateSdkworkGenerationControllerOptions, "locale" | "messages" | "service">,
): SdkworkGenerationController {
  const locale = options?.locale;
  const messages = options?.messages;
  const service = options?.service;

  return useMemo(
    () => controller ?? createSdkworkGenerationController({
      ...(locale ? { locale } : {}),
      ...(messages ? { messages } : {}),
      ...(service ? { service } : {}),
    }),
    [controller, locale, messages, service],
  );
}

export function useSdkworkGenerationControllerState(
  controller: SdkworkGenerationController,
): SdkworkGenerationControllerState {
  return useSyncExternalStore(
    controller.subscribe,
    controller.getState,
    controller.getState,
  );
}
