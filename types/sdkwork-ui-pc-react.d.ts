import type { ButtonHTMLAttributes, HTMLAttributes, ReactNode } from "react";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "destructive" | "ghost" | "link" | "outline" | "secondary" | string;
}

export interface EmptyStateProps extends Omit<HTMLAttributes<HTMLDivElement>, "title"> {
  description?: ReactNode;
  title?: ReactNode;
}

export interface LoadingBlockProps extends HTMLAttributes<HTMLDivElement> {
  label?: ReactNode;
}

export interface StatusNoticeProps extends HTMLAttributes<HTMLDivElement> {
  title?: ReactNode;
  tone?: "danger" | "info" | "success" | "warning" | string;
}

export function EmptyState(props: EmptyStateProps): ReactNode;
export function Button(props: ButtonProps): ReactNode;
export function LoadingBlock(props: LoadingBlockProps): ReactNode;
export function StatusNotice(props: StatusNoticeProps): ReactNode;
