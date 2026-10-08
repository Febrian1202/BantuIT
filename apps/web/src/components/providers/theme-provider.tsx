"use client";

import { ComponentProps } from "react";
import { ThemeProvider as NextThemesProvider } from "next-themes";

export function ThemeProvide({
  children,
  ...props
}: ComponentProps<typeof NextThemesProvider>) {
  return <NextThemesProvider {...props}>{children}</NextThemesProvider>;
}
