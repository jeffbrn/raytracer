using System.Drawing;
using System.Drawing.Imaging;

namespace Research.RayTracer.Draw {
	public class Canvas {
		private readonly int _width;
		private readonly int _height;
		private readonly uint[] _buff;

		public Canvas(int width, int height) {
			_width = width;
			_height = height;
			_buff = new uint[width * height];
			Clear();
		}

		public void Clear() {
			for(int i = 0; i < _width*_height; i++) {
				_buff[i] = 255;
			}
		}
		public void Clear(Color fill) {
			var (r, g, b, a) = fill.GetBytes();
			uint pixel = (uint)(r << 24 | g << 16 | b << 8 | a);
			for (int i = 0; i < _width * _height; i++) {
				_buff[i] = pixel;
			}
		}

		public Color this[int row, int col] {
			get {
				int pos = row * _width + col;
				if (pos < 0 || pos >= _buff.Length) throw new ArgumentOutOfRangeException("Invalid row/col = " + pos);
				return Color.SetBytes(_buff[pos]);
			}
			set {
				var (r, g, b, _) = value.GetBytes();
				SetPixel(row, col, r, g, b);
			}
		}

		public void WriteFile(string fname) {
			Bitmap bmp = new(_width, _height);
			for(int i = 0; i < _height; i++) {
				for(int j = 0; j < _width; j++) {
					var clr = this[i, j];
					bmp.SetPixel(j, i, clr.GetSysColor());
				}
			}
			bmp.Save(fname, ImageFormat.Png);
		}

		private void SetPixel(int x, int y, byte r, byte g, byte b) {
			if (x < 0 || x >= _width) throw new ArgumentOutOfRangeException(nameof(x));
			if (y < 0 || y >= _height) throw new ArgumentOutOfRangeException(nameof(y));
			int pos = y * _width + x;
			uint pixel = (uint)(r << 24 | g << 16 | b << 8 | b);
			_buff[pos] = pixel;
		}
	}
}
