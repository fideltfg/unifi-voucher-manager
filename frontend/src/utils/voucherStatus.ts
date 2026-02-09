import type { Voucher } from "@/types/voucher";

/**
 * Returns the appropriate status class for a voucher based on its state
 */
export function getVoucherStatusClass(voucher: Voucher): string {
  if (voucher.expired) {
    return "bg-status-danger text-status-danger";
  }
  if (voucher.activatedAt) {
    return "bg-status-warning text-status-warning";
  }
  return "bg-status-success text-status-success";
}

/**
 * Returns a human-readable status text for a voucher
 */
export function getVoucherStatusText(voucher: Voucher): string {
  if (voucher.expired) {
    return "expired";
  }
  if (voucher.activatedAt) {
    return "active";
  }
  return "unused";
}
