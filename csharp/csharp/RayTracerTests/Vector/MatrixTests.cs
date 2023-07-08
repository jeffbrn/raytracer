using Research.RayTracer.Vector;

namespace Research.RayTracer.Tests.Vector {
	public class MatrixTests {
		[Fact]
		public void Initialize() {
			var m1 = new Matrix<double> {
				new Vector<double> { 1, 2, 3, 4 },
				new Vector<double> { 5.5, 6.5, 7.5, 8.5 },
				new Vector<double> { 9, 10, 11, 12 },
				new Vector<double> { 13.5, 14.5, 15.5, 16.5 }
			};
			Assert.Equal(4, m1.Rows);
			Assert.Equal(4, m1.Cols);
			Assert.Equal(1, m1[0, 0]);
			Assert.Equal(5.5, m1[1, 0]);
			Assert.Equal(7.5, m1[1, 2]);
			Assert.Equal(11, m1[2, 2]);
			Assert.Equal(13.5, m1[3, 0]);
			Assert.Equal(15.5, m1[3, 2]);

			var m2 = new Matrix<double>( new[] {
				new Vector<double>{ -3, 5 },
				new Vector<double>{ 1, -2 }
			});
			Assert.Equal(2, m2.Rows);
			Assert.Equal(2, m2.Cols);
			Assert.Equal(-3, m2[0, 0]);
			Assert.Equal(5, m2[0, 1]);
			Assert.Equal(1, m2[1, 0]);
			Assert.Equal(-2, m2[1, 1]);

			var m3 = new Matrix<int> {
				new Vector<int> { -3, 5, 0 },
				new Vector<int> { 1, -2, -7 },
				new Vector<int> { 0, 1, 1 }
			};
			Assert.Equal(3, m3.Rows);
			Assert.Equal(3, m3.Cols);
			Assert.Equal(-3, m3[0, 0]);
			Assert.Equal(-2, m3[1, 1]);
			Assert.Equal(1, m3[2, 2]);

			var m4 = new Matrix<int>(1, 2);
			Assert.Equal(1, m4.Rows);
			Assert.Equal(2, m4.Cols);
			Assert.Equal(0, m4[0, 0]);
			Assert.Equal(0, m4[0, 1]);
		}

		[Fact]
		public void InitErrors() {
			Assert.Throws<ArgumentOutOfRangeException>(() => {
				var m = new Matrix<int> {
					new Vector<int> { 1, 2 },
					new Vector<int> { 3, 4, 5 },
				};
			});
			Assert.Throws<ArgumentOutOfRangeException>(() => {
				var m = new Matrix<int> (new [] {
					new Vector<int> { 1, 2 },
					new Vector<int> { 3, 4, 5 },
				});
			});
		}

		[Fact]
		public void GettersAndSetters() {
			var m = new Matrix<double>(new[] {
				new Vector<double>{ -3, 5 },
				new Vector<double>{ 1, -2 }
			}) {
				[0, 0] = 54.5
			};
			Assert.Equal(54.5, m[0, 0]);
			m[1, 1] = -1.23;
			Assert.Equal(-1.23, m[1, 1]);
			Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = m[-1, 1]; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = m[1, -1]; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { m[-1, 1] = 2; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { m[1, -1] = 1; });

			Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = m[2, 1]; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = m[1, 2]; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { m[2, 1] = 2; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { m[1, 2] = 1; });
		}

		[Fact]
		public void Equality() {
			var a = new Matrix<int> {
				new Vector<int> { 1, 2, 3, 4 },
				new Vector<int> { 5, 6, 7, 8 },
				new Vector<int> { 9, 8, 7, 6 },
				new Vector<int> { 5, 4, 3, 2 },
			};
			var b = new Matrix<int> {
				new Vector<int> { 1, 2, 3, 4 },
				new Vector<int> { 5, 6, 7, 8 },
				new Vector<int> { 9, 8, 7, 6 },
				new Vector<int> { 5, 4, 3, 2 },
			};
			Assert.Equal(a, b);

			var c = new Matrix<int> {
				new Vector<int> { 2, 3, 4, 5 },
				new Vector<int> { 6, 7, 8, 9 },
				new Vector<int> { 8, 7, 6, 5 },
				new Vector<int> { 4, 3, 2, 1 }
			};
			Assert.NotEqual(a, c);
		}

