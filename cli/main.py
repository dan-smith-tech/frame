import argparse
import os
import sys
from datetime import datetime

import requests

URL = "http://localhost:8000/image/"


def validate_date(date_str):
    """Validate date is in YYYY-MM-DD format."""
    try:
        datetime.strptime(date_str, "%Y-%m-%d")
    except ValueError:
        print("❌ Date must be in YYYY-MM-DD format.", file=sys.stderr)
        sys.exit(1)


def validate_file(path):
    """Check that the file exists and is readable."""
    if not os.path.isfile(path):
        print(f"❌ File not found: {path}", file=sys.stderr)
        sys.exit(1)
    if not os.access(path, os.R_OK):
        print(f"❌ File is not readable: {path}", file=sys.stderr)
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(
        description="Upload an image with associated date."
    )
    parser.add_argument(
        "image_path", help="Relative or absolute path to the image file"
    )
    parser.add_argument("date", help="Date in YYYY-MM-DD format")
    args = parser.parse_args()

    image_path = args.image_path
    date = args.date

    validate_file(image_path)
    validate_date(date)

    with open(image_path, "rb") as f:
        filename = os.path.basename(image_path)
        files = {"file": (filename, f, "application/octet-stream")}
        data = {"date": date}
        print(f"Uploading {filename} with date {date}...")

        resp = requests.post(URL, data=data, files=files)

    if resp.ok:
        print("Upload successful!")
        print("Response:", resp.json())
    else:
        print(f"Upload failed ({resp.status_code}): {resp.text}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
