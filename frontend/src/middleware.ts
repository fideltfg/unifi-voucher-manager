import { NextResponse, NextRequest } from "next/server";

export const config = {
  matcher: "/rust-api/:path*",
};

const DEFAULT_FRONTEND_TO_BACKEND_URL = "http://127.0.0.1";
const DEFAULT_BACKEND_BIND_PORT = "8080";

const IPV6_IPV4_MAPPED_PREFIX = "::ffff:";

export function middleware(request: NextRequest) {
  // Remove the /rust-api prefix and reconstruct the path for the backend
  const backendPath = request.nextUrl.pathname.replace(/^\/rust-api/, "/api");

  const backendUrl =
    process.env.FRONTEND_TO_BACKEND_URL || DEFAULT_FRONTEND_TO_BACKEND_URL;
  const backendPort =
    process.env.BACKEND_BIND_PORT || DEFAULT_BACKEND_BIND_PORT;

  const backendFullUrl = new URL(
    `${backendUrl}:${backendPort}${backendPath}${request.nextUrl.search}`,
  );

  const response = NextResponse.rewrite(backendFullUrl, { request });

  // Forward the real client IP
  let clientIp = request.headers.get("x-forwarded-for") || "";

  // Strip IPv6 prefix if it's a mapped IPv4
  if (clientIp.startsWith(IPV6_IPV4_MAPPED_PREFIX)) {
    clientIp = clientIp.replace(IPV6_IPV4_MAPPED_PREFIX, "");
  }

  response.headers.set("x-forwarded-for", clientIp);
  return response;
}
