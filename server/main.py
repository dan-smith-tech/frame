import os

from fastapi import FastAPI, File, Form, UploadFile

app = FastAPI()

LOCAL_IMAGE_STORE = ".images/"
os.makedirs(LOCAL_IMAGE_STORE, exist_ok=True)


@app.post("/image/")
async def upload_image(file: UploadFile = File(...), date: str = Form(...)):
    contents = await file.read()

    if not contents:
        return {"error": "No content uploaded"}

    if not file.filename:
        return {"error": "No filename provided"}

    if not file.filename.lower().endswith((".png", ".jpg", ".jpeg")):
        return {"error": "Invalid file type: only PNG and JPEG images are allowed"}

    ext = os.path.splitext(file.filename)[1]
    new_filename = f"{date}{ext}"
    save_path = os.path.join(LOCAL_IMAGE_STORE, new_filename)

    with open(save_path, "wb") as f:
        f.write(contents)

    return {
        "filename": new_filename,
        "content_type": file.content_type,
        "size": len(contents),
        "stored_at": save_path,
    }
