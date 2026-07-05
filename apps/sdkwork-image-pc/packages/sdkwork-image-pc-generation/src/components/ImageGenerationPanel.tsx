import { useEffect, useRef, useState } from 'react';
import { AlertCircle, Image as ImageIcon, Images, Sparkles, Upload, X } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { toExternalUrlMediaResource, type MediaResourceLike } from '@sdkwork/assets-pc-commons';
import {
  createDefaultSdkworkGenerationAssetConfig,
  estimateSdkworkGenerationCredits,
  findFirstSdkworkGenerationModelForModality,
  findSdkworkGenerationModelById,
  reconcileSdkworkGenerationAssetConfig,
  serializeSdkworkGenerationAssetConfig,
  updateSdkworkGenerationImageModeConfig,
  type SdkworkGenerationAssetConfig,
  type SdkworkGenerationCreditEstimate,
} from '../generation-asset-config';
import type { ImageGenerationPanelProps, ImageGenerationReferenceImageInput } from '../image-generation-panel-types';
import {
  IMAGE_REFERENCE_MODE_ORDER,
  resolveImageReferenceCapability,
  resolveImageReferenceModeUpload,
  type ImageReferenceCapability,
  type ImageReferenceMode,
} from '../image-reference-capability';
import { ImageGenerationModePopup } from './ImageGenerationModePopup';

interface ReferenceImagePreview {
  id: string;
  metadata: ImageGenerationReferenceImageInput;
  previewSrc: string;
}

