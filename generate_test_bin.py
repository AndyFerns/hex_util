# generate_test_binary.py

with open("edgecase.bin", "wb") as f:
    # Pattern: 0x00 to 0xFF repeated
    f.write(bytes(range(256)))

    # Fill up to 512 bytes with alternating 0xAA and 0x55
    f.write(bytes([0xAA, 0x55] * 128))

    # Insert printable ASCII range
    f.write(bytes(range(32, 127)))

    # Insert nulls and control characters
    f.write(bytes([0x00, 0x01, 0x02, 0x03, 0x1B, 0x7F]))

    # Fill rest with random junk pattern
    f.write(b"\xDE\xAD\xBE\xEF" * 16)

    # Pad to 1024 bytes
    current_size = f.tell()
    f.write(b"\x00" * (1024 - current_size))
