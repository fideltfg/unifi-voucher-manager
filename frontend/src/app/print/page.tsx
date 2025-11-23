"use client";

import "./styles.css";
import { useRouter, useSearchParams } from "next/navigation";
import { Suspense, useEffect, useRef, useState } from "react";
import { QRCodeSVG } from "qrcode.react";
import { Voucher } from "@/types/voucher";
import {
  formatBytes,
  formatDuration,
  formatMaxGuests,
  formatSpeed,
} from "@/utils/format";
import { useGlobal } from "@/contexts/GlobalContext";
import { formatCode } from "@/utils/format";
import Spinner from "@/components/utils/Spinner";

export type PrintMode = "list" | "grid";

interface PrintConfig {
  logo: {
    enabled: boolean;
    path: string;
    width: number;
    height: number;
  };
  header: {
    title: string;
    subtitle: string;
  };
  footer: {
    customText: string;
    showVoucherId: boolean;
    showPrintedTime: boolean;
  };
  qrCode: {
    size: number;
  };
  layout: {
    order: string[];
  };
  additionalInfo: {
    enabled: boolean;
    fields: Array<{
      label: string;
      value: string;
    }>;
  };
}

// This component represents a single voucher card to be printed
function VoucherPrintCard({ voucher, printConfig }: { voucher: Voucher, printConfig: PrintConfig | null }) {
  const { wifiConfig, wifiString } = useGlobal();

  const fields = [
    {
      label: "Duration",
      value: formatDuration(voucher.timeLimitMinutes),
    },
    {
      label: "Max Guests",
      value: formatMaxGuests(voucher.authorizedGuestLimit),
    },
    {
      label: "Data Limit",
      value: voucher.dataUsageLimitMBytes
        ? formatBytes(voucher.dataUsageLimitMBytes * 1024 * 1024)
        : "Unlimited",
    },
    {
      label: "Down Speed",
      value: formatSpeed(voucher.rxRateLimitKbps),
    },
    {
      label: "Up Speed",
      value: formatSpeed(voucher.txRateLimitKbps),
    },
  ];

  const renderSection = (section: string) => {
    switch (section) {
      case 'logo':
        return printConfig?.logo.enabled && printConfig.logo.path ? (
          <div key="logo" className="print-logo">
            <img 
              src={printConfig.logo.path} 
              alt="Logo" 
              width={printConfig.logo.width}
              height={printConfig.logo.height}
              style={{ 
                width: `${printConfig.logo.width}px`, 
                height: `${printConfig.logo.height}px`,
                objectFit: 'contain',
                display: 'block',
                margin: '0 auto'
              }}
              onError={(e: any) => {
                console.error('Logo failed to load:', printConfig.logo.path);
                e.currentTarget.style.display = 'none';
              }}
            />
          </div>
        ) : null;

      case 'header':
        return (
          <div key="header" className="print-header">
            <div className="print-title">{printConfig?.header.title || "WiFi Access Voucher"}</div>
            {printConfig?.header.subtitle && (
              <div className="print-subtitle">{printConfig.header.subtitle}</div>
            )}
          </div>
        );

      case 'code':
        return (
          <div key="code" className="print-voucher-code">{formatCode(voucher.code)}</div>
        );

      case 'details':
        return (
          <div key="details">
            {fields.map((field, index) => (
              <div key={index} className="print-info-row">
                <span className="print-label">{field.label}:</span>
                <span className="print-value">{field.value}</span>
              </div>
            ))}
          </div>
        );

      case 'qr':
        return wifiConfig ? (
          <div key="qr" className="print-qr-section">
            {wifiString && (
              <>
                <div className="font-bold mb-2">Scan to Connect</div>
                <QRCodeSVG
                  value={wifiString}
                  size={printConfig?.qrCode?.size || 180}
                  level="H"
                  marginSize={4}
                  title="Wi-Fi Access QR Code"
                />
              </>
            )}
            <div className="print-qr-text">
              <strong>Network:</strong> {wifiConfig.ssid}
              <br />
              {wifiConfig.type === "nopass" ? (
                "No Password"
              ) : (
                <>
                  <strong>Password:</strong> {wifiConfig.password}
                </>
              )}
              {wifiConfig.hidden && <div>(Hidden Network)</div>}
            </div>
          </div>
        ) : null;

      case 'additionalInfo':
        return printConfig?.additionalInfo.enabled && printConfig.additionalInfo.fields.length > 0 ? (
          <div key="additionalInfo" className="print-additional-info">
            {printConfig.additionalInfo.fields.map((field, index) => (
              <div key={index} className="print-tos-item">
                <div className="print-tos-label">{field.label}</div>
                <div className="print-tos-value" style={{ whiteSpace: 'pre-line' }}>{field.value}</div>
              </div>
            ))}
          </div>
        ) : null;

      case 'footer':
        return (
          <div key="footer" className="print-footer">
            {printConfig?.footer.customText && (
              <div className="print-custom-text">{printConfig.footer.customText}</div>
            )}
            {printConfig?.footer.showVoucherId && (
              <div>
                <strong className="text-sm">ID:</strong> {voucher.id}
              </div>
            )}
            {printConfig?.footer.showPrintedTime && (
              <div>
                <strong className="text-sm">Printed:</strong>{" "}
                {new Date().toUTCString()}
              </div>
            )}
          </div>
        );

      default:
        return null;
    }
  };

  const layoutOrder = printConfig?.layout?.order || ['logo', 'header', 'code', 'details', 'qr', 'additionalInfo', 'footer'];

  return (
    <div className="print-voucher">
      {layoutOrder.map(section => renderSection(section))}
    </div>
  );
}

