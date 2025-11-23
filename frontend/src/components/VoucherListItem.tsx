import { Voucher } from "@/types/voucher";
import {
  formatCode,
  formatDuration,
  formatGuestUsage,
  formatStatus,
} from "@/utils/format";
import { memo, useCallback } from "react";

type Props = {
  voucher: Voucher;
  selected: boolean;
  editMode: boolean;
  onClick?: (v: Voucher) => void;
};

const VoucherListItem = ({ voucher, selected, editMode, onClick }: Props) => {
  const statusClass = voucher.expired
    ? "bg-status-danger text-status-danger"
    : voucher.activatedAt
      ? "bg-status-warning text-status-warning"
      : "bg-status-success text-status-success";
  const onClickHandler = useCallback(
    () => onClick?.(voucher),
    [voucher, onClick],
  );

  return (
    <div
      onClick={onClickHandler}
      className={`card cursor-pointer hover:border-accent/50 transition-colors
        ${selected ? "border-accent" : ""}
        ${editMode ? "relative" : ""}`}
    >
      <div className="flex items-center gap-4">
        {editMode && (
          <div className="flex-shrink-0">
            <div
              className={`w-6 h-6 rounded-full border-2 flex-center
              ${selected ? "selected-accent" : "unselected-neutral"}`}
            >
              {selected && <div className="w-3 h-3 bg-white rounded-full" />}
            </div>
          </div>
        )}

        {/* Code */}
        <div className="flex-shrink-0 w-32">
          <div className="voucher-code text-lg">{formatCode(voucher.code)}</div>
        </div>

        {/* Name */}
        <div className="flex-1 min-w-0">
          <div className="font-semibold truncate">{voucher.name}</div>
        </div>

        {/* Guests Used */}
        <div className="flex-shrink-0 w-24 text-sm text-secondary text-right">
          {formatGuestUsage(
            voucher.authorizedGuestCount,
            voucher.authorizedGuestLimit,
          )}
        </div>

        {/* Session Time */}
        <div className="flex-shrink-0 w-24 text-sm text-secondary text-right">
          {formatDuration(voucher.timeLimitMinutes)}
        </div>

        {/* Status */}
        <div className="flex-shrink-0 w-24">
          <span
            className={`px-2 py-1 rounded-lg text-xs font-semibold uppercase ${statusClass} inline-block`}
          >
            {formatStatus(voucher.expired, voucher.activatedAt)}
          </span>
        </div>

        {/* First Used / Expires */}
        <div className="flex-shrink-0 w-40 text-xs text-secondary text-right">
          {voucher.activatedAt ? (
            <div>Used: {voucher.activatedAt}</div>
          ) : voucher.expiresAt ? (
            <div>Expires: {voucher.expiresAt}</div>
          ) : (
            <div>—</div>
          )}
        </div>
      </div>
    </div>
  );
};

export default memo(VoucherListItem);
