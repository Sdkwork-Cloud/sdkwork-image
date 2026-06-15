import { EmptyState } from "@sdkwork/ui-pc-react";
import type { SdkworkGenerationRun } from "../generation";
import { createSdkworkGenerationPanelStyle } from "../generation-appearance";
import { useSdkworkGenerationIntl } from "../generation-intl";

export interface SdkworkGenerationRunListProps {
  onSelectRun?: (runId: string) => void;
  runs: readonly SdkworkGenerationRun[];
  selectedRunId?: string | null;
}

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
            className={`w-full rounded-[1.25rem] border p-4 text-left transition-all ${
              isSelected
                ? "shadow-[var(--sdk-shadow-soft)]"
                : "border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] hover:-translate-y-0.5 hover:shadow-[var(--sdk-shadow-soft)]"
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
              <h3 className="font-semibold text-[var(--sdk-color-text-primary)]">{run.title}</h3>
              <span className="text-xs uppercase tracking-[0.14em] text-[var(--sdk-color-text-muted)]">{formatStatusLabel(run.status)}</span>
            </div>
            <p className="mt-2 text-xs leading-6 text-[var(--sdk-color-text-secondary)]">{run.promptPreview}</p>
          </button>
        );
      })}
    </div>
  );
}