		[Fact]
		public void Multiply() {
			var a = new Matrix<int> {
				new Vector<int> { 1, 2, 3, 4 },
				new Vector<int> { 5, 6, 7, 8 },
				new Vector<int> { 9, 8, 7, 6 },
				new Vector<int> { 5, 4, 3, 2 },
			};
			var b = new Matrix<int> {
				new Vector<int> { -2, 1, 2, 3 },
				new Vector<int> { 3, 2, 1, -1 },
				new Vector<int> { 4, 3, 6, 5 },
				new Vector<int> { 1, 2, 7, 8 },
			};
			var axb = new Matrix<int> {
				new Vector<int> { 20, 22, 50, 48 },
				new Vector<int> { 44, 54, 114, 108 },
				new Vector<int> { 40, 58, 110, 102 },
				new Vector<int> { 16, 26, 46, 42 },
			};
			Assert.Equal(a * b, axb);

			var c = new Matrix<int> {
				new Vector<int> { 1, 2, 3, 4 },
				new Vector<int> { 2, 4, 4, 2 },
				new Vector<int> { 8, 6, 4, 1 },
				new Vector<int> { 0, 0, 0, 1 },
			};
			var d = new Matrix<int> {
				new Vector<int> { 1 },
				new Vector<int> { 2 },
				new Vector<int> { 3 },
				new Vector<int> { 1 },
			};
			var cxd = new Matrix<int> {
				new Vector<int> { 18 },
				new Vector<int> { 24 },
				new Vector<int> { 33 },
				new Vector<int> { 1 },
			};
			Assert.Equal(c * d, cxd);
		}

		[Fact]
		public void Identity() {
			var a = new Matrix<int> {
				new Vector<int> { 0, 1, 2, 4 },
				new Vector<int> { 1, 2, 4, 8 },
				new Vector<int> { 2, 4, 8, 16 },
				new Vector<int> { 4, 8, 16, 32 },
			};
			Assert.Equal(a * Matrix<int>.Identity(4), a);

			var v = new Matrix<int> {
				new Vector<int> { 1 },
				new Vector<int> { 2 },
				new Vector<int> { 3 },
				new Vector<int> { 4 },
			};
			Assert.Equal(Matrix<int>.Identity(4)*v, v);
		}

		[Fact]
		public void Transpose() {
			var a = new Matrix<int> {
				new Vector<int> { 0, 9, 3, 0 },
				new Vector<int> { 9, 8, 0, 8 },
				new Vector<int> { 1, 8, 5, 3 },
				new Vector<int> { 0, 0, 5, 8 },
			};
			var b = new Matrix<int> {
				new Vector<int> { 0, 9, 1, 0 },
				new Vector<int> { 9, 8, 8, 0 },
				new Vector<int> { 3, 0, 5, 5 },
				new Vector<int> { 0, 8, 3, 8 },
			};
			Assert.Equal(a.Transpose(), b);

			var c = Matrix<float>.Identity(4);
			Assert.Equal(c.Transpose(), Matrix<float>.Identity(4));
		}