export function ImageGenerationPanel({
  placeholderKey,
  modelGroups,
  selectedModelId,
  onSubmitGeneration,
  submitting,
  submitError,
}: ImageGenerationPanelProps) {
  const { t } = useTranslation();
  const [prompt, setPrompt] = useState('');
  const [imageReferenceMode, setImageReferenceMode] = useState<ImageReferenceMode>('text_to_image');
  const [referenceUploadError, setReferenceUploadError] = useState<string | null>(null);
  const [referenceImages, setReferenceImages] = useState<ReferenceImagePreview[]>([]);
  const [config, setConfig] = useState<SdkworkGenerationAssetConfig>(() => createDefaultSdkworkGenerationAssetConfig('image'));
  const referenceImageUrlsRef = useRef<string[]>([]);

  const selectedModel = findSdkworkGenerationModelById(modelGroups, selectedModelId)
    ?? findFirstSdkworkGenerationModelForModality(modelGroups, 'image');
  const imageReferenceCapability = resolveImageReferenceCapability(selectedModel);
  const activeImageReferenceMode = imageReferenceCapability.supportedModes.includes(imageReferenceMode)
    ? imageReferenceMode
    : imageReferenceCapability.supportedModes[0] ?? 'text_to_image';
  const activeImageReferenceUpload = resolveImageReferenceModeUpload(imageReferenceCapability, activeImageReferenceMode);
  const imageModeRequiresReference = activeImageReferenceMode === 'image_to_image'
    || activeImageReferenceMode === 'multi_reference';
  const normalizedPrompt = prompt.trim();
  const canSubmit = normalizedPrompt.length > 0
    && !submitting
    && Boolean(selectedModel)
    && (!imageModeRequiresReference || referenceImages.length > 0);
  const creditEstimate = estimateSdkworkGenerationCredits({
    config,
    modality: 'image',
    model: selectedModel,
    unavailableDetail: 'playground.generationCost.settlement',
  });

  useEffect(() => {
    setConfig((current) => reconcileSdkworkGenerationAssetConfig(current, 'image'));
  }, []);

  useEffect(() => () => {
    referenceImageUrlsRef.current.forEach((url) => URL.revokeObjectURL(url));
    referenceImageUrlsRef.current = [];
  }, []);

  useEffect(() => {
    setReferenceImages((current) => {
      const next = current.slice(0, activeImageReferenceUpload.maxFiles);
      if (next.length === current.length) {
        return current;
      }
      revokeRemovedReferenceImageUrls(current, next);
      referenceImageUrlsRef.current = next.map((item) => item.previewSrc);
      return next;
    });
    setReferenceUploadError(null);
  }, [activeImageReferenceUpload.maxFiles]);

  useEffect(() => {
    if (!imageReferenceCapability.supportedModes.includes(imageReferenceMode)) {
      setImageReferenceMode(imageReferenceCapability.supportedModes[0] ?? 'text_to_image');
    }
  }, [imageReferenceCapability.supportedModes, imageReferenceMode]);

  useEffect(() => {
    if (activeImageReferenceMode !== 'text_to_image') {
      return;
    }
    setReferenceImages((current) => {
      if (current.length === 0) {
        return current;
      }
      revokeRemovedReferenceImageUrls(current, []);
      referenceImageUrlsRef.current = [];
      return [];
    });
    setReferenceUploadError(null);
  }, [activeImageReferenceMode]);

  const replaceReferenceImages = (updater: (current: ReferenceImagePreview[]) => ReferenceImagePreview[]) => {
    setReferenceImages((current) => {
      const next = updater(current);
      revokeRemovedReferenceImageUrls(current, next);
      referenceImageUrlsRef.current = next.map((item) => item.previewSrc);
      return next;
    });
  };

  const handleSubmit = async () => {
    if (!canSubmit) {
      return;
    }

    await onSubmitGeneration({
      prompt: normalizedPrompt,
      selectedModality: 'image',
      targetType: 'image',
      selectedModel: selectedModel?.id || undefined,
      generationConfig: serializeSdkworkGenerationAssetConfig(config, 'image'),
      referenceImages: referenceImages.map((item) => item.metadata),
      referenceMode: activeImageReferenceMode,
    });

    setPrompt('');
    setReferenceUploadError(null);
    replaceReferenceImages(() => []);
  };

  return (
    <div className="sdkwork-image-generation-panel flex min-h-0 flex-1 flex-col overflow-hidden">
      <div className="sdkwork-image-generation-hero">
        <div className="sdkwork-image-generation-hero-icon" aria-hidden="true">
          <Sparkles className="h-4 w-4" />
        </div>
        <div className="min-w-0">
          <div className="sdkwork-image-generation-hero-title">{t('playground.image.studioTitle')}</div>
          <div className="sdkwork-image-generation-hero-subtitle">{t('playground.image.studioSubtitle')}</div>
        </div>
      </div>

      <div className="sdkwork-image-generation-panel__scroll custom-scrollbar min-h-0 flex-1 overflow-y-auto px-4 pb-4 pt-4">
        <div className="relative flex flex-col gap-3">
          {submitError ? (
            <div className="sdkwork-image-generation-error">
              <AlertCircle className="mt-0.5 h-4 w-4 shrink-0 text-red-400" />
              <span className="leading-relaxed">{submitError}</span>
            </div>
          ) : null}

          <ImageReferenceWorkspace
            activeMode={activeImageReferenceMode}
            capability={imageReferenceCapability}
            modeUpload={activeImageReferenceUpload}
            onAddReferenceImages={(next) => {
              replaceReferenceImages((current) => [
                ...current,
                ...next,
              ].slice(0, activeImageReferenceUpload.maxFiles));
            }}
            onChangeMode={(nextMode) => {
              setImageReferenceMode(nextMode);
              setReferenceUploadError(null);
            }}
            onRemoveReferenceImage={(id) => {
              replaceReferenceImages((current) => current.filter((item) => item.id !== id));
            }}
            onUploadError={setReferenceUploadError}
            referenceImages={referenceImages}
            uploadError={referenceUploadError}
          />

          <div className="sdkwork-image-generation-prompt group relative flex flex-col overflow-hidden transition-all duration-300">
            <div className="sdkwork-image-generation-prompt__header flex items-center justify-between px-5 py-2.5">
              <span className="sdkwork-image-generation-prompt__title">
                {t('playground.image.promptSection')}
              </span>
              <span className="sdkwork-image-generation-prompt__hint">{t('playground.image.promptHint')}</span>
            </div>
            <textarea
              value={prompt}
              onChange={(event) => setPrompt(event.target.value)}
              onKeyDown={(event) => {
                if (event.key === 'Enter' && !event.shiftKey) {
                  event.preventDefault();
                  void handleSubmit();
                }
              }}
              className="sdkwork-image-generation-prompt__textarea custom-scrollbar w-full resize-none bg-transparent px-5 pb-4 pt-1 outline-none"
              placeholder={t(placeholderKey)}
            />
            <div className="sdkwork-image-generation-prompt__footer flex items-center justify-between px-5 py-2.5">
              <span className="sdkwork-image-generation-prompt__keyboard-hint">
                <kbd className="sdkwork-image-generation-prompt__kbd">Enter</kbd>
                <span className="ml-1.5">{t('playground.promptKeyboard.submit')}</span>
                <kbd className="sdkwork-image-generation-prompt__kbd ml-3">Shift+Enter</kbd>
                <span className="ml-1.5">{t('playground.promptKeyboard.newline')}</span>
              </span>
              <span className={`sdkwork-image-generation-prompt__char-count ${normalizedPrompt.length > 800 ? 'sdkwork-image-generation-prompt__char-count--warn' : ''}`}>
                {normalizedPrompt.length}
              </span>
            </div>
          </div>
        </div>
      </div>

      <ImageGenerationBottomBar
        canSubmit={canSubmit}
        config={config}
        creditEstimate={creditEstimate}
        onChangeConfig={setConfig}
        onSubmit={handleSubmit}
        submitting={submitting}
      />
    </div>
  );
}

