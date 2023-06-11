using Research.RayTracer.Vector;

namespace Research.RayTracer.Tests.Vector {
	public class Tuples {
		[Fact]
		public void IsAPoint() {
			var a = RayTuple<float>.Point(4.3f, -4.2f, 3.1f);
			Assert.Equal(4.3f, a.X);
			Assert.Equal(-4.2f, a.Y);
			Assert.Equal(3.1f, a.Z);
			Assert.Equal(1.0f, a.W);
			Assert.True(a.IsPoint);
			Assert.False(a.IsVector);
		}

		[Fact]
		public void IsAVector() {
			var a = RayTuple<float>.Vector(4.3f, -4.2f, 3.1f);
			Assert.Equal(4.3f, a.X);
			Assert.Equal(-4.2f, a.Y);
			Assert.Equal(3.1f, a.Z);
			Assert.Equal(0.0f, a.W);
			Assert.True(a.IsVector);
			Assert.False(a.IsPoint);
		}

		[Fact]
		public void Add() {
			var a1 = RayTuple<int>.Point(3, -2, 5);
			var a2 = RayTuple<int>.Vector(-2, 3, 1);
			var expected = RayTuple<int>.Point(1, 1, 6);
			Assert.Equal(expected, a1 + a2);
		}

		[Fact]
		public void InvalidAdd() {
			var a1 = RayTuple<int>.Point(3, -2, 5);
			var a2 = RayTuple<int>.Point(-2, 3, 1);
			Assert.Throws<ArgumentOutOfRangeException>(() => a1 + a2);
		}

		[Fact]
		public void Subtract() {
			var a1 = RayTuple<int>.Point(3, 2, 1);
			var a2 = RayTuple<int>.Point(5, 6, 7);
			var expected1 = RayTuple<int>.Vector(-2, -4, -6);
			Assert.Equal(expected1, a1 - a2);

			var v2 = RayTuple<int>.Vector(5, 6, 7);
			var expected2 = RayTuple<int>.Point(-2, -4, -6);
			Assert.Equal(expected2, a1 - v2);
		}

		[Fact]
		public void InvalidSubtract() {
			var a1 = RayTuple<int>.Vector(3, -2, 5);
			var a2 = RayTuple<int>.Point(-2, 3, 1);
			Assert.Throws<ArgumentOutOfRangeException>(() => a1 - a2);
		}

		[Fact]
		public void Negate() {
			var a = RayTuple<int>.Vector(1, -2, 3);
			var expected = RayTuple<int>.Vector(-1, 2, -3);
			Assert.Equal(expected, -a);
		}

		[Fact]
		public void ScalarMult() {
			var a = RayTuple<float>.Vector(1.0, -2.0, 3.0);
			Assert.Equal(RayTuple<float>.Vector(3.5, -7.0, 10.5), a * 3.5);
			var b = RayTuple<int>.Point(2, 4, 6);
			Assert.Equal(RayTuple<int>.Point(1, 2, 3), b / 2);
		}

		[Fact]
		public void Magnitude() {
			Assert.Equal(1.0, RayTuple<int>.Vector(0, 1, 0).Magnitude());
			Assert.Equal(1.0, RayTuple<float>.Vector(0.0, 0.0, 1.0).Magnitude());
			var expected = Math.Sqrt(14.0);
			Assert.Equal(expected, RayTuple<int>.Vector(1, 2, 3).Magnitude());
			Assert.Equal(expected, RayTuple<int>.Vector(-1, -2, -3).Magnitude());
			Assert.Throws<InvalidOperationException>(() => RayTuple<int>.Point(1, 2, 3).Magnitude());
		}

		[Fact]
		public void Normalize() {
			var v1 = RayTuple<int>.Vector(4, 0, 0);
			Assert.Equal(RayTuple<double>.Vector(1.0, 0, 0), v1.Normalize());
			var mag = Math.Sqrt(14.0);
			var expected = RayTuple<double>.Vector(1.0 / mag, 2.0 / mag, 3.0 / mag);
			Assert.Equal(expected, RayTuple<float>.Vector(1.0, 2.0, 3.0).Normalize());
			Assert.Equal(1.0, RayTuple<float>.Vector(1.0, 2.0, 3.0).Normalize().Magnitude());
			Assert.Throws<InvalidOperationException>(() => RayTuple<int>.Point(1, 2, 3).Normalize());
		}

		[Fact]
		public void DotProduct() {
			var a = RayTuple<int>.Vector(1, 2, 3);
			var b = RayTuple<int>.Vector(2, 3, 4);
			Assert.Equal(20, a.DotProd(b));
			Assert.Throws<InvalidOperationException>(() => RayTuple<int>.Point(1, 2, 3).DotProd(a));
			Assert.Throws<InvalidOperationException>(() => a.DotProd(RayTuple<int>.Point(1, 2, 3)));
			Assert.Throws<InvalidOperationException>(() => RayTuple<int>.Point(1, 2, 3).DotProd(RayTuple<int>.Point(1, 2, 3)));
		}

		[Fact]
		public void CrossProduct() {
			var a = RayTuple<int>.Vector(1, 2, 3);
			var b = RayTuple<int>.Vector(2, 3, 4);
			Assert.Equal(RayTuple<int>.Vector(-1, 2, -1), a * b);
			Assert.Equal(RayTuple<int>.Vector(1, -2, 1), b * a);
			Assert.Throws<InvalidOperationException>(() => RayTuple<int>.Point(1, 2, 3) * a);
			Assert.Throws<InvalidOperationException>(() => a * RayTuple<int>.Point(1, 2, 3));
			Assert.Throws<InvalidOperationException>(() => RayTuple<int>.Point(1, 2, 3) * RayTuple<int>.Point(1, 2, 3));
		}
	}
}
