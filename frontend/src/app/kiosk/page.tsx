"use client";

import Spinner from "@/components/utils/Spinner";
import { TriState } from "@/types/state";
import { Voucher } from "@/types/voucher";
import { api } from "@/utils/api";
import { formatCode } from "@/utils/format";
import { useEffect, useCallback, useState } from "react";

export default function KioskPage() {
  const [voucher, setVoucher] = useState<Voucher | null>(null);
  const [state, setState] = useState<TriState | null>(null);

  const load = useCallback(async () => {
    // If already loading, do nothing
    if (state === "loading") {
      return;
    }

    try {
      setState("loading");
      await api.getRollingVoucher().then(setVoucher);
      setState("ok");
    } catch (error: any) {
      if (error?.status !== 404) {
        setState("error");
        return;
      }

      // If 404 (Not Found), create rolling voucher
      try {
        await api.createRollingVoucher().then(setVoucher);
        setState("ok");
      } catch {
        setState("error");
      }
    }
  }, []);

  useEffect(() => {
    load();
    window.addEventListener("vouchersUpdated", load);

    return () => {
      window.removeEventListener("vouchersUpdated", load);
    };
  }, [load]);

  function renderContent() {
    switch (state) {
      case "loading":
        return <Spinner />;
      case "error":
        return "Could not load rolling voucher";
      case "ok":
        return (
          <>
            Voucher Code
            <br />
            {voucher ? (
              <div className="voucher-code">{formatCode(voucher.code)}</div>
            ) : (
              "No voucher available"
            )}
          </>
        );
    }
  }

  return (
    <div className="flex-center h-screen w-full px-4">
      <div className="w-full text-center font-bold text-4xl sm:text-5xl md:text-7xl lg:text-9xl leading-snug">
        {renderContent()}
      </div>
    </div>
  );
}
