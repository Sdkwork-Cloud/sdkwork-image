export interface ImageGenerationRetryCommand {
  retryProviderDispatch?: boolean | null;
  retryDriveImport?: boolean | null;
  reason?: string | null;
}
