"use client";

import { api } from "@/utils/api";
import { getRuntimeConfig } from "@/utils/runtimeConfig";
import { useCallback, useEffect, useState } from "react";

export default function WelcomePage() {
  const [visited, setVisited] = useState(false);
  const [ssid, setSsid] = useState<string | undefined>(undefined);

  const rotateVoucher = useCallback(async () => {
    try {
      await api.createRollingVoucher();
    } catch (error: any) {
      if (error?.status !== 403) {
        console.error("Failed to create rolling voucher", error);
      }
    }
  }, []);

  useEffect(() => {
    const { WIFI_SSID: ssid } = getRuntimeConfig();
    setSsid(ssid);
  }, []);

  useEffect(() => {
    if (visited) {
      return;
    }
    rotateVoucher();
    setVisited(true);
  }, [visited, rotateVoucher]);

  return (
    <main className="flex-center h-screen w-full px-4">
      <div className="w-full text-center font-bold text-4xl sm:text-5xl md:text-7xl lg:text-9xl leading-snug">
        {ssid ? `Welcome to ${ssid}!` : "Welcome!"}
      </div>
    </main>
  );
}