function ImageGenerationBottomBar({
  canSubmit,
  config,
  creditEstimate,
  onChangeConfig,
  onSubmit,
  submitting,
}: {
  canSubmit: boolean;
  config: SdkworkGenerationAssetConfig;
  creditEstimate: SdkworkGenerationCreditEstimate;
  onChangeConfig: (config: SdkworkGenerationAssetConfig) => void;
  onSubmit: () => Promise<void>;
  submitting: boolean;
}) {
  const { t } = useTranslation();

  return (
    <div className="z-30 shrink-0" title={creditEstimate.detail.startsWith('playground.') ? t(creditEstimate.detail) : creditEstimate.detail}>
      {config.imageMode ? (
        <ImageGenerationModePopup
          canGenerate={canSubmit}
          config={config.imageMode}
          isGenerating={submitting}
          onChangeConfig={(imageConfig) => onChangeConfig(updateSdkworkGenerationImageModeConfig(config, imageConfig))}
          onGenerate={onSubmit}
          showCost={creditEstimate.points ?? undefined}
        />
      ) : null}
    </div>
  );
}

function ImageReferenceWorkspace({
  activeMode,
  capability,
  modeUpload,
  referenceImages,
  uploadError,
  onAddReferenceImages,
  onChangeMode,
  onRemoveReferenceImage,
  onUploadError,
}: {
  activeMode: ImageReferenceMode;
  capability: ImageReferenceCapability;
  modeUpload: { maxFiles: number };
  referenceImages: ReferenceImagePreview[];
  uploadError: string | null;
  onAddReferenceImages: (items: ReferenceImagePreview[]) => void;
  onChangeMode: (mode: ImageReferenceMode) => void;
  onRemoveReferenceImage: (id: string) => void;
  onUploadError: (message: string | null) => void;
}) {
  const { t } = useTranslation();
  const availableModes = IMAGE_REFERENCE_MODE_ORDER.filter((item) => capability.supportedModes.includes(item));
  const showModeTabs = availableModes.length > 1;

  return (
    <div className="sdkwork-image-generation-reference">
      {showModeTabs ? (
        <ImageReferenceModeTabs activeMode={activeMode} modes={availableModes} onChangeMode={onChangeMode} />
      ) : null}

      {modeUpload.maxFiles > 0 ? (
        <ImageReferenceUploader
          activeMode={activeMode}
          modeUpload={modeUpload}
          onAddReferenceImages={onAddReferenceImages}
          onRemoveReferenceImage={onRemoveReferenceImage}
          onUploadError={onUploadError}
          referenceImages={referenceImages}
          showModeTabs={showModeTabs}
          uploadError={uploadError}
        />
      ) : (
        <div className={`${showModeTabs ? 'mt-3' : ''} sdkwork-image-generation-reference__mode-hint`}>
          {t(IMAGE_REFERENCE_MODE_DESCRIPTION_KEYS[activeMode])}
        </div>
      )}
    </div>
  );
}

function ImageReferenceModeTabs({
  activeMode,
  modes,
  onChangeMode,
}: {
  activeMode: ImageReferenceMode;
  modes: readonly ImageReferenceMode[];
  onChangeMode: (mode: ImageReferenceMode) => void;
}) {
  const { t } = useTranslation();

  return (
    <div
      className="sdkwork-image-generation-reference__tabs"
      role="tablist"
      aria-label={t('playground.imageReference.modeLabel')}
    >
      {modes.map((item) => {
        const selected = item === activeMode;
        const Icon = IMAGE_REFERENCE_MODE_ICONS[item];
        return (
          <button
            key={item}
            type="button"
            role="tab"
            aria-selected={selected}
            data-active={selected ? 'true' : 'false'}
            onClick={() => onChangeMode(item)}
            className="sdkwork-image-generation-reference__tab"
          >
            <Icon className="sdkwork-image-generation-reference__tab-icon h-3.5 w-3.5 shrink-0" aria-hidden="true" />
            <span className="whitespace-nowrap">{t(IMAGE_REFERENCE_MODE_LABEL_KEYS[item])}</span>
          </button>
        );
      })}
    </div>
  );
}

