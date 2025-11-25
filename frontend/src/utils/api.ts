import {
  Voucher,
  VoucherCreateData,
  VoucherCreatedResponse,
  VoucherDeletedResponse,
} from "@/types/voucher";
import { notifyVouchersUpdated } from "./actions";

function removeNullUndefined<T extends Record<string, any>>(obj: T): T {
  return Object.fromEntries(
    Object.entries(obj).filter(
      ([_, value]) => value !== null && value !== undefined,
    ),
  ) as T;
}

async function call<T>(endpoint: string, opts: RequestInit = {}) {
  // Remove leading slash from endpoint if present to avoid double slashes
  const cleanEndpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;
  const res = await fetch(`/rust-api/${cleanEndpoint}`, {
    headers: { "Content-Type": "application/json" },
    ...opts,
  });
  if (!res.ok) {
    const error = new Error(res.statusText);
    (error as any).status = res.status;
    throw error;
  }
  return res.json() as Promise<T>;
}

export const api = {
  getAllVouchers: () => call<{ data: Voucher[] }>("/vouchers"),

  getRollingVoucher: () => call<Voucher>("/vouchers/rolling"),

  getNewestVoucher: () => call<Voucher>("/vouchers/newest"),

  getVoucherDetails: (id: string) =>
    call<Voucher>(`/vouchers/details?id=${encodeURIComponent(id)}`),

  createVoucher: async (data: VoucherCreateData) => {
    const filteredData = removeNullUndefined(data);
    const result = await call<VoucherCreatedResponse>("/vouchers", {
      method: "POST",
      body: JSON.stringify(filteredData),
    });
    await notifyVouchersUpdated();
    return result;
  },

  createRollingVoucher: async () => {
    const result = await call<Voucher>("/vouchers/rolling", {
      method: "POST",
    });
    await notifyVouchersUpdated();
    return result;
  },

  deleteExpiredVouchers: async () => {
    const result = await call<VoucherDeletedResponse>("/vouchers/expired", {
      method: "DELETE",
    });
    await notifyVouchersUpdated();
    return result;
  },

  deleteExpiredRollingVouchers: async () => {
    const result = await call<VoucherDeletedResponse>(
      "/vouchers/expired/rolling",
      {
        method: "DELETE",
      },
    );
    await notifyVouchersUpdated();
    return result;
  },

  deleteSelectedVouchers: async (ids: string[]) => {
    const qs = ids.map(encodeURIComponent).join(",");
    const result = await call<VoucherDeletedResponse>(
      `/vouchers/selected?ids=${qs}`,
      {
        method: "DELETE",
      },
    );
    await notifyVouchersUpdated();
    return result;
  },
};
