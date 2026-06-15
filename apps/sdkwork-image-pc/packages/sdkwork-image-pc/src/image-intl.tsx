import {
  createContext,
  useContext,
  useMemo,
  type PropsWithChildren,
} from "react";
import type { SdkworkImageJobStatus } from "./image";
import {
  createSdkworkImageMessages,
  normalizeSdkworkImageLocale,
  type SdkworkImageLocale,
  type SdkworkImageMessages,
  type SdkworkImageMessagesOverrides,
} from "./image-copy";

export interface SdkworkImageIntlValue {
  copy: SdkworkImageMessages;
  formatInteger: (value: number) => string;
  formatStatusLabel: (value: SdkworkImageJobStatus | "all") => string;
  locale: SdkworkImageLocale;
}

export interface SdkworkImageIntlProviderProps extends PropsWithChildren {
  locale?: string | null;
  messages?: SdkworkImageMessagesOverrides;
}

function createSdkworkImageIntlValue(
  locale?: string | null,
  overrides?: SdkworkImageMessagesOverrides,
): SdkworkImageIntlValue {
  const resolvedLocale = normalizeSdkworkImageLocale(locale);
  const copy = createSdkworkImageMessages(resolvedLocale, overrides);
  const numberFormatter = new Intl.NumberFormat(resolvedLocale);

  return {
    copy,
    formatInteger(value) {
      return numberFormatter.format(value);
    },
    formatStatusLabel(value) {
      return copy.status[value];
    },
    locale: resolvedLocale,
  };
}

const DEFAULT_SDKWORK_IMAGE_INTL = createSdkworkImageIntlValue();

const SdkworkImageIntlContext = createContext<SdkworkImageIntlValue>(
  DEFAULT_SDKWORK_IMAGE_INTL,
);

export function SdkworkImageIntlProvider({
  children,
  locale,
  messages,
}: SdkworkImageIntlProviderProps) {
  const value = useMemo(
    () => createSdkworkImageIntlValue(locale, messages),
    [locale, messages],
  );

  return (
    <SdkworkImageIntlContext.Provider value={value}>
      {children}
    </SdkworkImageIntlContext.Provider>
  );
}

export function useSdkworkImageIntl(): SdkworkImageIntlValue {
  return useContext(SdkworkImageIntlContext);
}
