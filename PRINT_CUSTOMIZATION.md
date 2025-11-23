# Print Voucher Customization Guide

This comprehensive guide explains how to customize printed vouchers for your thermal printer (optimized for 80mm printers like the Epson TM-T88V). You can customize both the content and layout order without editing any code.

## Configuration File

All print customization is done through the `print-config.json` file in the root directory. This file is volume-mounted into the Docker container, so changes take effect immediately - just refresh your browser.

## Layout Configuration

### Customizing Section Order

The `layout.order` array determines the order of sections on the printed voucher. You can reorder any of the 7 available sections to match your needs:

```json
{
  "layout": {
    "order": ["logo", "header", "code", "details", "qr", "additionalInfo", "footer"]
  }
}
```

### Available Sections

1. **logo** - Company/venue logo image
2. **header** - Title and subtitle
3. **code** - Voucher access code (formatted)
4. **details** - Voucher information (duration, guests, data limit, speeds)
5. **qr** - QR code with WiFi connection details
6. **additionalInfo** - Terms of Service or other custom information
7. **footer** - Custom text, voucher ID, and print timestamp

### Layout Examples

**Default Layout:**
```json
"layout": {
  "order": ["logo", "header", "code", "details", "qr", "additionalInfo", "footer"]
}
```

**QR Code First (for quick scanning):**
```json
"layout": {
  "order": ["qr", "code", "logo", "header", "details", "additionalInfo", "footer"]
}
```

**Terms First:**
```json
"layout": {
  "order": ["logo", "additionalInfo", "header", "code", "details", "qr", "footer"]
}
```

**Minimal Layout:**
```json
"layout": {
  "order": ["header", "code", "qr", "footer"]
}
```

## Section Customization

#### Adding a Logo

1. **Prepare your logo:**
   - Save your logo as a PNG or JPG file
   - Recommended size: 180x60 to 240x120 pixels for 80mm thermal printers
   - Use a simple, high-contrast design for best results on thermal printers

2. **Place the logo file:**
   Place your logo in the frontend public directory:
   ```
   /unifi-voucher-manager/frontend/public/logo.png
   ```

3. **Configure in print-config.json:**
   ```json
   "logo": {
     "enabled": true,
     "path": "/logo.png",
     "width": 240,
     "height": 120
   }
   ```

4. **Adjust size if needed:**
   - For wider logos: increase `width` (max ~200-240 for 80mm paper)
   - For taller logos: increase `height` proportionally
   - The logo will be centered and scaled using `object-fit: contain`

### Customizing the Header

Configure the title and optional subtitle:

```json
"header": {
  "title": "WiFi Access Voucher",
  "subtitle": "Welcome to Our Network"
}
```

Examples:
- Business: `"title": "Guest WiFi Access"`, `"subtitle": "Thank you for visiting"`
- Hotel: `"title": "Hotel WiFi Voucher"`, `"subtitle": "Enjoy your stay"`
- Cafe: `"title": "Free WiFi"`, `"subtitle": "Enjoy your coffee"`

### QR Code Size

Adjust the QR code size for better scanning:

```json
"qrCode": {
  "size": 240
}
```

**Tip:** Larger QR codes (200-280px) are easier to scan from a distance, especially important for thermal printers.

### Adding Additional Information

Use this section for Terms of Service, support info, location, or custom messages:

### Adding Additional Information

Use this section for Terms of Service, support info, location, or custom messages:

```json
"additionalInfo": {
  "enabled": true,
  "fields": [
    {
      "label": "Terms of Service",
      "value": "By accessing the wireless network...\n\nYou agree not to use..."
    },
    {
      "label": "Support",
      "value": "support@example.com"
    },
    {
      "label": "Location",
      "value": "Building A, Floor 2"
    }
  ]
}
```

**Line Breaks:** Use `\n` in text fields for line breaks (they will render properly with preserved formatting).

Common use cases:
- **Business**: Support contact, office location, business hours
- **Hotel**: Reception desk number, hotel website, floor/room location
- **Cafe**: Phone number, social media handles, loyalty program info
- **Event**: Event name, venue location, support booth

### Customizing the Footer

Configure footer text and display options:

```json
"footer": {
  "customText": "Thank you for visiting! Questions? Call +1-555-0123",
  "showVoucherId": true,
  "showPrintedTime": true
}
```

- `customText`: Any custom message you want at the bottom
- `showVoucherId`: Show voucher ID for troubleshooting (true/false)
- `showPrintedTime`: Show when voucher was printed (true/false)

## Complete Configuration Examples

### Coffee Shop Example

### Coffee Shop Example

```json
{
  "logo": {
    "enabled": true,
    "path": "/coffee-logo.png",
    "width": 180,
    "height": 60
  },
  "header": {
    "title": "Free WiFi",
    "subtitle": "Enjoy Your Coffee ☕"
  },
  "qrCode": {
    "size": 240
  },
  "layout": {
    "order": ["logo", "header", "qr", "code", "details", "additionalInfo", "footer"]
  },
  "additionalInfo": {
    "enabled": true,
    "fields": [
      {
        "label": "Valid For",
        "value": "1 hour from activation"
      },
      {
        "label": "Need Help?",
        "value": "Ask any barista"
      }
    ]
  },
  "footer": {
    "customText": "Follow us @YourCafe | Questions? Ask at the counter",
    "showVoucherId": false,
    "showPrintedTime": false
  }
}
```

