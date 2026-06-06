import { EmptyState } from "@sdkwork/ui-pc-react";
import type { SdkworkImageAsset } from "../image";
import {
  createSdkworkImagePanelStyle,
  createSdkworkImageToneStyle,
  type SdkworkImageVisualTone,
} from "../image-appearance";
import { useSdkworkImageIntl } from "../image-intl";

export function SdkworkImageGallery({ images }: { images: readonly SdkworkImageAsset[] }) {
  const {
    copy,
    formatStatusLabel,
  } = useSdkworkImageIntl();

  if (images.length === 0) {
    return (
      <EmptyState
        description={copy.empty.noImagesDescription}
        title={copy.empty.noImagesTitle}
      />
    );
  }

  return (
    <div className="grid gap-4 xl:grid-cols-2">
      {images.map((image) => {
        const tone: SdkworkImageVisualTone = image.status === "ready"
          ? "success"
          : image.status === "queued"
            ? "warning"
            : "brand";

        return (
          <article
            className="rounded-[1.2rem] border p-4 shadow-[var(--sdk-shadow-soft)]"
            key={image.id}
            style={createSdkworkImagePanelStyle("neutral", {
              backgroundWeight: 8,
              borderWeight: 20,
            })}
          >
            <div className="flex items-center justify-between gap-3">
              <h3 className="text-base font-semibold text-[var(--sdk-color-text-primary)]">{image.title}</h3>
              <span
                className="rounded-full border px-2.5 py-1 text-xs font-semibold uppercase"
                style={createSdkworkImageToneStyle(tone, {
                  backgroundWeight: 14,
                  borderWeight: 26,
                })}
              >
                {formatStatusLabel(image.status)}
              </span>
            </div>
            <p className="mt-2 text-sm text-[var(--sdk-color-text-secondary)]">{image.style} | {image.resolution}</p>
          </article>
        );
      })}
    </div>
  );
}
