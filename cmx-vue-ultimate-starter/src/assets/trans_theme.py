import json
import os
import sys
import re
# 将json格式的数据转化为css主题文件
# 设置UTF-8编码环境
os.environ["PYTHONUTF8"] = "1"
os.environ["PYTHONIOENCODING"] = "utf-8"
sys.stdin.reconfigure(encoding='utf-8')
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

def extract_colors(css_content):
    """从CSS内容中提取五种主题色"""
    colors = {}
    color_keys = ['primary', 'secondary', 'muted', 'accent', 'destructive']
    
    for key in color_keys:
        match = re.search(rf'--{key}:\s*(oklch\([^)]+\))', css_content)
        colors[key] = match.group(1) if match else f"oklch(0.5 0 0)"  # 默认值
    
    return colors

def convert_to_css_themes(data, output_file="themes-combined.css"):
    css_content = []
    theme_list = []
    theme_names = []
    current_dir = os.path.dirname(os.path.abspath(__file__))
    themes_path = os.path.join(current_dir, output_file)
    
    for theme in data:
        theme_name = theme["name"].lower().replace(" ", "-")
        clean_theme_name = theme_name.replace("theme-", "")  # 移除可能存在的theme-前缀
        theme_names.append(clean_theme_name)
        
        # 处理亮色主题 (移除:root)
        light_theme = theme["code"].split(".dark")[0].strip()
        light_theme = light_theme.replace(":root {", "").replace("}", "").strip()
        
        # 提取五种颜色
        colors = extract_colors(light_theme)
        
        # 添加可视化颜色变量
        light_theme += "\n  --vis-color0: var(--chart-1);"
        light_theme += "\n  --vis-color1: var(--chart-2);"
        light_theme += "\n  --vis-color2: var(--chart-3);"
        light_theme += "\n  --vis-color3: var(--chart-4);"
        light_theme += "\n  --vis-color4: var(--chart-5);"
        
        # 添加到主题列表
        theme_list.append({
            "theme": clean_theme_name,
            "colors": {
                "primary": colors['primary'],
                "secondary": colors['secondary'],
                "muted": colors['muted'],
                "accent": colors['accent'],
                "destructive": colors['destructive']
            }
        })
        
        css_content.append(f".theme-{theme_name} {{\n{light_theme}\n}}")
        
        # 处理暗色主题 (如果存在)
        if ".dark" in theme["code"]:
            dark_theme = theme["code"].split(".dark")[1].split("@theme")[0].strip()
            dark_theme = dark_theme.replace("{", "").replace("}", "").strip()
            # 为暗色主题也添加可视化颜色变量
            dark_theme += "\n  --vis-color0: var(--chart-1);"
            dark_theme += "\n  --vis-color1: var(--chart-2);"
            dark_theme += "\n  --vis-color2: var(--chart-3);"
            dark_theme += "\n  --vis-color3: var(--chart-4);"
            dark_theme += "\n  --vis-color4: var(--chart-5);"
            css_content.append(f".theme-{theme_name}.dark {{\n{dark_theme}\n}}")
    
    # 将所有主题写入单个CSS文件
    with open(themes_path, "w", encoding="utf-8") as f:
        f.write("\n\n".join(css_content))
    
    # 输出主题信息到控制台
    print("\n导出主题信息:\n")
    print("export const themes = [")
    for theme in theme_list:
        print(f"  {{")
        print(f"    theme: '{theme['theme']}',")
        print(f"    colors: {{")
        print(f"      primary: '{theme['colors']['primary']}',")
        print(f"      secondary: '{theme['colors']['secondary']}',")
        print(f"      muted: '{theme['colors']['muted']}',")
        print(f"      accent: '{theme['colors']['accent']}',")
        print(f"      destructive: '{theme['colors']['destructive']}'")
        print(f"    }}")
        print(f"  }},")
    print("] as const\n")
    
    # 输出THEMES常量数组
    print("export const THEMES = [")
    for name in theme_names:
        print(f"  '{name}',")
    print("] as const\n")
    
    print(f"已生成合并的主题文件: {output_file}")

if __name__ == "__main__":
    # 获取当前脚本所在目录
    current_dir = os.path.dirname(os.path.abspath(__file__))
    themes_path = os.path.join(current_dir, "theme.json")
    
    try:
        with open(themes_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        convert_to_css_themes(data)
    except FileNotFoundError:
        print(f"错误：在当前目录下找不到 theme.json 文件")
        print(f"当前目录是: {current_dir}")
        print("请确保文件存在且名称正确")
    except json.JSONDecodeError as e:
        print(f"错误：JSON解析失败 - {str(e)}")
        print("请检查theme.json文件格式是否正确")
    except Exception as e:
        print(f"发生未知错误: {str(e)}")