# Print Voucher Customization Guide

This guide explains how to customize the printed vouchers for your thermal printer (optimized for 80mm printers like the Epson TM-T88V).

## Configuration File

All print customization is done through the `print-config.json` file in the root directory. This file is volume-mounted into the Docker container, so changes take effect immediately without rebuilding.

## Adding a Logo

1. **Prepare your logo:**
   - Save your logo as a PNG or JPG file
   - Recommended size: 180x60 pixels for 80mm thermal printers
   - Use a simple, high-contrast design for best results on thermal printers

2. **Place the logo file:**
   Place your logo in the frontend public directory
  ```
  /unifi-voucher-manager/frontend/public/logo.png
  ```

3. **Update print-config.json:**
   ```
   {
     "logo": {
       "enabled": true,
       "path": "/logo.png",
       "width": 180,
       "height": 60
     }
   }
   ```

4. **Adjust size if needed:**
   - For wider logos: increase `width` (max ~200-220 for 80mm paper)
   - For taller logos: increase `height` proportionally
   - The logo will be centered and scaled using `object-fit: contain`

## Customizing the Header

```json
{
  "header": {
    "title": "WiFi Access Voucher",      // Main title (large, bold)
    "subtitle": "Welcome to Our Network"  // Optional subtitle (smaller)
  }
}
```

Examples:
- Business: `"title": "Guest WiFi Access"`, `"subtitle": "Thank you for visiting"`
- Hotel: `"title": "Hotel WiFi Voucher"`, `"subtitle": "Enjoy your stay"`
- Cafe: `"title": "Free WiFi"`, `"subtitle": "Enjoy your coffee"`

## Adding Additional Information

```json
{
  "additionalInfo": {
    "enabled": true,
    "fields": [
      {
        "label": "Support",
        "value": "support@example.com"
      },
      {
        "label": "Location",
        "value": "Building A, Floor 2"
      },
      {
        "label": "Website",
        "value": "www.example.com"
      }
    ]
  }
}
```

Common use cases:
- **Business**: Support contact, office location, business hours
- **Hotel**: Reception desk number, hotel website, floor/room location
- **Cafe**: Phone number, social media handles, loyalty program info
- **Event**: Event name, venue location, support booth

## Customizing the Footer

```json
{
  "footer": {
    "customText": "Thank you for visiting! Questions? Call +1-555-0123",
    "showVoucherId": true,      // Show voucher ID for troubleshooting
    "showPrintedTime": true     // Show when voucher was printed
  }
}
```

## Complete Example

Here's a complete example for a coffee shop:

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
  "footer": {
    "customText": "Follow us @YourCafe | Questions? Ask at the counter",
    "showVoucherId": false,
    "showPrintedTime": false
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
      },
      {
        "label": "Location",
        "value": "123 Main St, Downtown"
      }
    ]
  }
}
```

## Testing Your Configuration

1. **Make changes to print-config.json**
   - Edit the file directly on your host machine
   - Changes are immediately available (no rebuild needed)

2. **Test print:**
   - Open the voucher manager in your browser
   - Select a voucher or create a new one
   - Click the print button (printer icon)
   - Choose "Print Preview" in your browser to see the layout
   - Print to your thermal printer

3. **Iterate:**
   - Adjust logo size, text, or additional fields as needed
   - Refresh the browser and test again
   - No container restart required!

## Thermal Printer Tips

### For Epson TM-T88V and similar 80mm printers:

1. **Logo dimensions:**
   - Keep width at or below 220 pixels for best results
   - Use high-contrast black and white designs
   - Avoid gradients (thermal printers work best with solid colors)

2. **Text length:**
   - Keep labels and values concise
   - Long text may wrap or get cut off
   - Test with actual printing to see how it looks

3. **Print settings:**
   - Use browser's print dialog
   - Set margins to minimum (usually 3-4mm)
   - Choose "Portrait" orientation
   - Select appropriate paper size (80mm thermal)

4. **Paper considerations:**
   - Standard thermal paper width: 80mm
   - Typical content width: 70-72mm (accounting for margins)
   - Vouchers print in portrait mode
   - Each voucher has a border for easy cutting

## Disabling Features

To disable any feature, set `enabled` to `false`:

```json
{
  "logo": {
    "enabled": false,  // Logo won't be displayed
    "path": "/logo.png",
    "width": 180,
    "height": 60
  },
  "additionalInfo": {
    "enabled": false,  // Additional info section won't be displayed
    "fields": []
  }
}
```

## Docker Volume Mount

The print-config.json is mounted in docker-compose.yml:

```yaml
volumes:
  - ./print-config.json:/app/frontend/public/print-config.json:ro
```

This means:
- Edit the file on your host machine at `/home/concordia/unifi-voucher-manager/print-config.json`
- Changes are instantly reflected in the container
- No restart or rebuild required
- The `:ro` flag makes it read-only in the container for security

## Troubleshooting

**Logo doesn't appear:**
- Check that the logo file exists in `frontend/public/`
- Verify the path in print-config.json matches the filename
- Check browser console for 404 errors (F12 → Console tab)
- Ensure `logo.enabled` is `true`

**Text is cut off:**
- Reduce logo width/height
- Shorten label or value text
- Check your printer's margins in print settings

**Layout looks wrong:**
- Clear browser cache (Ctrl+Shift+R or Cmd+Shift+R)
- Check print-config.json for JSON syntax errors
- Verify all required fields are present

**Configuration not loading:**
- Check browser console for errors
- Verify JSON syntax is valid (use a JSON validator)
- Ensure the file is properly mounted in compose.yaml

## Need Help?

If you encounter issues:
1. Check browser console (F12 → Console) for errors
2. Check Docker logs: `docker logs unifi-voucher-manager`
3. Verify print-config.json has valid JSON syntax
4. Test with the default configuration first

Happy printing! 🖨️
