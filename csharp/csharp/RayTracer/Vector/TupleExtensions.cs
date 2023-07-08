using System.Numerics;

namespace Research.RayTracer.Vector {
	public static class TupleExtensions {
		public static bool ApproxEq<T>(this RayTuple<T> lhs, RayTuple<T> rhs, double delta) where T : INumber<T> {
			if (Math.Abs((double)Convert.ToDouble(lhs.X - rhs.X)) > delta) return false;
			if (Math.Abs((double)Convert.ToDouble(lhs.Y - rhs.Y)) > delta) return false;
			if (Math.Abs((double)Convert.ToDouble(lhs.Z - rhs.Z)) > delta) return false;
			if (Math.Abs((double)Convert.ToDouble(lhs.W - rhs.W)) > delta) return false;
			return true;
		}

		public static Matrix<double> Translate<T>(this Matrix<double> m, RayTuple<T> translate) where T : INumber<T> {
			var t = translate.Cast<double>();
			var transform = Matrix<double>.Identity(4);
			transform[0, 3] = t.X;
			transform[1, 3] = t.Y;
			transform[2, 3] = t.Z;
			transform[3, 3] = t.W;
			return m * transform;
		}

		public static Matrix<double> Scale(this Matrix<double> m, RayTuple<double> scale) {
			var transform = Matrix<double>.Identity(4);
			transform[0, 0] = scale.X;
			transform[1, 1] = scale.Y;
			transform[2, 2] = scale.Z;
			return m * transform;
		}

		public static Matrix<double> RotateX(this Matrix<double> m, double angle_radians) {
			var transform = Matrix<double>.Identity(4);
			var (s, c) = Math.SinCos(angle_radians);
			transform[1, 1] = c;
			transform[1, 2] = -s;
			transform[2, 1] = s;
			transform[2, 2] = c;
			return m * transform;
		}

		public static Matrix<double> RotateY(this Matrix<double> m, double angle_radians) {
			var transform = Matrix<double>.Identity(4);
			var (s, c) = Math.SinCos(angle_radians);
			transform[0, 0] = c;
			transform[0, 2] = s;
			transform[2, 0] = -s;
			transform[2, 2] = c;
			return m * transform;
		}

		public static Matrix<double> RotateZ(this Matrix<double> m, double angle_radians) {
			var transform = Matrix<double>.Identity(4);
			var (s, c) = Math.SinCos(angle_radians);
			transform[0, 0] = c;
			transform[0, 1] = -s;
			transform[1, 0] = s;
			transform[1, 1] = c;
			return m * transform;
		}

		public static RayTuple<double> Shearing<T>(this RayTuple<T> point, double x_y, double x_z, double y_x, double y_z, double z_x, double z_y)
			where T : INumber<T> {
			var transform = Matrix<double>.Identity(4);
			transform[0, 1] = x_y;
			transform[0, 2] = x_z;
			transform[1, 0] = y_x;
			transform[1, 2] = y_z;
			transform[2, 0] = z_x;
			transform[2, 1] = z_y;
			return transform * point.Cast<double>();
		}
	}
}
