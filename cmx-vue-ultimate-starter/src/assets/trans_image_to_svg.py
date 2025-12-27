from PIL import Image
import potrace

def png_to_vector_svg(input_png, output_svg):
    """真正的矢量化转换"""
    img = Image.open(input_png).convert('L')  # 转为灰度
    bitmap = potrace.Bitmap(img)
    path = bitmap.trace()

    with open(output_svg, 'w') as f:
        f.write(path.to_svg())