import type { SdkworkGenerationDigest } from "../generation";
import { createSdkworkGenerationPanelStyle } from "../generation-appearance";
import { useSdkworkGenerationIntl } from "../generation-intl";

export interface SdkworkGenerationRunSummaryProps {
  digest: SdkworkGenerationDigest;
}

export function SdkworkGenerationRunSummary({
  digest,
}: SdkworkGenerationRunSummaryProps) {
  const { copy, formatInteger } = useSdkworkGenerationIntl();

  return (
    <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-5">
      <article
        className="rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)]"
        style={createSdkworkGenerationPanelStyle("brand", {
          backgroundWeight: 8,
          borderWeight: 24,
        })}
      >
        <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.totalRuns}</div>
        <div className="mt-1 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{formatInteger(digest.totalRuns)}</div>
      </article>
      <article
        className="rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)]"
        style={createSdkworkGenerationPanelStyle("success", {
          backgroundWeight: 10,
          borderWeight: 26,
        })}
      >
        <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.completed}</div>
        <div className="mt-1 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{formatInteger(digest.completedRuns)}</div>
      </article>
      <article
        className="rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)]"
        style={createSdkworkGenerationPanelStyle("accent", {
          backgroundWeight: 10,
          borderWeight: 24,
        })}
      >
        <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.running}</div>
        <div className="mt-1 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{formatInteger(digest.runningRuns)}</div>
      </article>
      <article
        className="rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)]"
        style={createSdkworkGenerationPanelStyle("danger", {
          backgroundWeight: 10,
          borderWeight: 28,
        })}
      >
        <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.failed}</div>
        <div className="mt-1 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{formatInteger(digest.failedRuns)}</div>
      </article>
      <article
        className="rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)]"
        style={createSdkworkGenerationPanelStyle("neutral", {
          backgroundWeight: 8,
          borderWeight: 24,
        })}
      >
        <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.tokensUsed}</div>
        <div className="mt-1 text-xl font-semibold text-[var(--sdk-color-text-primary)]">{formatInteger(digest.totalTokensUsed)}</div>
      </article>
    </div>
  );
}
