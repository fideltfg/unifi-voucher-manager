# Print Layout Customization Guide

## Overview
The print voucher layout is now fully customizable through the `print-config.json` file. You can reorder sections without editing any code - simply update the JSON file and refresh your browser.

## Configuration

### Layout Order
The `layout.order` array in `print-config.json` determines the order of sections on the printed voucher:

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

### Example Layouts

#### Default Layout (Current)
```json
"layout": {
  "order": ["logo", "header", "code", "details", "qr", "additionalInfo", "footer"]
}
```
This puts the logo at the top, followed by the header, access code, voucher details, QR code, terms of service, and footer.

#### QR Code First
```json
"layout": {
  "order": ["qr", "code", "logo", "header", "details", "additionalInfo", "footer"]
}
```
Puts the QR code prominently at the top for quick scanning.

#### Terms First
```json
"layout": {
  "order": ["logo", "additionalInfo", "header", "code", "details", "qr", "footer"]
}
```
Shows Terms of Service early in the voucher.

#### Minimal Layout
```json
"layout": {
  "order": ["header", "code", "qr", "footer"]
}
```
Only shows essential information (note: sections won't display if they're disabled in config).

## How It Works

1. **Live Updates**: Changes to `print-config.json` take effect immediately - just refresh the print page
2. **Volume Mounted**: The config file is volume-mounted in Docker, so no rebuild is needed
3. **Fallback**: If `layout.order` is missing or invalid, the default order is used
4. **Conditional Rendering**: Sections only display if they're enabled in their respective config sections

## Customizing Individual Sections

Each section can be customized independently:

### Logo
```json
"logo": {
  "enabled": true,
  "path": "/logo.png",
  "width": 240,
  "height": 120
}
```

### Header
```json
"header": {
  "title": "WiFi Access Voucher",
  "subtitle": "24/7 High-Speed Internet"
}
```

### Footer
```json
"footer": {
  "customText": "This is not a receipt or proof of purchase.",
  "showVoucherId": true,
  "showPrintedTime": true
}
```

### QR Code
```json
"qrCode": {
  "size": 240
}
```

### Additional Info (Terms of Service)
```json
"additionalInfo": {
  "enabled": true,
  "fields": [
    {
      "label": "Terms of Service",
      "value": "Line 1\nLine 2\nLine 3"
    }
  ]
}
```

## Tips

- **Testing**: Print to PDF to test different layouts without wasting paper
- **QR Code Size**: Larger QR codes (200-280px) are easier to scan from a distance
- **Line Breaks**: Use `\n` in text fields for line breaks (e.g., in Terms of Service)
- **Logo**: Place your logo file in the `public` directory and reference it as `/logo.png`
- **Thermal Printers**: For thermal printers (like Epson TM-T88V), consider putting QR code earlier and increasing its size

## Troubleshooting

**Layout doesn't change?**
- Make sure the JSON is valid (use a JSON validator)
- Refresh the browser page (Ctrl+F5 or Cmd+Shift+R)
- Check browser console for errors

**Section not showing?**
- Check if the section is enabled in its config (e.g., `"enabled": true` for logo)
- Verify the section name in `layout.order` matches exactly (case-sensitive)

**Text formatting issues?**
- Use `\n` for line breaks in JSON strings
- The CSS `whiteSpace: 'pre-line'` preserves line breaks
