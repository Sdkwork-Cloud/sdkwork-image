import type { SdkworkImageDigest } from "../image";
import { createSdkworkImagePanelStyle } from "../image-appearance";
import { useSdkworkImageIntl } from "../image-intl";

export function SdkworkImageSummaryCards({ digest }: { digest: SdkworkImageDigest }) {
  const { copy, formatInteger } = useSdkworkImageIntl();

  const cards = [
    {
      id: "total",
      label: copy.summary.totalImages,
      tone: "brand" as const,
      value: digest.totalImages,
    },
    {
      id: "ready",
      label: copy.summary.readyImages,
      tone: "success" as const,
      value: digest.readyImages,
    },
    {
      id: "renders",
      label: copy.summary.activeRenders,
      tone: "accent" as const,
      value: digest.activeRenders,
    },
    {
      id: "presets",
      label: copy.summary.presets,
      tone: "neutral" as const,
      value: digest.presetCount,
    },
  ];

  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {cards.map((card) => (
        <article
          className="rounded-[1.25rem] border p-4 shadow-[var(--sdk-shadow-soft)]"
          key={card.id}
          style={createSdkworkImagePanelStyle(card.tone, {
            backgroundWeight: 8,
            borderWeight: 24,
          })}
        >
          <div className="text-sm text-[var(--sdk-color-text-secondary)]">{card.label}</div>
          <div className="mt-2 text-2xl font-semibold text-[var(--sdk-color-text-primary)]">{formatInteger(card.value)}</div>
        </article>
      ))}
    </div>
  );
}
