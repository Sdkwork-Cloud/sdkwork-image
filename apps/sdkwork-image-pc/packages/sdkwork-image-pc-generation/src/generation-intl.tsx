import {
  createContext,
  useContext,
  useMemo,
  type PropsWithChildren,
} from "react";
import type {
  SdkworkGenerationSortBy,
  SdkworkGenerationStatus,
} from "./generation.ts";
import {
  createSdkworkGenerationMessages,
  formatSdkworkGenerationTemplate,
  normalizeSdkworkGenerationLocale,
  type SdkworkGenerationLocale,
  type SdkworkGenerationMessages,
  type SdkworkGenerationMessagesOverrides,
} from "./generation-copy.ts";

export interface SdkworkGenerationIntlValue {
  copy: SdkworkGenerationMessages;
  formatInteger: (value: number) => string;
  formatLatency: (value: number) => string;
  formatOpenRunLabel: (title: string) => string;
  formatSortLabel: (value: SdkworkGenerationSortBy) => string;
  formatStatusLabel: (value: SdkworkGenerationStatus | "all") => string;
  locale: SdkworkGenerationLocale;
}

export interface SdkworkGenerationIntlProviderProps extends PropsWithChildren {
  locale?: string | null;
  messages?: SdkworkGenerationMessagesOverrides;
}

function createSdkworkGenerationIntlValue(
  locale?: string | null,
  overrides?: SdkworkGenerationMessagesOverrides,
): SdkworkGenerationIntlValue {
  const resolvedLocale = normalizeSdkworkGenerationLocale(locale);
  const copy = createSdkworkGenerationMessages(resolvedLocale, overrides);
  const numberFormatter = new Intl.NumberFormat(resolvedLocale);

  return {
    copy,
    formatInteger(value) {
      return numberFormatter.format(value);
    },
    formatLatency(value) {
      return `${numberFormatter.format(value)} ms`;
    },
    formatOpenRunLabel(title) {
      return formatSdkworkGenerationTemplate(copy.list.openRunTemplate, {
        title,
      });
    },
    formatSortLabel(value) {
      return copy.sort[value];
    },
    formatStatusLabel(value) {
      return copy.status[value];
    },
    locale: resolvedLocale,
  };
}

const DEFAULT_SDKWORK_GENERATION_INTL = createSdkworkGenerationIntlValue();

const SdkworkGenerationIntlContext = createContext<SdkworkGenerationIntlValue>(
  DEFAULT_SDKWORK_GENERATION_INTL,
);

export function SdkworkGenerationIntlProvider({
  children,
  locale,
  messages,
}: SdkworkGenerationIntlProviderProps) {
  const value = useMemo(
    () => createSdkworkGenerationIntlValue(locale, messages),
    [locale, messages],
  );

  return (
    <SdkworkGenerationIntlContext.Provider value={value}>
      {children}
    </SdkworkGenerationIntlContext.Provider>
  );
}

export function useSdkworkGenerationIntl(): SdkworkGenerationIntlValue {
  return useContext(SdkworkGenerationIntlContext);
}
