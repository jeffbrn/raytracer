namespace Research.RayTracer.Draw {
	public class Color : IEquatable<Color> {
		// expected values between 0 and 1
		private readonly float _r;
		private readonly float _g;
		private readonly float _b;

		public Color(float red, float green, float blue) {
			_r = red;
			_g = green;
			_b = blue;
		}

		public float Red => _r;
		public float Green => _g;
		public float Blue => _b;

		public Color Normalize() {
			var r = (_r <= 0) ? 0 : _r;
			var g = (_g <= 0) ? 0 : _g;
			var b = (_b <= 0) ? 0 : _b;
			var max = r > g ? r : g;
			if (b > max) max = b;
			return (max <= 1) ? new Color(r, g, b) : new Color(r/max, g/max, b/max);

		}

		#region IEquatable
		public bool Equals(Color? other) {
			if (other == null) return false;
			return ApproxEq(_r, other._r) && ApproxEq(_g, other._g) && ApproxEq(_b, other._b);
		}

		public override bool Equals(object? obj) {
			return Equals(obj as Color);
		}

		public override int GetHashCode() {
			return _r.GetHashCode() ^ _g.GetHashCode() ^ _b.GetHashCode();
		}
		#endregion

		#region Operators
		public static Color operator +(Color lhs, Color rhs) => new(lhs._r + rhs._r, lhs._g + rhs._g, lhs._b + rhs._b);
		public static Color operator -(Color lhs, Color rhs) => new(lhs._r - rhs._r, lhs._g - rhs._g, lhs._b - rhs._b);
		public static Color operator *(Color lhs, float rhs) => new(lhs._r * rhs, lhs._g * rhs, lhs._b * rhs);
		public static Color operator *(Color lhs, Color rhs) => new(lhs._r * rhs._r, lhs._g * rhs._g, lhs._b * rhs._b);
		#endregion

		public (byte r, byte g, byte b, byte a) GetBytes() {
			return (ConvertToByte(_r), ConvertToByte(_g), ConvertToByte(_b), 255);
		}
		public static Color SetBytes(uint rgba) {
			var b = (byte)((rgba & 0xff000000) >> 24);
			var g = (byte)((rgba & 0x00ff0000) >> 16);
			var r = (byte)((rgba & 0x0000ff00) >> 8);
			return new Color((float)(r / 255.0), (float)(g / 255.0), (float)(b / 255.0));
		}

		private static byte ConvertToByte(float v) {
			if (v <= 0) return 0;
			if (v >= 1) return 255;
			return Convert.ToByte(v * 255);
		}

		private static bool ApproxEq(float f1, float f2) {
			const float epsilon = 0.000001f;
			var diff = MathF.Abs(f1 - f2);
			return diff <= epsilon;
		}

	}
}
