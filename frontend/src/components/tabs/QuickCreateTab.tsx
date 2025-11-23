"use client";

import SuccessModal from "@/components/modals/SuccessModal";
import { Voucher, VoucherCreateData } from "@/types/voucher";
import { api } from "@/utils/api";
import { notify } from "@/utils/notifications";
import { useCallback, useState, useEffect, FormEvent } from "react";

interface VoucherTier {
  id: string;
  name: string;
  description: string;
  durationHours: number;
  downloadMbps: number | null;
  uploadMbps: number | null;
  dataLimitMB: number | null;
}

interface TiersConfig {
  tiers: VoucherTier[];
}

export default function QuickCreateTab() {
  const [loading, setLoading] = useState<boolean>(false);
  const [newVoucher, setNewVoucher] = useState<Voucher | null>(null);
  const [selectedTierId, setSelectedTierId] = useState<string>("standard");
  const [tiers, setTiers] = useState<VoucherTier[]>([]);
  const [loadingTiers, setLoadingTiers] = useState(true);

  useEffect(() => {
    // Load tiers from JSON file
    fetch("/voucher-tiers.json")
      .then((res) => res.json())
      .then((data: TiersConfig) => {
        setTiers(data.tiers);
        setLoadingTiers(false);
      })
      .catch((err) => {
        console.error("Failed to load voucher tiers:", err);
        notify("Failed to load voucher tiers", "error");
        setLoadingTiers(false);
      });
  }, []);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);

    const form = e.currentTarget;
    const data = new FormData(form);
    const tier = tiers.find((t) => t.id === selectedTierId);

    if (!tier) {
      notify("Invalid tier selected", "error");
      setLoading(false);
      return;
    }

    const payload: VoucherCreateData = {
      count: 1,
      name: String(data.get("name")),
      timeLimitMinutes: Math.round(tier.durationHours * 60),
      authorizedGuestLimit: 1,
      rxRateLimitKbps: tier.downloadMbps ? tier.downloadMbps * 1000 : null,
      txRateLimitKbps: tier.uploadMbps ? tier.uploadMbps * 1000 : null,
      dataUsageLimitMBytes: tier.dataLimitMB,
    };

    try {
      const res = await api.createVoucher(payload);
      const voucher = res.vouchers?.[0];
      if (voucher) {
        setNewVoucher(voucher);
        form.reset();
      } else {
        notify(
          "Voucher created, but its data was found in response",
          "warning",
        );
      }
    } catch {
      notify("Failed to create voucher", "error");
    }
    setLoading(false);
  };

  const closeModal = useCallback(() => {
    setNewVoucher(null);
  }, []);

  if (loadingTiers) {
    return (
      <div className="card max-w-lg mx-auto p-6 text-center">
        Loading voucher tiers...
      </div>
    );
  }

  if (tiers.length === 0) {
    return (
      <div className="card max-w-lg mx-auto p-6 text-center text-red-500">
        No voucher tiers configured. Please check voucher-tiers.json
      </div>
    );
  }

  const selectedTier = tiers.find((t) => t.id === selectedTierId) || tiers[0];

  return (
    <div>
      <form onSubmit={handleSubmit} className="card max-w-lg mx-auto space-y-6">
        <label className="block font-medium mb-1">Voucher Tier</label>
        <select 
          value={selectedTierId} 
          onChange={(e) => setSelectedTierId(e.target.value)}
          className="w-full"
          required
        >
          {tiers.map((tier) => (
            <option key={tier.id} value={tier.id}>
              {tier.name} - {tier.description}
            </option>
          ))}
        </select>

        <label className="block font-medium mb-1">Name</label>
        <input 
          name="name" 
          defaultValue={`${selectedTier.name} Voucher`}
          key={selectedTierId}
          required 
        />

        <button type="submit" disabled={loading} className="btn-primary w-full">
          {loading ? "Creating…" : "Create Voucher"}
        </button>
      </form>

      {newVoucher && <SuccessModal voucher={newVoucher} onClose={closeModal} />}
    </div>
  );
}
