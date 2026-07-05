import { EmptyState } from "@sdkwork/ui-pc-react";
import type { SdkworkGenerationRun, SdkworkGenerationStatus } from "../generation";
import { createSdkworkGenerationPanelStyle } from "../generation-appearance";
import { useSdkworkGenerationIntl } from "../generation-intl";

export interface SdkworkGenerationRunListProps {
  onSelectRun?: (runId: string) => void;
  runs: readonly SdkworkGenerationRun[];
  selectedRunId?: string | null;
}

const STATUS_DOT_CLASS: Record<SdkworkGenerationStatus, string> = {
  completed: "bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.5)]",
  running: "bg-blue-400 shadow-[0_0_8px_rgba(96,165,250,0.5)] animate-pulse",
  failed: "bg-red-400 shadow-[0_0_8px_rgba(248,113,113,0.5)]",
  queued: "bg-amber-400 shadow-[0_0_8px_rgba(251,191,36,0.5)]",
};

export function SdkworkGenerationRunList({
  onSelectRun,
  runs,
  selectedRunId,
}: SdkworkGenerationRunListProps) {
  const {
    copy,
    formatOpenRunLabel,
    formatStatusLabel,
  } = useSdkworkGenerationIntl();

  if (runs.length === 0) {
    return (
      <EmptyState
        description={copy.empty.noRunsDescription}
        title={copy.empty.noRunsTitle}
      />
    );
  }

  return (
    <div className="grid gap-3">
      {runs.map((run) => {
        const isSelected = run.id === selectedRunId;
        return (
          <button
            aria-label={formatOpenRunLabel(run.title)}
            className={`group w-full rounded-[1.25rem] border p-4 text-left transition-all duration-200 ${
              isSelected
                ? "shadow-[var(--sdk-shadow-soft)]"
                : "border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] hover:-translate-y-0.5 hover:border-[color-mix(in_srgb,var(--sdk-color-brand-primary)_24%,var(--sdk-color-border-default))] hover:shadow-[var(--sdk-shadow-soft)]"
            }`}
            key={run.id}
            onClick={() => onSelectRun?.(run.id)}
            style={isSelected ? createSdkworkGenerationPanelStyle("brand", {
              backgroundWeight: 12,
              borderWeight: 34,
              surfaceColor: "var(--sdk-color-surface-panel-muted)",
              surfaceWeight: 94,
            }) : undefined}
            type="button"
          >
            <div className="flex items-center justify-between gap-3">
              <div className="flex min-w-0 items-center gap-2.5">
                <span className={`inline-block h-2 w-2 shrink-0 rounded-full ${STATUS_DOT_CLASS[run.status]}`} />
                <h3 className="truncate font-semibold text-[var(--sdk-color-text-primary)]">{run.title}</h3>
              </div>
              <span className="shrink-0 text-xs uppercase tracking-[0.14em] text-[var(--sdk-color-text-muted)]">{formatStatusLabel(run.status)}</span>
            </div>
            <p className="mt-2 line-clamp-2 text-xs leading-6 text-[var(--sdk-color-text-secondary)]">{run.promptPreview}</p>
          </button>
        );
      })}
    </div>
  );
}
