import { GlobalProvider } from "@/contexts/GlobalContext";
import "./globals.css";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "UniFi Voucher Manager",
  description: "Manage WiFi vouchers with ease",
  authors: [{ name: "etiennecollin", url: "https://etiennecollin.com" }],
  creator: "Etienne Collin",
  robots: {
    index: false,
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        {/* Prevent flash of unstyled content by setting theme immediately */}
        <script
          dangerouslySetInnerHTML={{
            __html: `
              (function() {
                function getTheme() {
                  const stored = localStorage.getItem('theme');
                  if (stored === 'dark') return 'dark';
                  if (stored === 'light') return 'light';
                  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
                }
                const theme = getTheme();
                document.documentElement.classList.toggle('dark', theme === 'dark');
                document.documentElement.style.backgroundColor = theme === 'dark' ? '#171717' : '#fafafa';
              })();
            `,
          }}
        />
        {/* Load runtime config */}
        <script src="/runtime-config.js"></script>
      </head>
      <body className={`antialiased`}>
        <GlobalProvider>{children}</GlobalProvider>
      </body>
    </html>
  );
}
