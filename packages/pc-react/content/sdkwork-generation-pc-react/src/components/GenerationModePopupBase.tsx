import {
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { Check, ChevronUp, Settings, X } from "lucide-react";

type ConfigValue = boolean | number | string;

export interface SdkworkGenerationModeOption<T extends ConfigValue = string> {
  icon?: ReactNode;
  isVip?: boolean;
  label: string;
  value: T;
}

export interface SdkworkGenerationModeSection<Config extends object = Record<string, ConfigValue>> {
  id: string;
  label: string;
  max?: number;
  min?: number;
  options?: SdkworkGenerationModeOption<ConfigValue>[];
  step?: number;
  type: "select" | "slider" | "toggle";
  unit?: string;
  valueKey: keyof Config & string;
}

export interface SdkworkGenerationModePopupBaseProps<Config extends object> {
  canGenerate?: boolean;
  config: Config;
  generateLabel?: string;
  generatingLabel?: string;
  getSummary: (config: Config) => string;
  isGenerating?: boolean;
  onChangeConfig: (config: Config) => void;
  onGenerate: () => void;
  renderExtraControls?: () => ReactNode;
  sections: SdkworkGenerationModeSection<Config>[];
  title?: string;
}

export function SdkworkGenerationModePopupBase<Config extends object>({
  canGenerate = true,
  config,
  generateLabel = "Generate",
  generatingLabel = "Generating...",
  getSummary,
  isGenerating = false,
  onChangeConfig,
  onGenerate,
  renderExtraControls,
  sections,
  title = "Generation settings",
}: SdkworkGenerationModePopupBaseProps<Config>) {
  const [isOpen, setIsOpen] = useState(false);
  const popupRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isOpen) {
      return undefined;
    }

    const handleClickOutside = (event: MouseEvent) => {
      if (popupRef.current && !popupRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [isOpen]);

  const handleSectionChange = (valueKey: keyof Config & string, value: ConfigValue) => {
    onChangeConfig({ ...config, [valueKey]: value } as Config);
  };

  const summary = getSummary(config);

  return (
    <div className="relative" ref={popupRef}>
      <div className="flex items-center justify-between gap-4 border-t border-white/10 bg-[#151515]/95 px-4 py-3 backdrop-blur">
        <button
          className="-mx-2 flex items-center gap-3 rounded-lg px-2 py-1.5 transition-colors hover:bg-white/5"
          onClick={() => setIsOpen(!isOpen)}
          type="button"
        >
          <Settings className={`h-5 w-5 text-gray-400 transition-transform ${isOpen ? "rotate-90" : ""}`} />
          <span className="text-sm font-medium text-gray-300">{summary}</span>
          <ChevronUp className={`h-4 w-4 text-gray-500 transition-transform duration-300 ${isOpen ? "" : "rotate-180"}`} />
        </button>

        <div className="flex items-center gap-3">
          {renderExtraControls?.()}

          <button
            className={`rounded-lg px-8 py-2.5 text-base font-bold transition-all ${
              canGenerate && !isGenerating
                ? "bg-gradient-to-r from-cyan-400 to-blue-500 text-white shadow-lg shadow-cyan-400/30 hover:from-cyan-500 hover:to-blue-600"
                : "cursor-not-allowed bg-gray-700 text-gray-500"
            }`}
            disabled={!canGenerate || isGenerating}
            onClick={(event) => {
              event.stopPropagation();
              if (canGenerate && !isGenerating) {
                onGenerate();
              }
            }}
            type="button"
          >
            {isGenerating ? generatingLabel : generateLabel}
          </button>
        </div>
      </div>

      {isOpen && (
        <div
          className="absolute bottom-full left-0 right-0 mb-2 rounded-xl border border-white/10 bg-[#1a1a1a] shadow-[0_-8px_32px_rgba(0,0,0,0.6)]"
          style={{ animation: "sdkworkGenerationSlideUp 0.2s ease-out" }}
        >
          <div className="flex items-center justify-between border-b border-white/5 px-6 py-4">
            <h3 className="text-sm font-semibold text-white">{title}</h3>
            <button
              className="rounded p-1 text-gray-400 transition-colors hover:bg-white/10 hover:text-white"
              onClick={() => setIsOpen(false)}
              type="button"
            >
              <X className="h-4 w-4" />
            </button>
          </div>

          <div className="custom-scrollbar max-h-[60vh] space-y-6 overflow-y-auto px-6 pb-6 pt-6">
            {sections.map((section) => (
              <SdkworkGenerationConfigSectionRenderer
                key={section.id}
                onChange={(value) => handleSectionChange(section.valueKey, value)}
                section={section}
                value={readConfigValue(config, section.valueKey)}
              />
            ))}
          </div>
        </div>
      )}

      <style>{`
        @keyframes sdkworkGenerationSlideUp {
          from { opacity: 0; transform: translateY(10px); }
          to { opacity: 1; transform: translateY(0); }
        }

        .custom-scrollbar::-webkit-scrollbar { width: 6px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.1); border-radius: 3px; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255,255,255,0.2); }

        input[type='range']::-webkit-slider-thumb {
          -webkit-appearance: none;
          width: 16px; height: 16px;
          border-radius: 50%;
          background: white;
          cursor: pointer;
          border: 2px solid #06b6d4;
          box-shadow: 0 2px 8px rgba(0,0,0,0.3);
        }

        input[type='range']::-moz-range-thumb {
          width: 16px; height: 16px;
          border-radius: 50%;
          background: white;
          cursor: pointer;
          border: 2px solid #06b6d4;
          box-shadow: 0 2px 8px rgba(0,0,0,0.3);
        }
      `}</style>
    </div>
  );
}

function SdkworkGenerationConfigSectionRenderer<Config extends object>({
  onChange,
  section,
  value,
}: {
  onChange: (value: ConfigValue) => void;
  section: SdkworkGenerationModeSection<Config>;
  value: ConfigValue;
}) {
  if (section.type === "select" && section.options) {
    return (
      <div className="space-y-3">
        <label className="text-sm font-medium text-gray-400">{section.label}</label>
        <div className={`grid gap-3 ${gridClassForOptionCount(section.options.length)}`}>
          {section.options.map((option) => (
            <button
              className={`relative flex items-center justify-center gap-2 rounded-lg border px-4 py-3 text-base font-semibold transition-all ${
                value === option.value
                  ? "border-white/30 bg-white/10 text-white shadow-lg"
                  : option.isVip
                    ? "cursor-not-allowed border-white/5 bg-[#252525] text-gray-500"
                    : "border-white/5 bg-[#252525] text-gray-300 hover:bg-white/5 hover:text-white"
              }`}
              disabled={option.isVip}
              key={String(option.value)}
              onClick={() => onChange(option.value)}
              type="button"
            >
              {option.icon}
              <span>{option.label}</span>
              {option.isVip && (
                <span className="absolute right-1.5 top-1.5 rounded border border-yellow-500/30 bg-yellow-500/20 px-1.5 py-0.5 text-[10px] font-bold text-yellow-400">
                  VIP
                </span>
              )}
            </button>
          ))}
        </div>
      </div>
    );
  }

  if (section.type === "slider") {
    const numberValue = Number(value);
    const min = section.min ?? 0;
    const max = section.max ?? 100;
    const range = max - min || 1;

    return (
      <div className="space-y-3">
        <div className="flex items-center justify-between">
          <label className="text-sm font-medium text-gray-400">{section.label}</label>
          <span className="font-mono text-sm text-gray-300">{numberValue}{section.unit}</span>
        </div>
        <div className="flex items-center gap-4">
          <span className="w-8 text-xs text-gray-500">{min}{section.unit}</span>
          <div className="relative flex-1">
            <input
              className="h-1.5 w-full cursor-pointer appearance-none rounded-full bg-gray-700"
              max={max}
              min={min}
              onChange={(event) => onChange(Number(event.target.value))}
              step={section.step ?? 1}
              style={{
                background: `linear-gradient(to right, #06b6d4 0%, #06b6d4 ${((numberValue - min) / range) * 100}%, #374151 ${((numberValue - min) / range) * 100}%, #374151 100%)`,
              }}
              type="range"
              value={numberValue}
            />
          </div>
          <span className="w-8 text-xs text-gray-500">{max}{section.unit}</span>
        </div>
      </div>
    );
  }

  if (section.type === "toggle") {
    return (
      <button
        className={`flex items-center gap-2 rounded-lg border px-3 py-1.5 text-xs font-medium transition-all ${
          value
            ? "border-white/20 bg-white/10 text-white"
            : "border-white/5 bg-transparent text-gray-500 hover:border-white/10"
        }`}
        onClick={() => onChange(!value)}
        type="button"
      >
        <Check className={`h-3.5 w-3.5 transition-opacity ${value ? "text-green-400 opacity-100" : "opacity-0"}`} />
        {section.label}
      </button>
    );
  }

  return null;
}

function gridClassForOptionCount(optionCount: number): string {
  if (optionCount <= 1) {
    return "grid-cols-1";
  }
  if (optionCount === 2) {
    return "grid-cols-2";
  }
  if (optionCount === 3) {
    return "grid-cols-3";
  }
  if (optionCount <= 6) {
    return "grid-cols-3";
  }
  return "grid-cols-4";
}

function readConfigValue<Config extends object>(
  config: Config,
  key: keyof Config & string,
): ConfigValue {
  const value = config[key as keyof Config];
  if (typeof value === "string" || typeof value === "number" || typeof value === "boolean") {
    return value;
  }
  return "";
}
