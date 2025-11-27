"use client";

import { useCallback, useEffect, useState, useRef } from "react";
import Spinner from "@/components/utils/Spinner";
import WifiQr from "@/components/utils/WifiQr";
import { TriState } from "@/types/state";
import { Voucher } from "@/types/voucher";
import { api } from "@/utils/api";
import { formatCode } from "@/utils/format";
import { useGlobal } from "@/contexts/GlobalContext";

export default function KioskPage() {
  const [voucher, setVoucher] = useState<Voucher | null>(null);
  const [state, setState] = useState<TriState | null>(null);
  const [kioskIndex, setKioskIndex] = useState<number | null>(null);
  const [countdown, setCountdown] = useState<number>(10);
  const { wifiConfig, wifiString } = useGlobal();
  const loadingRef = useRef(false);

  // Get kiosk index from URL parameter or localStorage
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const urlIndex = params.get("index");

    if (urlIndex !== null) {
      const index = parseInt(urlIndex, 10);
      if (!isNaN(index) && index >= 0) {
        setKioskIndex(index);
        localStorage.setItem("kioskIndex", index.toString());
        return;
      }
    }

    // Try to get from localStorage
    const storedIndex = localStorage.getItem("kioskIndex");
    if (storedIndex !== null) {
      const index = parseInt(storedIndex, 10);
      if (!isNaN(index) && index >= 0) {
        setKioskIndex(index);
        return;
      }
    }

    // Default to index 0
    setKioskIndex(0);
  }, []);

  const load = useCallback(async () => {
    if (loadingRef.current || kioskIndex === null) return;

    loadingRef.current = true;
    try {
      setState("loading");
      await api.getRollingVoucher(kioskIndex).then(setVoucher);
      setState("ok");
    } catch (error: any) {
      if (error?.status !== 404) {
        setState("error");
        return;
      }
      // No voucher found at this index, try to create more
      try {
        console.log(`Kiosk ${kioskIndex}: No voucher found, requesting rotation...`);
        await api.rotateRollingVoucherIfNeeded();
        // Try loading again after creating new vouchers
        await api.getRollingVoucher(kioskIndex).then(setVoucher);
        setState("ok");
      } catch (retryError) {
        console.error(`Kiosk ${kioskIndex}: Failed to load voucher after rotation:`, retryError);
        setState("error");
      }
    } finally {
      loadingRef.current = false;
    }
  }, [kioskIndex]);

  // Check for voucher usage and rotate if needed
  const checkAndRotate = useCallback(async () => {
    if (!voucher || state !== "ok" || kioskIndex === null) return;

    try {
      // Get fresh voucher data to check if it's been used
      const currentVoucher = await api.getRollingVoucher(kioskIndex);

      // If no voucher is returned or if the current voucher ID changed, we need to reload
      if (!currentVoucher || currentVoucher.id !== voucher.id) {
        console.log(`Kiosk ${kioskIndex}: Rolling voucher changed or was used, reloading...`);
        await load();
        return;
      }

      // If the voucher has been used (authorized_guest_count > 0), try to rotate
      if (currentVoucher.authorizedGuestCount > 0) {
        console.log(`Kiosk ${kioskIndex}: Current voucher has been used, attempting to rotate...`);
        try {
          await api.rotateRollingVoucherIfNeeded();
          // Load the new voucher at our assigned index
          await load();
        } catch (error) {
          console.error(`Kiosk ${kioskIndex}: Failed to rotate rolling voucher:`, error);
        }
      }
    } catch (error: any) {
      console.error(`Kiosk ${kioskIndex}: Error checking voucher status:`, error);
      // If there's an error getting the current voucher, just reload
      if (error?.status === 404) {
        await load();
      }
    }
  }, [voucher, state, kioskIndex, load]);

  // Store functions in refs for stable event listeners
  const loadRef = useRef(load);
  const checkAndRotateRef = useRef(checkAndRotate);

  useEffect(() => {
    loadRef.current = load;
    checkAndRotateRef.current = checkAndRotate;
  }, [load, checkAndRotate]);

  useEffect(() => {
    if (kioskIndex === null) return;

    const handleVouchersUpdated = () => loadRef.current();

    loadRef.current();
    window.addEventListener("vouchersUpdated", handleVouchersUpdated);

    // Set up periodic checking every 10 seconds
    const interval = setInterval(() => {
      checkAndRotateRef.current();
      setCountdown(10);
    }, 10000);

    // Countdown timer that updates every second
    const countdownInterval = setInterval(() => {
      setCountdown((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);

    return () => {
      window.removeEventListener("vouchersUpdated", handleVouchersUpdated);
      clearInterval(interval);
      clearInterval(countdownInterval);
    };
  }, [kioskIndex]); // Only run when kioskIndex is set

  const renderContent = useCallback(() => {
    switch (state) {
      case null:
      case "loading":
        return <Spinner />;
      case "error":
        return (
          <div className="text-center text-5xl sm:text-6xl md:text-7xl text-status-danger">
            Could not load rolling voucher
          </div>
        );
      case "ok":
        const qrAvailable = wifiConfig && wifiString;
        return (
          <div className="space-y-8">
            {/* Instructions */}
            <div className="text-center mb-8">
              <h1 className="font-bold text-4xl sm:text-5xl md:text-6xl mb-4">
                Norquay Guest WiFi Access
              </h1>
              <p className="text-center text-lg sm:text-xl text-gray-600 dark:text-gray-400">Scan the QR code below with your device to connect automatically</p>
            </div>

            {/* Main content */}
            <div className="flex flex-col items-center gap-8 max-w-2xl mx-auto">
              {qrAvailable && (
                <div className="space-y-4 w-full">
                  <WifiQr className="w-full sm:h-80 md:h-96 " />
                </div>
              )}
              <div className="text-center w-full">
                <h2 className="font-medium mb-4 text-3xl sm:text-4xl md:text-5xl">
                  Voucher Code
                </h2>
                <p className="text-lg sm:text-xl text-gray-600 dark:text-gray-400 mt-6">
                  Enter this code when prompted after connecting to the WiFi network
                </p>
                <div className="voucher-code tracking-widest text-5xl sm:text-6xl md:text-7xl mb-4">
                  {voucher ? formatCode(voucher.code) : "No voucher available"}
                </div>
                <div className="flex items-center justify-center gap-3 mt-6">
                  <svg className="w-6 h-6 -rotate-90" viewBox="0 0 36 36">
                    <circle
                      cx="18"
                      cy="18"
                      r="16"
                      fill="none"
                      className="stroke-gray-300 dark:stroke-gray-700"
                      strokeWidth="3"
                    />
                    <circle
                      cx="18"
                      cy="18"
                      r="16"
                      fill="none"
                      className="stroke-blue-500 dark:stroke-blue-400"
                      strokeWidth="3"
                      strokeDasharray="100"
                      strokeDashoffset={100 - (countdown / 10) * 100}
                      strokeLinecap="round"
                      style={{ transition: 'stroke-dashoffset 1s linear' }}
                    />
                  </svg>
                </div>
                <div className="flex items-center justify-center gap-3 mt-6">
                  <br /><span className="text-sm text-gray-500 dark:text-gray-400">
                    Refreshing code in {countdown}s
                  </span>
                </div>
              </div>
            </div>
          </div>
        );
    }
  }, [voucher, state, wifiConfig, wifiString, countdown]);

  return (
    <main className="flex-center h-screen w-full px-4">{renderContent()}</main>
  );
}
