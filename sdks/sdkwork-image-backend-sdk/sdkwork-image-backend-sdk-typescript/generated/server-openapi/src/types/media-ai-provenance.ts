export interface MediaAiProvenance {
  provenance?: 'uploaded' | 'generated' | 'edited' | 'imported';
  provider?: string | null;
  model?: string | null;
  generationTaskId?: string | null;
  moderationStatus?: 'unknown' | 'pending' | 'approved' | 'rejected' | 'blocked' | null;
}
