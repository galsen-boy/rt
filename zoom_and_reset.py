from PIL import Image

def zoom_image(image_path, output_path, zoom_factor):
    """
    Zooms in on an image and saves it to a new file.
    
    :param image_path: Path to the input image.
    :param output_path: Path to save the zoomed image.
    :param zoom_factor: Factor by which to zoom the image.
    """
    with Image.open(image_path) as img:
        width, height = img.size
        new_width = int(width * zoom_factor)
        new_height = int(height * zoom_factor)
        
        # Calculate cropping box
        left = (new_width - width) / 2
        top = (new_height - height) / 2
        right = (new_width + width) / 2
        bottom = (new_height + height) / 2
        
        img = img.crop((left, top, right, bottom))
        img = img.resize((width, height), Image.LANCZOS)
        img.save(output_path)

def reset_image(image_path, output_path):
    """
    Resets the image to its original state (or reverts zoom) and saves it to a new file.
    
    :param image_path: Path to the input image.
    :param output_path: Path to save the reset image.
    """
    with Image.open(image_path) as img:
        img.save(output_path)

def main():
    # Paths
    original_image_path = 'example.png'
    zoomed_image_path = 'example_zoomed.png'
    reset_image_path = 'example_reset.png'
    
    # Zoom factor (e.g., 1.5 for zooming in by 50%)
    zoom_factor = 1.5
    
    # Apply zoom
    zoom_image(original_image_path, zoomed_image_path, zoom_factor)
    print(f"Zoomed image saved to {zoomed_image_path}")
    
    # Reset image
    reset_image(original_image_path, reset_image_path)
    print(f"Reset image saved to {reset_image_path}")

if __name__ == "__main__":
    main()
