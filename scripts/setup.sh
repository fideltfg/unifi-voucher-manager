#!/bin/bash
set -e

# UniFi Voucher Manager Setup Script
# This script prepares the environment for running the UniFi Voucher Manager

echo "=================================="
echo "UniFi Voucher Manager Setup"
echo "=================================="
echo ""

# Check if running from the correct directory
if [ ! -f "compose.yaml" ]; then
    echo "Error: compose.yaml not found. Please run this script from the project root directory."
    exit 1
fi

# Create logs directory with correct permissions
echo "Creating logs directory..."
if [ ! -d "logs" ]; then
    mkdir -p logs
    echo "✓ Created logs directory"
else
    echo "✓ Logs directory already exists"
fi

echo "Setting correct permissions for logs directory (UID 1001)..."
if command -v sudo &> /dev/null; then
    sudo chown -R 1001:1001 logs
    echo "✓ Permissions set (requires sudo)"
else
    chown -R 1001:1001 logs 2>/dev/null || {
        echo "⚠ Warning: Could not set permissions. You may need to run:"
        echo "  sudo chown -R 1001:1001 logs"
    }
fi

# Create frontend/public directory for logo
echo ""
echo "Creating frontend/public directory for logo..."
if [ ! -d "frontend/public" ]; then
    mkdir -p frontend/public
    echo "✓ Created frontend/public directory"
else
    echo "✓ frontend/public directory already exists"
fi

# Check for configuration files
echo ""
echo "Checking configuration files..."

# Check for .env file
if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        echo "⚠ .env file not found. Copying from .env.example..."
        cp .env.example .env
        echo "✓ Created .env file - PLEASE EDIT IT WITH YOUR SETTINGS"
        NEED_ENV_CONFIG=true
    else
        echo "⚠ Warning: Neither .env nor .env.example found."
        echo "  You'll need to create .env manually or download .env.example"
    fi
else
    echo "✓ .env file exists"
fi

# Check for voucher-tiers.json
if [ ! -f "voucher-tiers.json" ]; then
    echo "⚠ Warning: voucher-tiers.json not found."
    echo "  Download from: https://raw.githubusercontent.com/fideltfg/unifi-voucher-manager/main/voucher-tiers.json"
else
    echo "✓ voucher-tiers.json exists"
fi

# Check for print-config.json
if [ ! -f "print-config.json" ]; then
    echo "⚠ Warning: print-config.json not found."
    echo "  Download from: https://raw.githubusercontent.com/fideltfg/unifi-voucher-manager/main/print-config.json"
else
    echo "✓ print-config.json exists"
fi

# Final instructions
echo ""
echo "=================================="
echo "Setup Complete!"
echo "=================================="
echo ""

if [ "$NEED_ENV_CONFIG" = true ]; then
    echo "⚠ IMPORTANT: Edit the .env file with your UniFi controller settings:"
    echo "  nano .env"
    echo ""
fi

echo "To add a logo for printed vouchers:"
echo "  cp /path/to/your/logo.png frontend/public/logo.png"
echo ""
echo "To start the application:"
echo "  docker compose up -d --build"
echo ""
echo "To view logs:"
echo "  docker logs -f unifi-voucher-manager"
echo "  # or"
echo "  tail -f logs/vouchers.log.\$(date +%Y-%m-%d)"
echo ""
echo "Access the application at: http://localhost:3012"
echo "(or the port you configured in compose.yaml)"
echo ""
