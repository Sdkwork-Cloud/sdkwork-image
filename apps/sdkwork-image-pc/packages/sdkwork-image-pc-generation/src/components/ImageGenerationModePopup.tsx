import { Coins } from 'lucide-react';
import { useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import {
  DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG,
  type SdkworkGenerationImageModeConfig,
} from '../generation-asset-config';
import {
  SdkworkGenerationModePopupBase as GenerationModePopupBase,
  type SdkworkGenerationModeSection as ConfigSection,
  formatGenerationCreditPoints,
} from '@sdkwork/generations-pc-studio/react';

export type ImageGenerationConfig = SdkworkGenerationImageModeConfig;

function AspectRatioPreview({ ratio }: { ratio: string }) {
  const dimensions = resolveAspectRatioDimensions(ratio);
  return (
    <span
      className="inline-flex shrink-0 items-center justify-center rounded-[3px] border border-current/30 bg-current/10"
      style={{ width: dimensions.width, height: dimensions.height }}
      aria-hidden="true"
    />
  );
}

function resolveAspectRatioDimensions(ratio: string): { width: number; height: number } {
  if (ratio === 'auto') {
    return { width: 16, height: 14 };
  }
  const [widthValue, heightValue] = ratio.split(':').map((part) => Number(part));
  if (!Number.isFinite(widthValue) || !Number.isFinite(heightValue) || widthValue <= 0 || heightValue <= 0) {
    return { width: 16, height: 14 };
  }
  const max = 18;
  if (widthValue >= heightValue) {
    return { width: max, height: Math.max(8, Math.round((max * heightValue) / widthValue)) };
  }
  return { width: Math.max(8, Math.round((max * widthValue) / heightValue)), height: max };
}

interface ImageGenerationModePopupProps {
  config: ImageGenerationConfig;
  onChangeConfig: (config: ImageGenerationConfig) => void;
  onGenerate: () => void;
  isGenerating?: boolean;
  canGenerate?: boolean;
  showCost?: number;
}

export function ImageGenerationModePopup({
  canGenerate = true,
  config,
  isGenerating = false,
  onChangeConfig,
  onGenerate,
  showCost,
}: ImageGenerationModePopupProps) {
  const { i18n, t } = useTranslation();

  const sections = useMemo(() => [
    {
      id: 'quality',
      label: t('playground.imageSettings.quality'),
      type: 'select' as const,
      valueKey: 'quality',
      options: [
        { value: '1k', label: t('playground.imageSettings.quality1k') },
        { value: '2k', label: t('playground.imageSettings.quality2k'), isVip: true },
      ],
    },
    {
      id: 'aspectRatio',
      label: t('playground.imageSettings.aspectRatio'),
      type: 'select' as const,
      valueKey: 'aspectRatio',
      options: [
        { value: 'auto', label: t('playground.imageSettings.ratioAuto'), icon: <AspectRatioPreview ratio="auto" /> },
        { value: '9:16', label: '9:16', icon: <AspectRatioPreview ratio="9:16" /> },
        { value: '2:3', label: '2:3', icon: <AspectRatioPreview ratio="2:3" /> },
        { value: '3:4', label: '3:4', icon: <AspectRatioPreview ratio="3:4" /> },
        { value: '1:1', label: '1:1', icon: <AspectRatioPreview ratio="1:1" /> },
        { value: '4:3', label: '4:3', icon: <AspectRatioPreview ratio="4:3" /> },
        { value: '3:2', label: '3:2', icon: <AspectRatioPreview ratio="3:2" /> },
        { value: '16:9', label: '16:9', icon: <AspectRatioPreview ratio="16:9" /> },
        { value: '21:9', label: '21:9', icon: <AspectRatioPreview ratio="21:9" /> },
      ],
    },
    {
      id: 'count',
      label: t('playground.imageSettings.count'),
      type: 'select' as const,
      valueKey: 'count',
      options: [
        { value: 1, label: '1' },
        { value: 2, label: '2' },
        { value: 3, label: '3' },
        { value: 4, label: '4' },
        { value: 5, label: '5', isVip: true },
        { value: 6, label: '6', isVip: true },
        { value: 7, label: '7', isVip: true },
        { value: 8, label: '8', isVip: true },
        { value: 9, label: '9', isVip: true },
      ],
    },
  ] satisfies ConfigSection<ImageGenerationConfig>[], [t]);

  const getSummary = (current: ImageGenerationConfig) => {
    const qualityLabel = current.quality === '2k'
      ? t('playground.imageSettings.quality2kShort')
      : t('playground.imageSettings.quality1kShort');
    const ratioLabel = current.aspectRatio === 'auto'
      ? t('playground.imageSettings.ratioAutoShort')
      : current.aspectRatio;
    return `${qualityLabel} · ${ratioLabel} · ${current.count}`;
  };

  return (
    <GenerationModePopupBase
      canGenerate={canGenerate}
      config={config}
      generateLabel={t('playground.generate')}
      generatingLabel={t('playground.imageSettings.generating')}
      getSummary={getSummary}
      isGenerating={isGenerating}
      onChangeConfig={onChangeConfig}
      onGenerate={onGenerate}
      sections={sections}
      title={t('playground.imageSettings.title')}
      barClassName="sdkwork-image-generation-bottom-bar"
      popupClassName="sdkwork-image-generation-settings-popup"
      renderExtraControls={() => (
        showCost !== undefined && (
          <div className="sdkwork-image-generation-cost">
            <Coins className="h-3.5 w-3.5" />
            <span className="font-bold">{formatGenerationCreditPoints(showCost, i18n.language)}</span>
          </div>
        )
      )}
    />
  );
}

export const DEFAULT_IMAGE_GENERATION_CONFIG: ImageGenerationConfig = {
  ...DEFAULT_SDKWORK_GENERATION_IMAGE_MODE_CONFIG,
};