function ImageReferenceUploader({
  activeMode,
  modeUpload,
  referenceImages,
  uploadError,
  showModeTabs,
  onAddReferenceImages,
  onRemoveReferenceImage,
  onUploadError,
}: {
  activeMode: ImageReferenceMode;
  modeUpload: { maxFiles: number };
  referenceImages: ReferenceImagePreview[];
  uploadError: string | null;
  showModeTabs: boolean;
  onAddReferenceImages: (items: ReferenceImagePreview[]) => void;
  onRemoveReferenceImage: (id: string) => void;
  onUploadError: (message: string | null) => void;
}) {
  const { t } = useTranslation();
  const remainingSlots = Math.max(0, modeUpload.maxFiles - referenceImages.length);
  const canUpload = modeUpload.maxFiles > 0 && remainingSlots > 0;

  return (
    <div className={showModeTabs ? 'mt-3' : ''}>
      <div className="mb-3 flex items-center justify-between gap-3">
        <div className="min-w-0">
          <div className="sdkwork-image-generation-reference__heading">
            <Images className="sdkwork-image-generation-reference__heading-icon" />
            <span className="sdkwork-image-generation-reference__heading-title">{t('playground.referenceAssets')}</span>
          </div>
          <div className="sdkwork-image-generation-reference__count">
            <span className="tabular-nums">{referenceImages.length}</span>
            <span className="sdkwork-image-generation-reference__count-separator">/</span>
            <span className="tabular-nums">{modeUpload.maxFiles}</span>
          </div>
          {showModeTabs ? (
            <p className="sdkwork-image-generation-reference__mode-copy">
              {t(IMAGE_REFERENCE_MODE_DESCRIPTION_KEYS[activeMode])}
            </p>
          ) : null}
        </div>
        <label
          className={`sdkwork-image-generation-reference__upload-btn ${
            canUpload
              ? 'sdkwork-image-generation-reference__upload-btn--enabled'
              : 'sdkwork-image-generation-reference__upload-btn--disabled'
          }`}
        >
          <Upload className="h-3.5 w-3.5" />
          <span className="whitespace-nowrap">{t('playground.referenceImage.upload')}</span>
          <ReferenceImageFileInput
            disabled={!canUpload}
            maxFiles={modeUpload.maxFiles}
            onAddReferenceImages={onAddReferenceImages}
            onUploadError={onUploadError}
            remainingSlots={remainingSlots}
          />
        </label>
      </div>

      {referenceImages.length > 0 ? (
        <div className="grid grid-cols-2 gap-2.5 sm:grid-cols-3">
          {referenceImages.map((referenceImage) => (
            <div
              key={referenceImage.id}
              className="sdkwork-image-generation-reference__thumb group relative aspect-square overflow-hidden playground-image-canvas"
            >
              <img
                src={referenceImage.previewSrc}
                alt={referenceImage.metadata.name || t('playground.referenceAssets')}
                className="h-full w-full object-cover"
              />
              <div className="sdkwork-image-generation-reference__thumb-caption">
                <div className="truncate font-medium">{referenceImage.metadata.name || t('playground.referenceAssets')}</div>
              </div>
              <button
                type="button"
                onClick={() => onRemoveReferenceImage(referenceImage.id)}
                className="sdkwork-image-generation-reference__thumb-remove"
                title={t('playground.referenceImage.remove')}
              >
                <X className="h-4 w-4" />
              </button>
            </div>
          ))}
          {canUpload ? (
            <label className="sdkwork-image-generation-reference__add-tile">
              <ImageIcon className="h-5 w-5" />
              <span>{t('playground.referenceImage.upload')}</span>
              <ReferenceImageFileInput
                maxFiles={modeUpload.maxFiles}
                onAddReferenceImages={onAddReferenceImages}
                onUploadError={onUploadError}
                remainingSlots={remainingSlots}
              />
            </label>
          ) : null}
        </div>
      ) : (
        <div
          className="sdkwork-image-generation-reference__dropzone flex items-center justify-center text-center text-xs"
        >
          <label className="sdkwork-image-generation-reference__dropzone-label">
            <span className="sdkwork-image-generation-reference__dropzone-icon">
              <ImageIcon className="h-4 w-4" />
            </span>
            <span>{t('playground.referenceImage.upload')}</span>
            <span className="sdkwork-image-generation-reference__dropzone-hint">{t('playground.image.referenceDropHint')}</span>
            <ReferenceImageFileInput
              maxFiles={modeUpload.maxFiles}
              onAddReferenceImages={onAddReferenceImages}
              onUploadError={onUploadError}
              remainingSlots={remainingSlots}
            />
          </label>
        </div>
      )}

      {uploadError ? (
        <div className="sdkwork-image-generation-reference__error">
          <AlertCircle className="h-3.5 w-3.5 shrink-0" />
          <span>{uploadError}</span>
        </div>
      ) : null}
    </div>
  );
}

