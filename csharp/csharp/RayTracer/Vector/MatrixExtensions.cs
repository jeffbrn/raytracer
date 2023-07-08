using System.Numerics;

namespace Research.RayTracer.Vector {
	public static class MatrixExtensions {
		public static Matrix<TResult> Cast<TRequest, TResult>(this Matrix<TRequest> m)
				where TRequest : INumber<TRequest>
				where TResult : INumber<TResult>
		{
			var retval = new Matrix<TResult>(m.Rows, m.Cols);
			for (int i = 0; i < m.Rows; i++)
				for (int j = 0; j < m.Cols; j++) {
					retval[i, j] = (TResult)Convert.ChangeType(m[i, j], typeof(TResult));
				}
			return retval;
		}

		public static Matrix<T> Transpose<T>(this Matrix<T> m) where T : INumber<T> {
			var retval = new Matrix<T>(m.Rows, m.Cols);
			for (int i = 0; i < m.Rows; i++) {
				for (int j = 0; j < m.Cols; j++) {
					retval[j, i] = m[i, j];
				}
			}
			return retval;
		}

		public static T Determinant<T>(this Matrix<T> m) where T : INumber<T> {
			if (m.Rows < 2 || m.Cols < 2  || m.Rows != m.Cols) throw new InvalidOperationException($"Unable to calc the determinant of a {m.Rows} x {m.Cols} matrix");
			if (m.Rows == 2 && m.Cols == 2) {
				return m[0, 0] * m[1, 1] - m[0, 1] * m[1, 0];
			} else {
				T retval = (T)Convert.ChangeType(0, typeof(T));
				for(int c = 0; c < m.Cols; c++) {
					retval += m[0, c] * m.Cofactor(0, c);
				}
				return retval;
			}
		}

		public static Matrix<T> SubMatrix<T>(this Matrix<T> m, int row, int col) where T : INumber<T> {
			if (row < 0 || row >= m.Rows) throw new ArgumentOutOfRangeException(nameof(row));
			if (col < 0 || col >= m.Cols) throw new ArgumentOutOfRangeException(nameof(col));
			var rows = new List<Vector<T>>();
			for(int i  = 0; i < m.Rows; ++i) {
				if (i == row) continue;
				var vals = new List<T>();
				for(int j = 0; j < m.Cols; ++j) {
					if (j == col) continue;
					vals.Add(m[i, j]);
				}
				var r = new Vector<T>(vals);
				rows.Add(r);
			}
			return new Matrix<T>(rows);
		}

		public static T Minor<T>(this Matrix<T> m, int row, int col) where T : INumber<T> {
			return m.SubMatrix(row, col).Determinant();
		}

		public static T Cofactor<T>(this Matrix<T> m, int row, int col) where T : INumber<T> {
			var minor = m.Minor(row, col);
			var neg = (row + col) % 2 == 1;
			return neg ? -minor : minor;
		}

		public static Matrix<double> Inverse<T>(this Matrix<T> m) where T : INumber<T>
		{
			var det = (double)Convert.ChangeType(m.Determinant(), typeof(double));
			if (det == 0.0) throw new InvalidOperationException("Matrix is not invertible");
			var retval = new Matrix<double>(m.Rows, m.Cols);
			for(int i = 0; i < m.Rows; i++)
				for(int j = 0; j < m.Cols; j++) {
					var v = m.Cofactor(i, j);
					retval[j, i] = (double)Convert.ChangeType(v, typeof(double)) / det;
				}
			return retval;
		}

		public static bool ApproxEq<T>(this Matrix<T> lhs, Matrix<T> rhs, double delta) where T : INumber<T> {
			if (lhs.Rows != rhs.Rows || lhs.Cols != rhs.Cols) return false;
			for(int i = 0; i < lhs.Rows; i++)
				for(int j = 0 ; j < rhs.Cols; j++) {
					if (Math.Abs((double)Convert.ToDouble(lhs[i,j] - rhs[i,j])) > delta) return false;
				}
			return true;
		}

	}
}
