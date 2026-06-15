import type { ReactNode } from "react";

export interface AuthGateProps {
  children: ReactNode;
}

export function AuthGate({ children }: AuthGateProps) {
  return <>{children}</>;
}