// This component handles displaying and printing the vouchers based on URL params
function Vouchers() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [vouchers, setVouchers] = useState<Voucher[]>([]);
  const [mode, setMode] = useState<PrintMode>("list");
  const [printConfig, setPrintConfig] = useState<PrintConfig | null>(null);
  const lastSearchParams = useRef<string | null>(null);

  // Load print configuration
  useEffect(() => {
    fetch('/print-config.json')
      .then(res => res.json())
      .then(config => setPrintConfig(config))
      .catch(err => console.error('Failed to load print config:', err));
  }, []);

  useEffect(() => {
    const paramString = searchParams.toString();
    if (lastSearchParams.current === paramString) {
      return;
    }
    lastSearchParams.current = paramString;

    const vouchersParam = searchParams.get("vouchers");
    const modeParam = searchParams.get("mode");

    if (!vouchersParam || !modeParam) {
      return;
    }

    try {
      const parsedVouchers = JSON.parse(decodeURIComponent(vouchersParam));
      setVouchers(parsedVouchers);
      setMode(modeParam as PrintMode);

      setTimeout(() => {
        window.print();
        router.replace("/");
      }, 100);
    } catch (error) {
      console.error("Failed to parse vouchers:", error);
    }
  }, [searchParams, router]);

  return !vouchers.length ? (
    <div style={{ textAlign: "center" }}>
      No vouchers to print, press backspace
    </div>
  ) : (
    <div className={mode === "grid" ? "print-grid" : "print-list"}>
      {vouchers.map((v) => (
        <VoucherPrintCard key={v.id} voucher={v} printConfig={printConfig} />
      ))}
    </div>
  );
}

// This sets up the print page itself
export default function PrintPage() {
  const router = useRouter();

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape" || e.key === "Backspace") router.replace("/");
    };
    window.addEventListener("keydown", onKey);

    return () => {
      window.removeEventListener("keydown", onKey);
    };
  }, [router]);

  return (
    <main className="print-wrapper">
      <Suspense fallback={<Spinner />}>
        <Vouchers />
      </Suspense>
    </main>
  );
}
