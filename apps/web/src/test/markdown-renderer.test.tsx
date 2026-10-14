import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { MarkdownRenderer } from "@/components/shared/markdown-renderer";

describe("MarkdownRenderer XSS Sanitization & Accessibility", () => {
  it("sanitizes script tags from markdown content so no script element is executed", () => {
    const malicious = "Halo dunia! <script>alert('xss')</script>";
    const { container } = render(<MarkdownRenderer content={malicious} />);

    expect(screen.getByText(/halo dunia!/i)).toBeInTheDocument();
    // Script tag must not exist in DOM
    expect(container.querySelector("script")).toBeNull();
  });

  it("strips raw HTML elements with malicious attributes like onerror", () => {
    const malicious = 'Test gambar: <img src="invalid.png" onerror="alert(1)">';
    const { container } = render(<MarkdownRenderer content={malicious} />);

    const img = container.querySelector("img");
    // If rendered, onerror handler must never be present
    if (img) {
      expect(img.getAttribute("onerror")).toBeNull();
    } else {
      expect(img).toBeNull();
    }
  });

  it("sanitizes javascript: URLs by stripping the href attribute from links", () => {
    const malicious = "[Tautan Berbahaya](javascript:alert('pwned'))";
    const { container } = render(<MarkdownRenderer content={malicious} />);

    const link = container.querySelector("a");
    expect(link).toBeInTheDocument();
    // Dangerous href must be completely removed by rehype-sanitize
    expect(link?.getAttribute("href")).toBeNull();
  });

  it("renders safe formatting correctly (headings, bold, lists)", () => {
    const safeContent = `
# Panduan Reset Wi-Fi
Berikut langkah-langkah **penting**:
1. Matikan router
2. Tunggu 10 detik
3. Nyalakan kembali
    `;
    render(<MarkdownRenderer content={safeContent} />);

    expect(
      screen.getByRole("heading", { level: 1, name: /panduan reset wi-fi/i })
    ).toBeInTheDocument();
    expect(screen.getByText(/penting/i)).toBeInTheDocument();
    expect(screen.getByText(/matikan router/i)).toBeInTheDocument();
  });
});
