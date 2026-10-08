import type { Metadata } from "next";
import { Plus_Jakarta_Sans } from "next/font/google";
import "./globals.css";

const jakarta = Plus_Jakarta_Sans({
  subsets: ["latin"],
  variable: "--font-jakarta",
  weight: ["400", "500", "600"],
  display: "swap",
});

export const metadata: Metadata = {
  title: "JARVIS OPS — IT Service Management",
  description: "Platform manajemen tiket, aset, dan layanan IT perusahaan.",
  // icons: {
  //   icon: [
  //     { url: "/brand/logo.png", sizes: "32x32", type: "image/png" },
  //     { url: "/brand/logo.png", sizes: "512x512", type: "image/png" },
  //   ],
  //   apple: [{ url: "/brand/logo.png", sizes: "180x180", type: "image/png" }],
  // },
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="id" className={`${jakarta.variable} h-full antialiased`}>
      <body className="flex min-h-full flex-col">{children}</body>
    </html>
  );
}
