# SDKWork Image Generation Provider SPI

Stable image generation provider ports and normalized contracts. Business services depend on this
crate; external SDK adapters implement it.

Consumers import the public crate root. Provider implementations must not add generated SDK DTOs,
URLs, credentials, or transport method names to these contracts.
