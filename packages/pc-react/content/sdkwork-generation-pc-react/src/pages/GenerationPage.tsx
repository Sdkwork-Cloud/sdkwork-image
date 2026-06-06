import { useEffect } from "react";
import {
  Button,
  LoadingBlock,
  StatusNotice,
} from "@sdkwork/ui-pc-react";
import { SdkworkGenerationRunDetail } from "../components/GenerationRunDetail";
import { SdkworkGenerationRunList } from "../components/GenerationRunList";
import { SdkworkGenerationRunSummary } from "../components/GenerationRunSummary";
import type { SdkworkGenerationStatus } from "../generation";
import {
  createSdkworkGenerationBackdropStyle,
  createSdkworkGenerationHeroStyle,
  createSdkworkGenerationHeroTextStyle,
  createSdkworkGenerationPanelStyle,
} from "../generation-appearance";
import type { SdkworkGenerationMessagesOverrides } from "../generation-copy";
import type { SdkworkGenerationController } from "../generation-controller";
import {
  useSdkworkGenerationController,
  useSdkworkGenerationControllerState,
} from "../generation-controller";
import {
  SdkworkGenerationIntlProvider,
  useSdkworkGenerationIntl,
} from "../generation-intl";
import type { SdkworkGenerationService } from "../generation-service";

export interface SdkworkGenerationPageProps {
  controller?: SdkworkGenerationController;
  locale?: string | null;
  messages?: SdkworkGenerationMessagesOverrides;
  service?: Partial<SdkworkGenerationService>;
}

interface SdkworkGenerationPageContentProps {
  controller?: SdkworkGenerationController;
  locale?: string | null;
  messages?: SdkworkGenerationMessagesOverrides;
  service?: Partial<SdkworkGenerationService>;
}

const statuses: Array<SdkworkGenerationStatus | "all"> = ["all", "running", "completed", "failed", "queued"];

function SdkworkGenerationPageContent({
  controller: controllerProp,
  locale,
  messages,
  service,
}: SdkworkGenerationPageContentProps) {
  const controller = useSdkworkGenerationController(controllerProp, {
    locale,
    messages,
    service,
  });
  const state = useSdkworkGenerationControllerState(controller);
  const {
    copy,
    formatSortLabel,
    formatStatusLabel,
  } = useSdkworkGenerationIntl();
  const primaryHeroTextStyle = createSdkworkGenerationHeroTextStyle();
  const mutedHeroTextStyle = createSdkworkGenerationHeroTextStyle("muted");
  const subtleHeroTextStyle = createSdkworkGenerationHeroTextStyle("subtle");

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading) {
      void controller.bootstrap();
    }
  }, [controller, state.isBootstrapped, state.isLoading]);

  return (
    <div className="relative h-full overflow-y-auto">
      <div
        className="pointer-events-none absolute inset-x-0 top-0 h-72"
        style={createSdkworkGenerationBackdropStyle()}
      />

      <div className="relative px-4 py-4 sm:px-5 sm:py-5">
        <div className="mx-auto max-w-[96rem] space-y-5">
          <section
            className="overflow-hidden rounded-[2rem] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] px-6 py-7 text-white shadow-[var(--sdk-shadow-lg)]"
            style={createSdkworkGenerationHeroStyle()}
          >
            <div className="flex flex-col gap-5 lg:flex-row lg:items-end lg:justify-between">
              <div className="max-w-3xl">
                <div className="text-[0.7rem] font-semibold uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>
                  {copy.page.eyebrow}
                </div>
                <h1 className="mt-3 text-4xl font-semibold tracking-tight" style={primaryHeroTextStyle}>{copy.page.title}</h1>
                <p className="mt-3 max-w-2xl text-sm leading-7" style={mutedHeroTextStyle}>
                  {copy.page.description}
                </p>
              </div>
              <Button onClick={() => void controller.refresh()} type="button" variant="secondary">
                {copy.actions.refresh}
              </Button>
            </div>
          </section>

          {state.isLoading && !state.isBootstrapped ? <LoadingBlock label={copy.page.loading} /> : null}

          {state.lastError ? (
            <StatusNotice title={copy.page.errorTitle} tone="danger">
              {state.lastError}
            </StatusNotice>
          ) : null}

          <SdkworkGenerationRunSummary digest={state.workspace.digest} />

          <section
            className="space-y-4 rounded-[1.5rem] border p-5 shadow-[var(--sdk-shadow-soft)]"
            style={createSdkworkGenerationPanelStyle("neutral", {
              backgroundWeight: 8,
              borderWeight: 20,
            })}
          >
            <div className="flex flex-wrap gap-2">
              {statuses.map((status) => (
                <Button
                  key={status}
                  onClick={() => controller.setStatus(status)}
                  type="button"
                  variant={state.activeStatus === status ? "secondary" : "ghost"}
                >
                  {formatStatusLabel(status)}
                </Button>
              ))}
              <Button
                onClick={() => controller.setSortBy("recent")}
                type="button"
                variant={state.sortBy === "recent" ? "secondary" : "ghost"}
              >
                {formatSortLabel("recent")}
              </Button>
              <Button
                onClick={() => controller.setSortBy("latency")}
                type="button"
                variant={state.sortBy === "latency" ? "secondary" : "ghost"}
              >
                {formatSortLabel("latency")}
              </Button>
              <Button
                onClick={() => controller.setSortBy("alphabetical")}
                type="button"
                variant={state.sortBy === "alphabetical" ? "secondary" : "ghost"}
              >
                {formatSortLabel("alphabetical")}
              </Button>
            </div>

            <label className="block">
              <span className="sr-only">{copy.page.searchLabel}</span>
              <input
                className="w-full rounded-[1rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3.5 py-2.5 text-sm text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-sm)] outline-none ring-offset-[var(--sdk-color-surface-canvas)] placeholder:text-[var(--sdk-color-text-muted)] focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)]"
                onChange={(event) => controller.setSearchQuery(event.target.value)}
                placeholder={copy.page.searchPlaceholder}
                type="search"
                value={state.searchQuery}
              />
            </label>

            <div className="grid gap-5 xl:grid-cols-[24rem_minmax(0,1fr)]">
              <SdkworkGenerationRunList
                onSelectRun={(runId) => controller.selectRun(runId)}
                runs={state.visibleRuns}
                selectedRunId={state.selectedRunId}
              />
              <SdkworkGenerationRunDetail run={state.selectedRun} />
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}

export function SdkworkGenerationPage({
  locale,
  messages,
  ...props
}: SdkworkGenerationPageProps) {
  const content = (
    <SdkworkGenerationPageContent
      {...props}
      locale={locale}
      messages={messages}
    />
  );

  if (locale || messages) {
    return (
      <SdkworkGenerationIntlProvider locale={locale} messages={messages}>
        {content}
      </SdkworkGenerationIntlProvider>
    );
  }

  return content;
}