### Hotel Example

```json
{
  "logo": {
    "enabled": true,
    "path": "/hotel-logo.png",
    "width": 240,
    "height": 100
  },
  "header": {
    "title": "Hotel WiFi Voucher",
    "subtitle": "Enjoy Your Stay"
  },
  "qrCode": {
    "size": 200
  },
  "layout": {
    "order": ["logo", "header", "code", "qr", "details", "additionalInfo", "footer"]
  },
  "additionalInfo": {
    "enabled": true,
    "fields": [
      {
        "label": "Reception",
        "value": "Dial 0 from your room"
      },
      {
        "label": "Support Hours",
        "value": "24/7 Front Desk"
      }
    ]
  },
  "footer": {
    "customText": "Thank you for choosing our hotel!",
    "showVoucherId": true,
    "showPrintedTime": true
  }
}
```

## Testing Your Configuration

1. **Make changes to print-config.json**
   - Edit the file directly on your host machine
   - Changes are immediately available (no rebuild or restart needed)

2. **Test print:**
   - Open the voucher manager in your browser
   - Select a voucher or create a new one
   - Click the print button (printer icon)
   - Choose "Print Preview" in your browser to see the layout
   - Use "Print to PDF" to test layouts without wasting paper
   - Print to your thermal printer when satisfied

3. **Iterate:**
   - Adjust logo size, text, layout order, or additional fields as needed
   - Refresh the browser page (Ctrl+F5 or Cmd+Shift+R) and test again
   - No container restart required!

## Disabling Features

To disable any feature, set `enabled` to `false`:

```json
"logo": {
  "enabled": false,
  "path": "/logo.png",
  "width": 180,
  "height": 60
},
"additionalInfo": {
  "enabled": false,
  "fields": []
}
```

## Thermal Printer Tips

### For Epson TM-T88V and similar 80mm printers:

1. **Logo dimensions:**
   - Keep width at or below 240 pixels for best results
   - Use high-contrast black and white designs
   - Avoid gradients (thermal printers work best with solid colors)

2. **QR Code:**
   - Larger QR codes (200-280px) scan better from a distance
   - For thermal printers, consider putting QR code earlier in layout order
   - Test scanning distance with your specific printer

3. **Text length:**
   - Keep labels and values concise
   - Long text may wrap or get cut off
   - Test with actual printing to see how it looks

4. **Print settings:**
   - Use browser's print dialog
   - Set margins to minimum (usually 3-4mm)
   - Choose "Portrait" orientation
   - Select appropriate paper size (80mm thermal)

5. **Paper considerations:**
   - Standard thermal paper width: 80mm
   - Typical content width: 70-72mm (accounting for margins)
   - Vouchers print in portrait mode
   - Each voucher has a border for easy cutting

## How It Works

1. **Live Updates**: Changes to `print-config.json` take effect immediately - just refresh the print page
2. **Volume Mounted**: The config file is volume-mounted in Docker, so no rebuild is needed
3. **Fallback**: If `layout.order` is missing or invalid, the default order is used
4. **Conditional Rendering**: Sections only display if they're enabled in their respective config sections

## Docker Volume Mount

The print-config.json is mounted in docker-compose.yml:

```yaml
volumes:
  - ./print-config.json:/app/frontend/public/print-config.json:ro
```

This means:
- Edit the file on your host machine
- Changes are instantly reflected in the container
- No restart or rebuild required
- The `:ro` flag makes it read-only in the container for security

## Troubleshooting

**Logo doesn't appear:**
- Check that the logo file exists in `frontend/public/`
- Verify the path in print-config.json matches the filename
- Check browser console for 404 errors (F12 → Console tab)
- Ensure `logo.enabled` is `true`

**Layout doesn't change:**
- Make sure the JSON is valid (use a JSON validator like jsonlint.com)
- Clear browser cache and refresh (Ctrl+Shift+R or Cmd+Shift+R)
- Check browser console for errors

**Section not showing:**
- Check if the section is enabled in its config (e.g., `"enabled": true`)
- Verify the section name in `layout.order` matches exactly (case-sensitive)
- Available sections: logo, header, code, details, qr, additionalInfo, footer

**Text is cut off:**
- Reduce logo width/height
- Shorten label or value text
- Check your printer's margins in print settings

**Text formatting issues:**
- Use `\n` for line breaks in JSON strings
- The system preserves line breaks automatically
- Check for valid JSON syntax (no trailing commas, proper escaping)

**Configuration not loading:**
- Check browser console (F12 → Console) for errors
- Verify JSON syntax is valid
- Ensure the file is properly mounted in docker-compose.yml
- Check Docker logs: `docker logs unifi-voucher-manager`

## Need Help?

If you encounter issues:
1. Check browser console (F12 → Console) for errors
2. Verify JSON syntax with a validator
3. Check Docker logs: `docker logs unifi-voucher-manager`
4. Test with the default configuration first
5. Ensure all required fields are present in print-config.json

Happy printing! 🖨️