function ReferenceImageFileInput({
  disabled = false,
  maxFiles,
  remainingSlots,
  onAddReferenceImages,
  onUploadError,
}: {
  disabled?: boolean;
  maxFiles: number;
  remainingSlots: number;
  onAddReferenceImages: (items: ReferenceImagePreview[]) => void;
  onUploadError: (message: string | null) => void;
}) {
  const { t } = useTranslation();

  return (
    <input
      type="file"
      accept="image/*"
      multiple={maxFiles > 1}
      disabled={disabled}
      className="sr-only"
      onChange={(event) => {
        const selectedFiles = Array.from(event.currentTarget.files ?? []);
        const files = selectedFiles.slice(0, remainingSlots);
        if (selectedFiles.length > remainingSlots) {
          onUploadError(t('playground.referenceImage.tooMany', { max: maxFiles }));
        } else {
          onUploadError(null);
        }
        if (files.length > 0) {
          void Promise.all(files.map(async (file, index): Promise<ReferenceImagePreview> => ({
            id: createReferenceImagePreviewId(file, index),
            metadata: {
              name: file.name,
              mimeType: file.type,
              resource: createUploadedReferenceMediaResource(await readReferenceImageDataUrl(file), file.name, file.type, file.size),
              sizeBytes: file.size,
            },
            previewSrc: URL.createObjectURL(file),
          })))
            .then(onAddReferenceImages)
            .catch((error) => {
              const message = error instanceof Error && error.message !== 'playground.referenceImage.readFailed'
                ? error.message
                : t('playground.referenceImage.readFailed');
              onUploadError(message);
            });
        }
        event.currentTarget.value = '';
      }}
    />
  );
}

function revokeRemovedReferenceImageUrls(
  previous: readonly ReferenceImagePreview[],
  next: readonly ReferenceImagePreview[],
): void {
  const nextUrls = new Set(next.map((item) => item.previewSrc));
  previous.forEach((item) => {
    if (!nextUrls.has(item.previewSrc)) {
      URL.revokeObjectURL(item.previewSrc);
    }
  });
}

function createUploadedReferenceMediaResource(
  encodedReference: string,
  fileName: string,
  mimeType: string,
  sizeBytes: number,
): MediaResourceLike {
  const resource = toExternalUrlMediaResource(encodedReference, 'image');
  if (!resource) {
    throw new Error('playground.referenceImage.readFailed');
  }
  return {
    ...resource,
    fileName,
    mimeType: mimeType || undefined,
    sizeBytes: String(sizeBytes),
    title: fileName,
  };
}

function createReferenceImagePreviewId(file: File, index: number): string {
  const safeName = file.name.trim().replace(/[^a-zA-Z0-9._-]+/g, '-').replace(/^-+|-+$/g, '') || 'reference-image';
  return [safeName, file.size, file.lastModified, index, Math.random().toString(36).slice(2, 8)].join('-');
}

function readReferenceImageDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onerror = () => reject(new Error('playground.referenceImage.readFailed'));
    reader.onload = () => {
      if (typeof reader.result === 'string') {
        resolve(reader.result);
        return;
      }
      reject(new Error('playground.referenceImage.readFailed'));
    };
    reader.readAsDataURL(file);
  });
}

const IMAGE_REFERENCE_MODE_ICONS = {
  text_to_image: Sparkles,
  image_to_image: ImageIcon,
  multi_reference: Images,
} satisfies Record<ImageReferenceMode, typeof Sparkles>;

const IMAGE_REFERENCE_MODE_LABEL_KEYS = {
  text_to_image: 'playground.imageReference.mode.textToImage',
  image_to_image: 'playground.imageReference.mode.imageToImage',
  multi_reference: 'playground.imageReference.mode.multiReference',
} satisfies Record<ImageReferenceMode, string>;

const IMAGE_REFERENCE_MODE_DESCRIPTION_KEYS = {
  text_to_image: 'playground.imageReference.mode.textToImage.desc',
  image_to_image: 'playground.imageReference.mode.imageToImage.desc',
  multi_reference: 'playground.imageReference.mode.multiReference.desc',
} satisfies Record<ImageReferenceMode, string>;
