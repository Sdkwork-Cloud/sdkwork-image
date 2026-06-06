import { useEffect } from "react";
import {
  LoadingBlock,
  StatusNotice,
} from "@sdkwork/ui-pc-react";
import { SdkworkImageGallery } from "../components/ImageGallery";
import { SdkworkImageSummaryCards } from "../components/ImageSummaryCards";
import type { SdkworkImageJobStatus } from "../image";
import {
  createSdkworkImageBackdropStyle,
  createSdkworkImageHeroStyle,
  createSdkworkImageHeroTextStyle,
  createSdkworkImagePanelStyle,
  createSdkworkImageToneStyle,
} from "../image-appearance";
import type { SdkworkImageMessagesOverrides } from "../image-copy";
import {
  useSdkworkImageController,
  useSdkworkImageControllerState,
} from "../image-controller";
import {
  SdkworkImageIntlProvider,
  useSdkworkImageIntl,
} from "../image-intl";
import type { SdkworkImageService } from "../image-service";

export interface SdkworkImagePageProps {
  controller?: ReturnType<typeof import("../image-controller").createSdkworkImageController>;
  locale?: string | null;
  messages?: SdkworkImageMessagesOverrides;
  service?: Partial<SdkworkImageService>;
}

interface SdkworkImagePageContentProps extends SdkworkImagePageProps {}

const statuses: Array<SdkworkImageJobStatus | "all"> = ["all", "ready", "rendering", "queued"];

function SdkworkImagePageContent({
  controller: controllerProp,
  locale,
  messages,
  service,
}: SdkworkImagePageContentProps) {
  const controller = useSdkworkImageController(controllerProp, {
    locale,
    messages,
    service,
  });
  const state = useSdkworkImageControllerState(controller);
  const {
    copy,
    formatStatusLabel,
  } = useSdkworkImageIntl();
  const primaryHeroTextStyle = createSdkworkImageHeroTextStyle();
  const mutedHeroTextStyle = createSdkworkImageHeroTextStyle("muted");
  const subtleHeroTextStyle = createSdkworkImageHeroTextStyle("subtle");

  useEffect(() => {
    if (!state.isBootstrapped && !state.isLoading) {
      void controller.bootstrap();
    }
  }, [controller, state.isBootstrapped, state.isLoading]);

  return (
    <div className="relative h-full overflow-y-auto">
      <div
        className="pointer-events-none absolute inset-x-0 top-0 h-72"
        style={createSdkworkImageBackdropStyle()}
      />

      <div className="relative px-4 py-4 sm:px-5 sm:py-5">
        <div className="mx-auto max-w-[96rem] space-y-5">
          <section
            className="rounded-[2rem] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] px-6 py-7 text-white shadow-[var(--sdk-shadow-lg)]"
            style={createSdkworkImageHeroStyle()}
          >
            <div className="text-[0.7rem] font-semibold uppercase tracking-[0.18em]" style={subtleHeroTextStyle}>
              {copy.page.eyebrow}
            </div>
            <h1 className="mt-3 text-4xl font-semibold tracking-tight" style={primaryHeroTextStyle}>{copy.page.title}</h1>
            <p className="mt-3 max-w-3xl text-sm leading-7" style={mutedHeroTextStyle}>
              {copy.page.description}
            </p>
          </section>

          {state.isLoading && !state.isBootstrapped ? <LoadingBlock label={copy.page.loading} /> : null}
          {state.lastError ? <StatusNotice title={copy.page.errorTitle} tone="danger">{state.lastError}</StatusNotice> : null}

          <SdkworkImageSummaryCards digest={state.workspace.digest} />

          <section
            className="rounded-[1.5rem] border p-5 shadow-[var(--sdk-shadow-soft)]"
            style={createSdkworkImagePanelStyle("neutral", {
              backgroundWeight: 8,
              borderWeight: 20,
            })}
          >
            <div className="flex flex-col gap-3 lg:flex-row lg:items-center">
              <input
                className="w-full rounded-[0.95rem] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-2 text-sm text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-sm)] outline-none ring-offset-[var(--sdk-color-surface-canvas)] placeholder:text-[var(--sdk-color-text-muted)] focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)]"
                onChange={(event) => controller.setSearchQuery(event.target.value)}
                placeholder={copy.page.searchPlaceholder}
                type="search"
                value={state.searchQuery}
              />
              <div className="flex flex-wrap gap-2">
                <button
                  className="rounded-full border px-3 py-1.5 text-xs font-semibold"
                  onClick={() => controller.setPreset("all")}
                  style={state.activePreset === "all"
                    ? createSdkworkImageToneStyle("brand", {
                      backgroundWeight: 18,
                      borderWeight: 30,
                    })
                    : undefined}
                  type="button"
                >
                  {copy.presets.all}
                </button>
                {state.workspace.presets.map((preset) => (
                  <button
                    className={`rounded-full border px-3 py-1.5 text-xs font-semibold ${
                      state.activePreset === preset.id
                        ? ""
                        : "border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-secondary)]"
                    }`}
                    key={preset.id}
                    onClick={() => controller.setPreset(preset.id)}
                    style={state.activePreset === preset.id
                      ? createSdkworkImageToneStyle("accent", {
                        backgroundWeight: 18,
                        borderWeight: 30,
                      })
                      : undefined}
                    type="button"
                  >
                    {preset.title}
                  </button>
                ))}
              </div>
            </div>

            <div className="mt-3 flex flex-wrap gap-2">
              {statuses.map((status) => (
                <button
                  className={`rounded-[0.85rem] border px-3 py-1.5 text-xs font-semibold ${
                    state.activeStatus === status
                      ? ""
                      : "border-[var(--sdk-color-border-default)] text-[var(--sdk-color-text-secondary)]"
                  }`}
                  key={status}
                  onClick={() => controller.setStatus(status as SdkworkImageJobStatus | "all")}
                  style={state.activeStatus === status
                    ? createSdkworkImageToneStyle(status === "ready" ? "success" : status === "queued" ? "warning" : "brand", {
                      backgroundWeight: 14,
                      borderWeight: 28,
                    })
                    : undefined}
                  type="button"
                >
                  {formatStatusLabel(status)}
                </button>
              ))}
            </div>

            <div className="mt-5">
              <SdkworkImageGallery images={state.visibleImages} />
            </div>
          </section>
        </div>
      </div>
    </div>
  );
}

export function SdkworkImagePage({
  locale,
  messages,
  ...props
}: SdkworkImagePageProps) {
  const content = (
    <SdkworkImagePageContent
      {...props}
      locale={locale}
      messages={messages}
    />
  );

  if (locale || messages) {
    return (
      <SdkworkImageIntlProvider locale={locale} messages={messages}>
        {content}
      </SdkworkImageIntlProvider>
    );
  }

  return content;
}
