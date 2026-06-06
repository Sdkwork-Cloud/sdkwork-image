import { EmptyState } from "@sdkwork/ui-pc-react";
import type { SdkworkGenerationRun } from "../generation";
import {
  createSdkworkGenerationPanelStyle,
  createSdkworkGenerationToneStyle,
  type SdkworkGenerationVisualTone,
} from "../generation-appearance";
import { useSdkworkGenerationIntl } from "../generation-intl";

export interface SdkworkGenerationRunDetailProps {
  run: SdkworkGenerationRun | null;
}

export function SdkworkGenerationRunDetail({
  run,
}: SdkworkGenerationRunDetailProps) {
  const {
    copy,
    formatInteger,
    formatLatency,
    formatStatusLabel,
  } = useSdkworkGenerationIntl();

  if (!run) {
    return (
      <EmptyState
        description={copy.detail.emptyDescription}
        title={copy.detail.emptyTitle}
      />
    );
  }

  const tone: SdkworkGenerationVisualTone = run.status === "completed"
    ? "success"
    : run.status === "failed"
      ? "danger"
      : run.status === "queued"
        ? "warning"
        : "brand";

  return (
    <article
      className="rounded-[1.5rem] border p-5 shadow-[var(--sdk-shadow-soft)]"
      style={createSdkworkGenerationPanelStyle("accent", {
        backgroundWeight: 10,
        borderWeight: 26,
      })}
    >
      <div className="flex flex-wrap items-center gap-2">
        <span
          className="rounded-full border px-3 py-1 text-[0.68rem] font-semibold uppercase tracking-[0.16em]"
          style={createSdkworkGenerationToneStyle(tone, {
            backgroundWeight: 14,
            borderWeight: 26,
          })}
        >
          {formatStatusLabel(run.status)}
        </span>
        <span className="rounded-full bg-[var(--sdk-color-surface-panel-muted)] px-3 py-1 text-[0.68rem] font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-muted)]">
          {run.model}
        </span>
      </div>
      <h3 className="mt-4 text-2xl font-semibold tracking-tight text-[var(--sdk-color-text-primary)]">{run.title}</h3>
      <p className="mt-3 text-sm leading-7 text-[var(--sdk-color-text-secondary)]">{run.promptPreview}</p>
      <div className="mt-4 grid gap-3 text-sm sm:grid-cols-2">
        <div className="rounded-[1rem] bg-[var(--sdk-color-surface-panel-muted)] px-3.5 py-2.5">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.detail.latencyLabel}</div>
          <div className="mt-1 font-medium text-[var(--sdk-color-text-primary)]">{formatLatency(run.latencyMs)}</div>
        </div>
        <div className="rounded-[1rem] bg-[var(--sdk-color-surface-panel-muted)] px-3.5 py-2.5">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.detail.tokensLabel}</div>
          <div className="mt-1 font-medium text-[var(--sdk-color-text-primary)]">{formatInteger(run.tokensUsed)}</div>
        </div>
      </div>
    </article>
  );
}
