using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Research.RayTracer.Vector;

namespace Research.RayTracer.Tests.Vector {
	public class TransformTests {
		private readonly double PI = Math.PI;
		private readonly double Root2 = Math.Sqrt(2);

		[Fact]
		public void TranslatePoint() {
			var transform = Matrix<double>.Identity(4).Translate(RayTuple<int>.Point(5, -3, 2));
			var p = RayTuple<double>.Point(- 3.0, 4.0, 5.0);
			var result = transform * p;
			Assert.Equal(RayTuple<double>.Point(2.0, 1.0, 7.0), result);

			var inverse = transform.Inverse();
			var p2 = RayTuple<double>.Point(-3.0, 4.0, 5.0);
			var result2 = inverse * p2;
			Assert.Equal(RayTuple<double>.Point(-8.0, 7.0, 3.0), result2);
		}

		[Fact]
		public void TranslateVector() {
			var v = RayTuple<double>.Vector(-3.0, 4.0, 5.0);
			var t = Matrix<double>.Identity(4).Translate(RayTuple<double>.Point(5.0, -3.0, 2.0));
			var result = t * v;
			Assert.Equal(v, result);
		}

		[Fact]
		public void Scale() {
			var p = RayTuple<double>.Point(2.0, 3.0, 4.0);
			var transform = Matrix<double>.Identity(4).Scale(RayTuple<double>.Vector(-1.0, 1.0, 1.0));
			var result = transform * p;
			Assert.Equal(RayTuple<double>.Point(-2.0, 3.0, 4.0), result);
		}

		[Fact]
		public void RotationX() {
			var p = RayTuple<double>.Point(0.0, 1.0, 0.0);
			var half_qtr = Matrix<double>.Identity(4).RotateX(PI/4);
			var full_qtr = Matrix<double>.Identity(4).RotateX(PI/2);
			Assert.True(RayTuple<double>.Point(0.0, Root2/2, Root2/2).ApproxEq(half_qtr * p, 0.000001));
			Assert.True(RayTuple<double>.Point(0.0, 0.0, 1.0).ApproxEq(full_qtr * p, 0.000001));
		}

		[Fact]
		public void RotationY() {
			var p = RayTuple<double>.Point(0.0, 0.0, 1.0);
			var half_qtr = Matrix<double>.Identity(4).RotateY(PI / 4);
			var full_qtr = Matrix<double>.Identity(4).RotateY(PI / 2);
			Assert.True(RayTuple<double>.Point(Root2 / 2, 0.0, Root2 / 2).ApproxEq(half_qtr * p, 0.000001));
			Assert.True(RayTuple<double>.Point(1.0, 0.0, 0.0).ApproxEq(full_qtr * p, 0.000001));
		}

		[Fact]
		public void RotationZ() {
			var p = RayTuple<double>.Point(0.0, 1.0, 0.0);
			var half_qtr = Matrix<double>.Identity(4).RotateZ(PI / 4);
			var full_qtr = Matrix<double>.Identity(4).RotateZ(PI / 2);
			Assert.True(RayTuple<double>.Point(-Root2 / 2, Root2 / 2, 0.0).ApproxEq(half_qtr * p, 0.000001));
			Assert.True(RayTuple<double>.Point(-1.0, 0.0, 0.0).ApproxEq(full_qtr * p, 0.000001));
		}

		[Fact]
		public void Shearing() {
			var p = RayTuple<int>.Point(2, 3, 4);
			var result = p.Shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
			Assert.Equal(RayTuple<double>.Point(5.0, 3.0, 4.0), result);

			p = RayTuple<int>.Point(2, 3, 4);
			result = p.Shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
			Assert.Equal(RayTuple<double>.Point(6.0, 3.0, 4.0), result);

			p = RayTuple<int>.Point(2, 3, 4);
			result = p.Shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
			Assert.Equal(RayTuple<double>.Point(2.0, 5.0, 4.0), result);

			p = RayTuple<int>.Point(2, 3, 4);
			result = p.Shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
			Assert.Equal(RayTuple<double>.Point(2.0,7.0, 4.0), result);

			p = RayTuple<int>.Point(2, 3, 4);
			result = p.Shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
			Assert.Equal(RayTuple<double>.Point(2.0, 3.0, 6.0), result);

			p = RayTuple<int>.Point(2, 3, 4);
			result = p.Shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
			Assert.Equal(RayTuple<double>.Point(2.0, 3.0, 7.0), result);
		}

		[Fact]
		public void IndividualTransforms() {
			var p = RayTuple<double>.Point(1.0, 0.0, 1.0);
			var A = Matrix<double>.Identity(4).RotateX(PI / 2);
			var B = Matrix<double>.Identity(4).Scale(RayTuple<double>.Point(5.0, 5.0, 5.0));
			var C = Matrix<double>.Identity(4).Translate(RayTuple<double>.Point(10.0, 5.0, 7.0));
			var p2 = A * p;
			Assert.True(RayTuple<double>.Point(1.0, -1.0, 0.0).ApproxEq(p2, 0.000001));
			var p3 = B * p2;
			Assert.True(RayTuple<double>.Point(5.0, -5.0, 0.0).ApproxEq(p3, 0.000001));
			var p4 = C * p3;
			Assert.True(RayTuple<double>.Point(15.0, 0.0, 7.0).ApproxEq(p4, 0.000001));
		}

		[Fact]
		public void ChainedTransforms() {
			var p = RayTuple<double>.Point(1.0, 0.0, 1.0);
			var t = Matrix<double>.Identity(4).Translate(RayTuple<int>.Point(10, 5, 7))
				.Scale(RayTuple<double>.Point(5.0, 5.0, 5.0))
				.RotateX(PI / 2);
			Assert.True(RayTuple<double>.Point(15.0, 0.0, 7.0).ApproxEq(t * p, 0.000001));
		}

	}
}
