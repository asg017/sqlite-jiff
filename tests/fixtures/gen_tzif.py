#!/usr/bin/env python3
"""Generate the self-contained TZif blob used by the tzif() test in test.sql.

This builds a minimal, hand-crafted TZif v2 file (RFC 8536) for a single
fixed-offset zone with no transitions, so it parses identically on any machine
without depending on the system time zone database. The default is a distinctive
+05:45 offset, chosen so the blob is obviously synthetic rather than a real IANA
zone.

Usage:
    python3 tests/fixtures/gen_tzif.py            # print the +05:45 blob as hex
    python3 tests/fixtures/gen_tzif.py -o out.tzif  # also write the raw bytes

Paste the printed hex into test.sql as a `x'...'` blob literal, e.g.:
    select jiff_zoned('2024-07-01T12:00:00', tzif('Custom/Plus0545', x'...'));
"""
import argparse
import struct


def tzif_block(desig: bytes, utoff: int) -> bytes:
    """One TZif data block: a single time type, zero transitions."""
    # header counts: isutcnt, isstdcnt, leapcnt, timecnt, typecnt, charcnt
    header = b"TZif" + b"2" + b"\x00" * 15 + struct.pack(
        ">6I", 0, 0, 0, 0, 1, len(desig)
    )
    # ttinfo: utoff (int32), isdst (u8), desigidx (u8), then the designation
    ttinfo = struct.pack(">i", utoff) + b"\x00" + b"\x00"
    return header + ttinfo + desig


def build(desig: bytes, utoff: int, tz_string: bytes) -> bytes:
    """A version-2 TZif file is the v1-format block, the v2 block, and a footer."""
    return tzif_block(desig, utoff) * 2 + b"\n" + tz_string + b"\n"


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("-o", "--output", help="also write the raw TZif bytes here")
    parser.add_argument("--offset", type=int, default=20700,
                        help="offset from UTC in seconds (default 20700 = +05:45)")
    parser.add_argument("--abbrev", default="NPT",
                        help="time zone abbreviation (default NPT)")
    parser.add_argument("--tz-string", default="<+0545>-5:45",
                        help="POSIX TZ footer string (default <+0545>-5:45)")
    args = parser.parse_args()

    blob = build(args.abbrev.encode() + b"\x00", args.offset,
                 args.tz_string.encode())
    if args.output:
        with open(args.output, "wb") as f:
            f.write(blob)
    print(blob.hex())


if __name__ == "__main__":
    main()
