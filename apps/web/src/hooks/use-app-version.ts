"use client";

import { useEffect, useState } from "react";
import { formatAppVersion } from "@/lib/version";

export function useAppVersion(initialVersion?: string): string {
  const fallback = formatAppVersion(initialVersion);
  const [version, setVersion] = useState(fallback);

  useEffect(() => {
    let isMounted = true;

    fetch("/api/version")
      .then((res) => {
        if (!res.ok) throw new Error("Failed to fetch version");
        return res.json();
      })
      .then((data: { version?: string }) => {
        if (isMounted && data.version) {
          setVersion(formatAppVersion(data.version));
        }
      })
      .catch(() => {
        // Fallback is maintained on error
      });

    return () => {
      isMounted = false;
    };
  }, []);

  return version;
}
