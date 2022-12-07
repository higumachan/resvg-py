import imageio
from resvg_py import resvg_py


def main():
    svg_xml = """
    <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
        <rect x="10" y="10" width="80" height="80" fill="red" />
    </svg>
    """
    array = resvg_py.render_svg(svg_xml, 1.0)

    imageio.imwrite("assets/simple.png", array)

if __name__ == '__main__':
    main()