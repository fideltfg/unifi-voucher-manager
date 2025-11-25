# UniFi Voucher Manager

A modern, touch-friendly web application for managing WiFi vouchers on UniFi controllers with enhanced print customization features.
Perfect for businesses, cafes, hotels, and home networks that need to provide guest WiFi access with professional printed vouchers.

![WiFi Voucher Manager](./assets/view.png)

> **Note:** This is a customized fork with enhanced features for thermal printer voucher printing, preset tiers, and live JSON configuration.
> Based on [etiennecollin/unifi-voucher-manager](https://github.com/etiennecollin/unifi-voucher-manager)

<!-- vim-markdown-toc GFM -->

- [✨ Features](#-features)
  - [🎫 Voucher Management & WiFi QR Code](#-voucher-management--wifi-qr-code)
  - [Kiosk Display](#kiosk-display)
  - [🎨 Modern Interface](#-modern-interface)
  - [🔧 Technical Features](#-technical-features)
- [🚀 Quick Start](#-quick-start)
  - [Using Docker Compose (Recommended)](#using-docker-compose-recommended)
  - [Without Docker](#without-docker)
- [⚙️ Configuration](#-configuration)
  - [Getting UniFi API Credentials](#getting-unifi-api-credentials)
  - [Voucher Tiers](#voucher-tiers)
  - [Print Configuration](#print-configuration)
  - [Rolling Vouchers and Kiosk Page](#rolling-vouchers-and-kiosk-page)
    - [How Rolling Vouchers Work](#how-rolling-vouchers-work)
  - [Environment Variables](#environment-variables)
- [🐛 Troubleshooting](#-troubleshooting)
  - [Common Issues](#common-issues)
  - [Getting Help](#getting-help)

<!-- vim-markdown-toc -->

## ✨ Features

### 🎫 Voucher Management & WiFi QR Code

- **Preset Voucher Tiers** - Quick create vouchers with predefined configurations:
  - Duration (hours to days)
  - Download/upload speed limits (Mbps)
  - Data usage caps (MB)
  - Display tier details before creation
  - Live configuration via `voucher-tiers.json`
- **Custom Create** - Full control over voucher parameters:
  - Custom name
  - Duration (minutes to days)
  - Guest count limits
  - Data usage limits
  - Upload/download speed limits
- **Browse Vouchers** - Browse and search existing vouchers by name
  - Card view or compact list view
  - Real-time filtering
- **Bulk Operations** - Select and delete multiple vouchers at once
- **Enhanced Print System** - Professional voucher printing for thermal printers:
  - **Logo support** - Add your business logo
  - **Customizable layout** - Header, footer, and additional information fields
  - **Terms of Service** - Display TOS or usage policies
  - **Optimized for thermal printers** - Specifically designed for 80mm printers (Epson TM-T88V, etc.)
  - **Live configuration** - Changes via `print-config.json` take effect immediately
  - **List or grid format** - Choose your print layout
- **Auto-cleanup** - Remove expired vouchers with a single click
- **QR Code** - Easily connect guests to your network
- **Rolling Vouchers** - Automatically generate a voucher for the next guest when the current one gets used
  - Configurable duration, speed limits, and data caps
  - Settings managed in `voucher-tiers.json`

### Kiosk Display

The kiosk page (`/kiosk`) provides a guest-friendly interface displaying:

- **QR Code**: For easy network connection (if configured in [Environment Variables](#environment-variables))
- **Current Voucher**: The active rolling voucher code
- **Real-time Updates**: Automatically refreshes when the rolling voucher changes

### 🎨 Modern Interface

- **Touch-Friendly** – Optimized for tablet, mobile, and desktop
- **UniFi-Matched Dark Theme** – Custom dark theme matching UniFi's official design system
  - Dark mode with UniFi colors (#191b1e, #1f2226, #e1e3e9)
  - Light mode support
  - Theme persists across sessions
  - Instant theme switching without white flash
- **Responsive Design** - Works seamlessly across all screen sizes
- **Multiple View Modes** - Switch between card and list views
- **Smooth Animations** – Semantic transitions for polished UX
- **Real-time Notifications** - Instant feedback for all operations

### 🔧 Technical Features

- **Docker Ready** - Easy deployment with Docker Compose and included healthcheck
- **UniFi Integration** - Session-based authentication with traditional UniFi Controller API
  - Automatic session management (30-minute expiry with auto-refresh)
  - Username/password authentication
  - Support for self-signed certificates
- **Live Configuration** - JSON-based configuration files with volume mounts
  - `voucher-tiers.json` - Tier presets and rolling voucher settings
  - `print-config.json` - Print layout customization
  - Changes take effect immediately without rebuild
- **Audit Logging** - Comprehensive logging of voucher operations
  - Daily rolling log files stored in `./logs/` directory
  - Logs include hostname, client IP, voucher details
  - Useful for tracking and auditing voucher creation
- **Secure Architecture** - Next.js (TypeScript + Tailwind CSS) frontend with an Axum-based Rust backend that handles all UniFi Controller communication, keeping credentials isolated from the user-facing UI

## 🚀 Quick Start

### Using Docker Compose (Recommended)

**Option 1: Quick Setup with Script**

```bash
# Clone the repository
git clone https://github.com/fideltfg/unifi-voucher-manager
cd unifi-voucher-manager

# Run the setup script (creates directories, sets permissions)
./scripts/setup.sh

# Edit your configuration
nano .env

# Start the application
docker compose up -d --build
```

**Option 2: Manual Setup**

1. **Create the configuration files**
   ```bash
   # Download the compose file and configuration examples
   curl -o compose.yaml https://raw.githubusercontent.com/fideltfg/unifi-voucher-manager/main/compose.yaml
   curl -o .env.example https://raw.githubusercontent.com/fideltfg/unifi-voucher-manager/main/.env.example
   curl -o voucher-tiers.json https://raw.githubusercontent.com/fideltfg/unifi-voucher-manager/main/voucher-tiers.json
   curl -o print-config.json https://raw.githubusercontent.com/fideltfg/unifi-voucher-manager/main/print-config.json
   
   # Create logs directory with correct permissions
   mkdir -p logs
   sudo chown -R 1001:1001 logs
   
   # Copy the example and edit with your settings
   cp .env.example .env
   nano .env  # or use your preferred editor
   ```

2. **Configure your environment**
   - Edit the `.env` file with your UniFi controller details (see [Environment Variables](#environment-variables))
   - Customize `voucher-tiers.json` with your preferred tier presets (optional)
   - Customize `print-config.json` for your printed vouchers (optional)

3. **Add your logo (optional)**
   ```bash
   # Create the frontend public directory
   mkdir -p frontend/public
   
   # Copy your logo (PNG recommended, 180x60px for thermal printers)
   cp /path/to/your/logo.png frontend/public/logo.png
   ```

4. **Start the application**
   ```bash
   docker compose up -d --build --force-recreate
   ```

5. **Access the interface**
   - Open your browser to `http://localhost:3000` (or your configured port)

### Without Docker

1. **Install the dependencies**
   - `rust >= 1.88.0`
   - `nodejs >= 24.3.0`
   - `npm >= 11.4.2`
2. **Clone the repository**
   ```bash
   git clone https://github.com/fideltfg/unifi-voucher-manager
   cd unifi-voucher-manager
   ```
3. **Configure your environment**
   - In your shell, set the required environment variables (see [Environment Variables](#environment-variables))
     or set them in a `.env` file at the root of the repository and use the `dotenv` feature of the rust backend.
4. **Start the frontend and backend**

   ```bash
   # Backend (without using a .env file)
   cd backend && cargo run --release

   # Backend (using a .env file)
   cd backend && cargo run --release --features dotenv

   # Frontend (development)
   cd frontend && npm install && npm run dev

   # Frontend (release)
   cd frontend && npm ci && npm run build && npm run start
   ```

5. **Access the interface**
   - Open your browser to `http://localhost:3000`.

## ⚙️ Configuration

### Getting UniFi Controller Credentials

1. **Access your UniFi Controller**
2. **Create or use an existing administrator account**
   - You can use the main admin account or create a dedicated account for the voucher manager
   - The account needs permissions to manage hotspot vouchers
3. **Note your credentials**
   - Username (typically your email or local username)
   - Password
4. **Find your Site ID** (optional)
   - Usually "default" for most installations
   - Can be found in the controller URL when viewing a specific site
   - Format: `https://your-controller/manage/site/SITE_ID/dashboard`

### Voucher Tiers

The application supports predefined voucher tiers with preset durations and speed/data limits. These tiers are configured in the `voucher-tiers.json` file and are volume-mounted into the container for live editing.

**Configuration structure:**
```json
{
  "rollingVoucher": {
    "enabled": false,
    "durationHours": 1,
    "downloadMbps": 5,
    "uploadMbps": 2,
    "dataLimitMB": "unlimited"
  },
  "tiers": [
    {
      "name": "1 Hour Basic",
      "durationHours": 1,
      "downloadMbps": 10,
      "uploadMbps": 5,
      "dataLimitMB": 500
    },
    {
      "name": "4 Hours Standard",
      "durationHours": 4,
      "downloadMbps": 25,
      "uploadMbps": 10,
      "dataLimitMB": 2000
    },
    {
      "name": "1 Day Premium",
      "durationHours": 24,
      "downloadMbps": 100,
      "uploadMbps": 50,
      "dataLimitMB": "unlimited"
    }
  ]
}
```

**Configuration options:**
- `durationHours`: Duration in hours (auto-converted to minutes)
- `downloadMbps` / `uploadMbps`: Speed in Mbps (auto-converted to Kbps), or `"unlimited"`
- `dataLimitMB`: Data limit in megabytes, or `"unlimited"`
- `rollingVoucher.enabled`: Enable/disable rolling voucher feature

Changes to this file take effect immediately without container rebuild.

### Print Configuration

Customize printed vouchers for thermal printers using the `print-config.json` file. This feature is specifically optimized for 80mm thermal printers like the Epson TM-T88V.

**Quick configuration:**
```json
{
  "logo": {
    "enabled": true,
    "path": "/logo.png",
    "width": 180,
    "height": 60
  },
  "header": {
    "title": "WiFi Access Voucher",
    "subtitle": ""
  },
  "footer": {
    "customText": "",
    "showVoucherId": true,
    "showPrintedTime": true
  },
  "additionalInfo": {
    "enabled": true,
    "fields": [
      {
        "label": "Terms of Service",
        "value": "By using this network you agree to our terms of service and acceptable use policy."
      }
    ]
  }
}
```

**Print layout order:**
1. **Logo** (optional) - Your business logo at the top
2. **Header** - Title and optional subtitle
3. **Voucher Code** - Large, prominent display code
4. **Voucher Details** - Duration, max guests, data limit, speeds
5. **QR Code** - WiFi connection QR code (if configured)
6. **Network Info** - SSID and password
7. **Additional Info** - Terms of Service or custom information
8. **Footer** - Custom text, voucher ID, print timestamp

**Main options:**
- `logo`: Enable logo, set path (e.g., `/logo.png`), and dimensions (recommended: 180x60px)
- `header`: Customize title and optional subtitle
- `footer`: Add custom text, toggle voucher ID and print timestamp
- `additionalInfo`: Add custom fields (e.g., Terms of Service, support info)

**Quick setup:**
1. Place your logo: `frontend/public/logo.png` (PNG recommended, 180x60px)
2. Edit `print-config.json` - changes take effect immediately
3. Test with browser print preview before printing

**Thermal printer tips:** Keep logo ≤220px width, use high-contrast images, set browser margins to 3-4mm minimum.

**For detailed customization, see [PRINT_CUSTOMIZATION.md](PRINT_CUSTOMIZATION.md)**

### Rolling Vouchers and Kiosk Page

Rolling vouchers provide a seamless way to automatically generate guest network access codes. When one voucher is used, a new one is automatically created for the next guest.

> [!IMPORTANT]
> **Setup Required**
>
> For rolling vouchers to work properly, you **must** configure your UniFi Hotspot:
>
> 1. Go to your UniFi Controller -> Insights -> Hotspot
> 2. Set the **Success Landing Page** to: `https://your-uvm-domain.com/welcome`, the `/welcome` page of UVM
>
> Without this configuration, vouchers **will not** automatically roll when guests connect.

> [!CAUTION]
> To restrict UVM access to the guest subnetwork users while still allowing access to `/welcome` page, set the `GUEST_SUBNETWORK` environment variable. This makes sure guests do not have access to other UVM pages, such as the voucher management interface (the root `/` page).
>
> Without this configuration, guests **will be able** to access the voucher management interface of UVM. This means they will be able to both create and delete vouchers by themselves.

#### How Rolling Vouchers Work

1. **Initial Setup**: Rolling vouchers are generated automatically when needed
2. **Guest Connection**: When a guest connects to your network, they're redirected to the `/welcome` page
3. **Automatic Rolling**: The welcome page triggers the creation of a new voucher for the next guest
   - Rolling vouchers are created with special naming conventions to distinguish them from manually created vouchers, making them easy to identify in your voucher management interface
4. **IP-Based Uniqueness**: Each IP address can only generate one voucher per session (prevents abuse from page reloads)
5. **Daily Maintenance**: To prevent clutter, expired rolling vouchers are automatically deleted at midnight (based on your configured `TIMEZONE` in [Environment Variables](#environment-variables))

### Environment Variables

Make sure to configure the required variables. The optional variables generally have default values that you should not have to change.

> [!TIP]
>
> - To configure the WiFi QR code, you are required to configure the `WIFI_SSID` and `WIFI_PASSWORD` variables.
> - For proper timezone, make sure to set the `TIMEZONE` variable.

> [!IMPORTANT]
> Make sure to expand this section and read what the environment variables are doing. Some variables are **required**, they are placed at the top of the list.

- **`UNIFI_CONTROLLER_URL`: `string`** (_Required_)
  - **Description**: URL to your UniFi controller with protocol (`http://` or `https://`).
  - **Example**: `https://unifi.example.com` or `https://192.168.8.1:443`
- **`UNIFI_USERNAME`: `string`** (_Required_)
  - **Description**: Username for your UniFi controller (administrator account).
  - **Example**: `admin@example.com` or `admin`
- **`UNIFI_PASSWORD`: `string`** (_Required_)
  - **Description**: Password for your UniFi controller account.
  - **Example**: `your-secure-password`

> [!WARNING]
> Improperly setting the `UNIFI_HAS_VALID_CERT` variable **will** prevent UVM from communicating with the UniFi controller.

- **`UNIFI_HAS_VALID_CERT`: `bool`** (_Optional_)
  - **Description**: Whether your UniFi controller uses a valid SSL certificate. This should normally be set to `true`, especially if you access the controller through a reverse proxy or another setup that provides trusted certificates (e.g., Let's Encrypt). **If you connect directly to the controller’s IP address (which usually serves a self-signed certificate), you may need to set this to `false`.**
  - **Example**: `true` (default)
- **`UNIFI_SITE_ID`: `string`** (_Optional_)
  - **Description**: Site ID of your UniFi controller. Using the value `default`, the backend will try to fetch the ID of the default site.
  - **Example**: `default` (default)

- **`GUEST_SUBNETWORK`: `IPv4 CIDR`** (_Optional_)
  - **Description**: Restrict guest network users to only the `/welcome` page. Without this, guests can access the voucher management interface. See [Rolling Vouchers](#rolling-vouchers-and-kiosk-page) for details.
  - **Example**: `10.0.5.0/24`
- **`FRONTEND_BIND_HOST`: `IPv4`** (_Optional_)
  - **Description**: Address on which the frontend server binds.
  - **Example**: `0.0.0.0` (default)
- **`FRONTEND_BIND_PORT`: `u16`** (_Optional_)
  - **Description**: Port on which the frontend server binds.
  - **Example**: `3000` (default)
- **`FRONTEND_TO_BACKEND_URL`: `URL`** (_Optional_)
  - **Description**: URL where the frontend will make its API requests to the backend.
  - **Example**: `http://127.0.0.1` (default)
- **`BACKEND_BIND_HOST`: `IPv4`** (_Optional_)
  - **Description**: Address on which the server binds.
  - **Example**: `127.0.0.1` (default)
- **`BACKEND_BIND_PORT`: `u16`** (_Optional_)
  - **Description**: Port on which the backend server binds.
  - **Example**: `8080` (default)
- **`BACKEND_LOG_LEVEL`: `trace|debug|info|warn|error`** (_Optional_)
  - **Description**: Log level of the Rust backend.
  - **Example**: `info`(default)
- **`TIMEZONE`: [`timezone identifier`](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List)** (_Optional_)
  - **Description**: [Timezone identifier](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List) used to format dates and time.
  - **Example**: `UTC` (default)
- **`ROLLING_VOUCHER_ENABLED`: `bool`** (_Optional_)
  - **Description**: Enable/disable the rolling voucher feature. Note: Rolling voucher settings (duration, speed limits, data caps) are now configured in `voucher-tiers.json` instead of environment variables.
  - **Example**: `false` (default)
- **`WIFI_SSID`: `string`** (_Optional_)
  - **Description**: WiFi SSID used for the QR code. (required for QR code to be generated)
  - **Example**: `My WiFi SSID`
- **`WIFI_PASSWORD`: `string`** (_Optional_)
  - **Description**: WiFi password used for the QR code. If the WiFi network does not have a password, set to an empty string `""`. (required for QR code to be generated)
  - **Example**: `My WiFi Password`
- **`WIFI_TYPE`: `WPA|WEP|nopass`** (_Optional_)
  - **Description**: WiFi security type used. Defaults to `WPA` if a password is provided and `nopass` otherwise.
  - **Example**: `WPA`
- **`WIFI_HIDDEN`: `bool`** (_Optional_)
  - **Description**: Whether the WiFi SSID is hidden or broadcasted.
  - **Example**: `false` (default)

## 🐛 Troubleshooting

### Common Issues

- **Connection to UniFi controller fails**: Verify `UNIFI_CONTROLLER_URL`, `UNIFI_SITE_ID`, and `UNIFI_HAS_VALID_CERT` settings. Check controller is reachable.
- **Application won't start**: Check all required environment variables are set. Review logs: `docker logs unifi-voucher-manager`
- **WiFi QR code disabled**: Configure `WIFI_SSID` and `WIFI_PASSWORD` environment variables.
- **Print issues**: See [PRINT_CUSTOMIZATION.md](PRINT_CUSTOMIZATION.md) for detailed troubleshooting.
- **Log files not created**: Ensure the `logs` directory exists and is owned by UID 1001:
  ```bash
  mkdir -p logs
  sudo chown -R 1001:1001 logs
  docker compose restart
  ```

### Viewing Logs

Application logs are written to both the console and daily rolling files in the `./logs/` directory:

```bash
# View today's log file
cat logs/vouchers.log.$(date +%Y-%m-%d)

# Follow log file in real-time
tail -f logs/vouchers.log.$(date +%Y-%m-%d)

# View Docker container logs
docker logs unifi-voucher-manager

# Follow Docker logs in real-time
docker logs -f unifi-voucher-manager
```

Log files include:
- Voucher creation events with hostname and client IP
- Authentication and session management
- Error messages and warnings
- Backend operational information

### Getting Help

- Check the [Issues](https://github.com/fideltfg/unifi-voucher-manager/issues) page
- Create a new issue with detailed information about your problem
- Include relevant logs and environment details (redact sensitive information)
  - Run the container/backend with `BACKEND_LOG_LEVEL="debug"`
  - Include Docker logs: `docker logs unifi-voucher-manager`
  - Include browser logs: generally by hitting `F12` and going to the 'console' tab of your browser
- For print customization issues, see [PRINT_CUSTOMIZATION.md](PRINT_CUSTOMIZATION.md)

## 📚 Additional Documentation

- **[PRINT_CUSTOMIZATION.md](PRINT_CUSTOMIZATION.md)** - Complete guide for customizing printed vouchers with examples and troubleshooting

## 🙏 Credits

This fork is based on the excellent work by [etiennecollin](https://github.com/etiennecollin/unifi-voucher-manager).

Additional features and customizations:
- Username/password authentication instead of API keys
- Preset voucher tiers with live configuration
- Enhanced print system with logo and custom fields
- UniFi-matched dark theme
- Rolling voucher configuration management
- Improved Docker deployment

---

**⭐ If this project helped you, please consider giving it a star!**

**Original project:** [etiennecollin/unifi-voucher-manager](https://github.com/etiennecollin/unifi-voucher-manager)