		[Fact]
		public void Determinant2x2() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { 1, 5 }),
				new Vector<int>(new[] { -3, 2 })
			});
			Assert.Equal(17, a.Determinant());
		}

		[Fact]
		public void Submatrix() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { 1, 5, 0 }),
				new Vector<int>(new[] { -3, 2, 7 }),
				new Vector<int>(new[] { 0, 6, -3 }),
			});
			var expected1 = new Matrix<int>(new[] {
				new Vector<int>(new[] { -3, 2 }),
				new Vector<int>(new[] { 0, 6 }),
			});
			Assert.Equal(expected1, a.SubMatrix(0, 2));

			var b = new Matrix<int>(new[] {
				new Vector<int>(new[] { -6, 1, 1, 6 }),
				new Vector<int>(new[] { -8, 5, 8, 6 }),
				new Vector<int>(new[] { -1, 0, 8, 2 }),
				new Vector<int>(new[] { -7, 1, -1, 1 }),
			});
			var expected2 = new Matrix<int>(new[] {
				new Vector<int>(new[] { -6, 1, 6 }),
				new Vector<int>(new[] { -8, 8, 6 }),
				new Vector<int>(new[] { -7, -1, 1 }),
			});
			Assert.Equal(expected2, b.SubMatrix(2, 1));
		}

		[Fact]
		public void Minor() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { 3, 5, 0 }),
				new Vector<int>(new[] { 2, -1, -7 }),
				new Vector<int>(new[] { 6, -1, 5 }),
			});
			Assert.Equal(25, a.Minor(1, 0));
		}

		[Fact]
		public void Cofactor() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { 3, 5, 0 }),
				new Vector<int>(new[] { 2, -1, -7 }),
				new Vector<int>(new[] { 6, -1, 5 }),
			});
			Assert.Equal(-12, a.Cofactor(0, 0));
			Assert.Equal(-25, a.Cofactor(1, 0));
		}

		[Fact]
		public void DeterminantLarger() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { 1, 2, 6 }),
				new Vector<int>(new[] { -5, 8, -4 }),
				new Vector<int>(new[] { 2, 6, 4 }),
			});
			Assert.Equal(-196, a.Determinant());

			var b = new Matrix<int>(new[] {
				new Vector<int>(new[] { -2, -8, 3, 5 }),
				new Vector<int>(new[] { -3, 1, 7, 3 }),
				new Vector<int>(new[] { 1, 2, -9, 6 }),
				new Vector<int>(new[] { -6, 7, 7, -9 }),
			});
			Assert.Equal(-4071, b.Determinant());

			var c = new Matrix<int>(new[] {
				new Vector<int>(new[] { 6, 4, 4, 4 }),
				new Vector<int>(new[] { 5, 5, 7, 6 }),
				new Vector<int>(new[] { 4, -9, 3, -7 }),
				new Vector<int>(new[] { 9, 1, 7, -6 }),
			});
			Assert.Equal(-2120, c.Determinant());

			var d = new Matrix<int>(new[] {
				new Vector<int>(new[] { -4, 2, -2, -3 }),
				new Vector<int>(new[] { 9, 6, 2, 6 }),
				new Vector<int>(new[] { 0, -5, 1, -5 }),
				new Vector<int>(new[] { 0, 0, 0, 0 }),
			});
			Assert.Equal(0, d.Determinant());
		}

		[Fact]
		public void Inverse() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { -5, 2, 6, -8 }),
				new Vector<int>(new[] { 1, -5, 1, 8 }),
				new Vector<int>(new[] { 7, 7, -6, -7 }),
				new Vector<int>(new[] { 1, -3, 7, 4 }),
			});
			Assert.Equal(532, a.Determinant());
			Assert.Equal(-160, a.Cofactor(2, 3));
			Assert.Equal(105, a.Cofactor(3,2));
			var expected1 = new Matrix<double>(new[] {
				new Vector<double>(new[] { 0.21805, 0.45113, 0.24060, -0.04511 }),
				new Vector<double>(new[] { -0.80827, -1.45677, -0.44361, 0.52068 }),
				new Vector<double>(new[] { -0.07895, -0.22368, -0.05263, 0.19737 }),
				new Vector<double>(new[] { -0.52256, -0.81391, -0.30075, 0.30639 }),
			});
			Assert.True(expected1.ApproxEq(a.Inverse(), 0.00001));

			var b = new Matrix<int>(new[] {
				new Vector<int>(new[] { 8, -5, 9, 2 }),
				new Vector<int>(new[] { 7, 5, 6, 1 }),
				new Vector<int>(new[] { -6, 0, 9, 6 }),
				new Vector<int>(new[] { -3, 0, -9, -4 }),
			});
			var expected2 = new Matrix<double>(new[] {
				new Vector<double>(new[] { -0.15385, -0.15385, -0.28205, -0.53846 }),
				new Vector<double>(new[] { -0.07692, 0.12308, 0.02564, 0.03077 }),
				new Vector<double>(new[] { 0.35897, 0.35897, 0.43590, 0.92308 }),
				new Vector<double>(new[] { -0.69231, -0.69231, -0.76923, -1.92308 }),
			});
			Assert.True(expected2.ApproxEq(b.Inverse(), 0.00001));

			var c = new Matrix<int>(new[] {
				new Vector<int>(new[] { 9, 3, 0, 9 }),
				new Vector<int>(new[] { -5, -2, -6, -3 }),
				new Vector<int>(new[] { -4, 9, 6, 4 }),
				new Vector<int>(new[] { -7, 6, 6, 2 }),
			});
			var expected3 = new Matrix<double>(new[] {
				new Vector<double>(new[] { -0.04074, -0.07778, 0.14444, -0.22222 }),
				new Vector<double>(new[] { -0.07778, 0.03333, 0.36667, -0.33333 }),
				new Vector<double>(new[] { -0.02901, -0.14630, -0.10926, 0.12963 }),
				new Vector<double>(new[] { 0.17778, 0.06667, -0.26667, 0.33333 }),
			});
			Assert.True(expected3.ApproxEq(c.Inverse(), 0.00001));
		}

		[Fact]
		public void InverseProduct() {
			var a = new Matrix<int>(new[] {
				new Vector<int>(new[] { 3, -9, 7, 3 }),
				new Vector<int>(new[] { 3, -8, 2, -9 }),
				new Vector<int>(new[] { -4, 4, 4, 1 }),
				new Vector<int>(new[] { -6, 5, -1, 1 }),
			});
			var b = new Matrix<int>(new[] {
				new Vector<int>(new[] { 8, 2, 2, 2 }),
				new Vector<int>(new[] { 3, -1, 7, 0 }),
				new Vector<int>(new[] { 7, 0, 5, 4 }),
				new Vector<int>(new[] { 6, -2, 0, 5 }),
			});
			var c = a * b;
			var d = c.Cast<int, double>() * b.Inverse();
			Assert.True(d.ApproxEq(a.Cast<int, double>(), 0.00001));
		}
	}
}
