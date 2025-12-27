from PIL import Image
import os

def batch_resize_images(input_folder, output_folder, size_list, maintain_aspect_ratio=True):
    """
    批量将图片修改为一系列指定的大小
    
    参数:
        input_folder (str): 输入图片文件夹路径
        output_folder (str): 输出图片文件夹路径
        size_list (list): 目标尺寸列表，格式为 [(宽1, 高1), (宽2, 高2), ...]
        maintain_aspect_ratio (bool): 是否保持原始宽高比，默认为True
    """
    # 创建输出文件夹
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)
    
    # 支持的图片格式
    supported_formats = ('.jpg', '.jpeg', '.png', '.bmp', '.tiff', '.webp')
    
    # 遍历输入文件夹中的所有图片
    for filename in os.listdir(input_folder):
        if filename.lower().endswith(supported_formats):
            input_path = os.path.join(input_folder, filename)
            
            try:
                # 打开图片
                with Image.open(input_path) as img:
                    original_width, original_height = img.size
                    name, ext = os.path.splitext(filename)
                    
                    # 为每个目标尺寸生成新图片
                    for i, (target_width, target_height) in enumerate(size_list):
                        if maintain_aspect_ratio:
                            # 保持宽高比，以较小的缩放比例为准
                            width_ratio = target_width / original_width
                            height_ratio = target_height / original_height
                            ratio = min(width_ratio, height_ratio)
                            
                            new_width = int(original_width * ratio)
                            new_height = int(original_height * ratio)
                        else:
                            # 不保持宽高比，直接拉伸到目标尺寸
                            new_width = target_width
                            new_height = target_height
                        
                        # 调整图片大小
                        resized_img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
                        
                        # 生成输出文件名
                        output_filename = f"{name}_{new_width}x{new_height}{ext}"
                        output_path = os.path.join(output_folder, output_filename)
                        
                        # 保存图片
                        resized_img.save(output_path)
                        print(f"已生成: {output_filename}")
                        
            except Exception as e:
                print(f"处理图片 {filename} 时出错: {e}")

# 使用示例
if __name__ == "__main__":
    # 定义一系列目标尺寸
    target_sizes = [
        (1024, 1024),     # 正方形图标
        # (256, 256),     # 正方形图标
        # (128, 128),     # 小图标
        # (32, 32),       # 极小图标
        # (30, 30),       # 极小图标
        # (44, 44),       # 极小图标
        # (71, 71),       # 极小图标
        # (89, 89),       # 极小图标
        # (107, 107),       # 极小图标
        # (142, 142),       # 极小图标
        # (150, 150),       # 极小图标
        # (284, 284),       # 极小图标
        # (310, 310),       # 极小图标
        # (50, 50),       # 极小图标
    ]
    
    # 设置输入输出文件夹路径
    input_directory = "./input_images"    # 请修改为您的输入文件夹路径
    output_directory = "./resized_images" # 请修改为您的输出文件夹路径
    
    # 执行批量处理
    batch_resize_images(input_directory, output_directory, target_sizes, maintain_aspect_ratio=True)
    
    print("批量处理完成！")