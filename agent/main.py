import base64
import os
from io import BytesIO

import requests
from dotenv import load_dotenv
from inky.auto import auto
from PIL import Image

load_dotenv()


def main():
    headers = {"x-private-key": os.getenv("PRIVATE_KEY", "testkey")}
    response = requests.get(os.getenv("API_URL", "localhost:3001"), headers=headers)
    response.raise_for_status()

    base64_image = response.json().get("image")
    image_bytes = BytesIO(base64.b64decode(base64_image))
    image = Image.open(image_bytes)

    inky = auto(ask_user=True, verbose=True)
    inky.set_image(image)
    inky.show()


if __name__ == "__main__":
    main()
