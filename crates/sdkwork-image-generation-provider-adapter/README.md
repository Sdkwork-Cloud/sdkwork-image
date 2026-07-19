# SDKWork Image Generation Provider Adapter

Default L4 implementation of the image generation provider SPI. It maps normalized vendor and
model requests to the generated unified Rust SDK and maps SDK responses back to SPI results.

SDK resource names, SDK methods, DTOs, and vendor option schemas are private adapter concerns.
