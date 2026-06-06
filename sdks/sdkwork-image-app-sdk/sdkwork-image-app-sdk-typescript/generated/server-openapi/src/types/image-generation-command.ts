export interface ImageGenerationCommand {
  prompt: string;
  negativePrompt?: string | null;
  /** Business scene recorded on generated Drive files for filtering and lifecycle governance. */
  scene: string;
  providerCode?: string | null;
  model?: string | null;
  resolution?: string | null;
  style?: string | null;
  outputCount?: number | null;
  webhookUrl?: string;
  idempotencyKey?: string | null;
  input?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
}
