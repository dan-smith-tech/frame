import base64
from io import BytesIO

import requests
from PIL import Image

API_URL = "http://0.0.0.0:3000"


def main():
    print("Fetching image from API...")
    response = requests.get(API_URL)
    response.raise_for_status()

    base64_image = response.json().get("image")
    image_bytes = BytesIO(base64.b64decode(base64_image))
    image = Image.open(image_bytes)
    print("Image fetched successfully!")


if __name__ == "__main__":
    main()
