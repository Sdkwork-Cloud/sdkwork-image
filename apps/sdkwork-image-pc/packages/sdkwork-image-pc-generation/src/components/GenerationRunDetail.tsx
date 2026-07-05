import { EmptyState } from "@sdkwork/ui-pc-react";
import type { SdkworkGenerationRun, SdkworkGenerationStatus } from "../generation";
import {
  createSdkworkGenerationPanelStyle,
  createSdkworkGenerationToneStyle,
  type SdkworkGenerationVisualTone,
} from "../generation-appearance";
import { useSdkworkGenerationIntl } from "../generation-intl";

export interface SdkworkGenerationRunDetailProps {
  run: SdkworkGenerationRun | null;
}

const STATUS_DOT_CLASS: Record<SdkworkGenerationStatus, string> = {
  completed: "bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.5)]",
  running: "bg-blue-400 shadow-[0_0_8px_rgba(96,165,250,0.5)] animate-pulse",
  failed: "bg-red-400 shadow-[0_0_8px_rgba(248,113,113,0.5)]",
  queued: "bg-amber-400 shadow-[0_0_8px_rgba(251,191,36,0.5)]",
};

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
          className="inline-flex items-center gap-2 rounded-full border px-3 py-1 text-[0.68rem] font-semibold uppercase tracking-[0.16em]"
          style={createSdkworkGenerationToneStyle(tone, {
            backgroundWeight: 14,
            borderWeight: 26,
          })}
        >
          <span className={`inline-block h-1.5 w-1.5 rounded-full ${STATUS_DOT_CLASS[run.status]}`} />
          {formatStatusLabel(run.status)}
        </span>
        <span className="rounded-full bg-[var(--sdk-color-surface-panel-muted)] px-3 py-1 text-[0.68rem] font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-muted)]">
          {run.model}
        </span>
      </div>
      <h3 className="mt-4 text-2xl font-semibold tracking-tight text-[var(--sdk-color-text-primary)]">{run.title}</h3>
      <p className="mt-3 text-sm leading-7 text-[var(--sdk-color-text-secondary)]">{run.promptPreview}</p>
      <div className="mt-5 grid gap-3 text-sm sm:grid-cols-2">
        <div className="rounded-[1rem] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_40%,transparent)] bg-[var(--sdk-color-surface-panel-muted)] px-4 py-3">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.detail.latencyLabel}</div>
          <div className="mt-1 font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">{formatLatency(run.latencyMs)}</div>
        </div>
        <div className="rounded-[1rem] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_40%,transparent)] bg-[var(--sdk-color-surface-panel-muted)] px-4 py-3">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.detail.tokensLabel}</div>
          <div className="mt-1 font-semibold tabular-nums text-[var(--sdk-color-text-primary)]">{formatInteger(run.tokensUsed)}</div>
        </div>
      </div>
    </article>
  );
}
