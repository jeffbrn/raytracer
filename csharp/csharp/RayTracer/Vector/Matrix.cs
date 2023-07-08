using System.Collections;
using System.Numerics;

namespace Research.RayTracer.Vector {
	public class Matrix<T> : IEnumerable<Vector<T>>, IEquatable<Matrix<T>> where T : INumber<T> {
		private readonly List<Vector<T>> _data;

		public Matrix() {
			_data = new List<Vector<T>>();
		}
		public Matrix(int rows, int cols) : this() {
			for(int i = 0; i < rows; i++) {
				_data.Add(new Vector<T>(cols));
			}
		}
		public Matrix(IEnumerable<Vector<T>> rows) : this() {
			_data.AddRange(rows);
			var cols = _data.Select(x => x.Count).Distinct().Count();
			if (cols != 1) throw new ArgumentOutOfRangeException(nameof(rows));
		}

		public static Matrix<T> Identity(int size) {
			var ident = (T)Convert.ChangeType(1, typeof(T));
			var retval = new Matrix<T>(size, size);
			for (int x = 0; x < size; x++) {
				retval[x, x] = ident;
			}
			return retval;
		}

		public int Rows { get { return _data.Count; } }
		public int Cols { get { return _data.Count == 0 ? 0 : _data[0].Count; } }

		public T this[int row, int col] {
			get {
				if (row < 0 || row >= Rows) throw new ArgumentOutOfRangeException(nameof(row));
				if (col < 0 || col >= Cols) throw new ArgumentOutOfRangeException(nameof(col));
				return _data[row][col];
			}
			set {
				if (row < 0 || row >= Rows) throw new ArgumentOutOfRangeException(nameof(row));
				if (col < 0 || col >= Cols) throw new ArgumentOutOfRangeException(nameof(col));
				_data[row][col] = value;
			}
		}

		public Vector<T> Row(int row) {
			if (row < 0 || row >= Rows) throw new ArgumentOutOfRangeException(nameof(row));
			return _data[row];
		}

		public Vector<T> Col(int col) {
			if (col < 0 || col >= Cols) throw new ArgumentOutOfRangeException(nameof(col));
			var retval = new Vector<T>(Rows);
			for(int i = 0; i < Rows; i++) {
				retval[i] = _data[i][col];
			}
			return retval;
		}

		public void Add(Vector<T> item) {
			if (_data.Count > 0 && item.Count != _data[0].Count) throw new ArgumentOutOfRangeException(nameof(item));
			_data.Add(item);
		}

		public IEnumerator<Vector<T>> GetEnumerator() {
			return _data.GetEnumerator();
		}

		IEnumerator IEnumerable.GetEnumerator() {
			return this.GetEnumerator();
		}

		public bool Equals(Matrix<T>? other) {
			if (other == null) return false;
			if (_data.Count != other._data.Count) return false;
			for(int i = 0; i < _data.Count; i++) {
				if (!_data[i].Equals(other._data[i])) return false;
			}
			return true;
		}

		public override bool Equals(object? obj) {
			return Equals(obj as Matrix<T>);
		}

		public override int GetHashCode() {
			return _data.Aggregate(0, (a, b) => a.GetHashCode() ^ b.GetHashCode());
		}

		public static Matrix<T> operator *(Matrix<T> a, Matrix<T> b) {
			var retval = new Matrix<T>(a.Rows, b.Cols);
			for (int i = 0; i < a.Rows; i++)
				for (int j = 0; j < b.Cols; j++) {
					retval[i, j] = a.Row(i).Dot(b.Col(j));
				}
			return retval;
		}

		public static RayTuple<T> operator *(Matrix<T> lhs, RayTuple<T> rhs) {
			if (lhs.Rows != 4 || lhs.Cols != 4) throw new InvalidOperationException("Invalid matrix size");
			var x = lhs[0, 0] * rhs.X + lhs[0, 1] * rhs.Y + lhs[0, 2] * rhs.Z + lhs[0, 3] * rhs.W;
			var y = lhs[1, 0] * rhs.X + lhs[1, 1] * rhs.Y + lhs[1, 2] * rhs.Z + lhs[1, 3] * rhs.W;
			var z = lhs[2, 0] * rhs.X + lhs[2, 1] * rhs.Y + lhs[2, 2] * rhs.Z + lhs[2, 3] * rhs.W;
			var w = lhs[3, 0] * rhs.X + lhs[3, 1] * rhs.Y + lhs[3, 2] * rhs.Z + lhs[3, 3] * rhs.W;
			var retval = new RayTuple<T>(x, y, z, w);
			return retval;
		}
	}
}
