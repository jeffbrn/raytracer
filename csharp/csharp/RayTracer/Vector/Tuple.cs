using System.Numerics;

namespace Research.RayTracer.Vector {
	public class RayTuple<T> : IEquatable<RayTuple<T>> where T : INumber<T> {
		private readonly T _x;
		private readonly T _y;
		private readonly T _z;
		private readonly T _w;

		public RayTuple(T x, T y, T z, T w) {
			if(w != ZeroNum && w != OneNum) throw new ArgumentOutOfRangeException(nameof(w));
			_x = x;
			_y = y;
			_z = z;
			_w = w;
		}

		public T X { get => _x; }
		public T Y { get => _y; }
		public T Z { get => _z; }
		public T W { get => _w; }

		public bool IsPoint => _w == OneNum;
		public bool IsVector => _w == ZeroNum;

		public double Magnitude() {
			if (IsPoint) throw new InvalidOperationException("Magnitude is for vectors");
			return Math.Sqrt((double)Convert.ChangeType(_x * _x + _y * _y + _z * _z, typeof(double)));
		}

		public RayTuple<double> Normalize() {
			if (IsPoint) throw new InvalidOperationException("Normailze is for vectors");
			var mag = Magnitude();
			return new(Convert.ToDouble(_x)/mag, Convert.ToDouble(_y)/mag, Convert.ToDouble(_z)/mag, 0.0);
		}

		public T DotProd(RayTuple<T> other) {
			if (IsPoint || other.IsPoint) throw new InvalidOperationException("Dot Product is for vectors");
			return _x*other._x + _y*other._y + _z*other._z;
		}

		public RayTuple<TResult> Cast<TResult>() where TResult : INumber<TResult> {
			var x = (TResult)Convert.ChangeType(_x, typeof(TResult));
			var y = (TResult)Convert.ChangeType(_y, typeof(TResult));
			var z = (TResult)Convert.ChangeType(_z, typeof(TResult));
			var w = (TResult)Convert.ChangeType(_w, typeof(TResult));
			return new RayTuple<TResult>(x, y, z, w);
		}

		public bool Equals(RayTuple<T>? other) {
			return other != null && _x == other._x && _y == other._y && _z == other._z && _w == other._w;
		}

		public override bool Equals(object? obj) {
			return Equals(obj as RayTuple<T>);
		}

		public override int GetHashCode() => _x.GetHashCode() ^ _y.GetHashCode() ^ _z.GetHashCode() ^ _w.GetHashCode();

		public static RayTuple<TNum> Point<TNum>(TNum x, TNum y, TNum z) where TNum : INumber<TNum> =>
			new(x, y, z, RayTuple<TNum>.OneNum);
		public static RayTuple<TNum> Vector<TNum>(TNum x, TNum y, TNum z) where TNum : INumber<TNum> =>
			new(x, y, z, RayTuple<TNum>.ZeroNum);
		public static RayTuple<T> Origin() => new(RayTuple<T>.ZeroNum, RayTuple<T>.ZeroNum, RayTuple<T>.ZeroNum, RayTuple<T>.OneNum);

		public static RayTuple<T> operator +(RayTuple<T> lhs, RayTuple<T> rhs) => new(lhs._x + rhs._x, lhs._y + rhs._y, lhs._z + rhs._z, lhs._w + rhs._w);
		public static RayTuple<T> operator -(RayTuple<T> lhs, RayTuple<T> rhs) => new(lhs._x - rhs._x, lhs._y - rhs._y, lhs._z - rhs._z, lhs._w - rhs._w);
		public static RayTuple<T> operator -(RayTuple<T> rhs) => new(-rhs._x, -rhs._y, -rhs._z, rhs._w);
		public static RayTuple<T> operator *(RayTuple<T> lhs, T rhs) => new(lhs._x * rhs, lhs._y * rhs, lhs._z * rhs, lhs._w);
		public static RayTuple<T> operator *(RayTuple<T> lhs, RayTuple<T> rhs) {
			if (lhs.IsPoint || rhs.IsPoint) throw new InvalidOperationException("Cross Product is for vectors");
			return new(
				lhs._y * rhs._z - lhs._z * rhs._y,
				lhs._z * rhs._x - lhs._x * rhs._z,
				lhs._x * rhs._y - lhs._y * rhs._x,
				ZeroNum);
		}

		public static RayTuple<T> operator /(RayTuple<T> lhs, T rhs) => new(lhs._x / rhs, lhs._y / rhs, lhs._z / rhs, lhs._w);

		private static T ZeroNum => (T)Convert.ChangeType(0, typeof (T));
		private static T OneNum => (T)Convert.ChangeType(1, typeof(T));
	}
}
