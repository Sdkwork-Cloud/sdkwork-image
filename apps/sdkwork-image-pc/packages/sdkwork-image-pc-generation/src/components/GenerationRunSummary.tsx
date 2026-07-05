import { Activity, CheckCircle2, Coins, Loader2, XCircle } from "lucide-react";
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
        className="group rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)] transition-transform duration-200 hover:-translate-y-0.5"
        style={createSdkworkGenerationPanelStyle("brand", {
          backgroundWeight: 8,
          borderWeight: 24,
        })}
      >
        <div className="flex items-center justify-between">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.totalRuns}</div>
          <Activity className="h-3.5 w-3.5 text-[var(--sdk-color-text-muted)] opacity-60 transition-opacity group-hover:opacity-100" />
        </div>
        <div className="mt-1.5 text-xl font-bold tabular-nums text-[var(--sdk-color-text-primary)]">{formatInteger(digest.totalRuns)}</div>
      </article>
      <article
        className="group rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)] transition-transform duration-200 hover:-translate-y-0.5"
        style={createSdkworkGenerationPanelStyle("success", {
          backgroundWeight: 10,
          borderWeight: 26,
        })}
      >
        <div className="flex items-center justify-between">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.completed}</div>
          <CheckCircle2 className="h-3.5 w-3.5 text-emerald-400 opacity-60 transition-opacity group-hover:opacity-100" />
        </div>
        <div className="mt-1.5 text-xl font-bold tabular-nums text-[var(--sdk-color-text-primary)]">{formatInteger(digest.completedRuns)}</div>
      </article>
      <article
        className="group rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)] transition-transform duration-200 hover:-translate-y-0.5"
        style={createSdkworkGenerationPanelStyle("accent", {
          backgroundWeight: 10,
          borderWeight: 24,
        })}
      >
        <div className="flex items-center justify-between">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.running}</div>
          <Loader2 className="h-3.5 w-3.5 animate-spin text-blue-400 opacity-60 transition-opacity group-hover:opacity-100" />
        </div>
        <div className="mt-1.5 text-xl font-bold tabular-nums text-[var(--sdk-color-text-primary)]">{formatInteger(digest.runningRuns)}</div>
      </article>
      <article
        className="group rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)] transition-transform duration-200 hover:-translate-y-0.5"
        style={createSdkworkGenerationPanelStyle("danger", {
          backgroundWeight: 10,
          borderWeight: 28,
        })}
      >
        <div className="flex items-center justify-between">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.failed}</div>
          <XCircle className="h-3.5 w-3.5 text-red-400 opacity-60 transition-opacity group-hover:opacity-100" />
        </div>
        <div className="mt-1.5 text-xl font-bold tabular-nums text-[var(--sdk-color-text-primary)]">{formatInteger(digest.failedRuns)}</div>
      </article>
      <article
        className="group rounded-[1rem] border p-3.5 shadow-[var(--sdk-shadow-soft)] transition-transform duration-200 hover:-translate-y-0.5"
        style={createSdkworkGenerationPanelStyle("neutral", {
          backgroundWeight: 8,
          borderWeight: 24,
        })}
      >
        <div className="flex items-center justify-between">
          <div className="text-xs uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">{copy.summary.tokensUsed}</div>
          <Coins className="h-3.5 w-3.5 text-[var(--sdk-color-text-muted)] opacity-60 transition-opacity group-hover:opacity-100" />
        </div>
        <div className="mt-1.5 text-xl font-bold tabular-nums text-[var(--sdk-color-text-primary)]">{formatInteger(digest.totalTokensUsed)}</div>
      </article>
    </div>
  );
}
