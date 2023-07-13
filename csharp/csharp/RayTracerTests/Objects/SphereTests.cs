using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Research.RayTracer.Objects;
using Research.RayTracer.Vector;

namespace Research.RayTracer.Tests.Objects {
	public class SphereTests {

		[Fact]
		public void RayIntersect2Points() {
			var r = new Ray<double>(RayTuple<double>.Point(0.0, 0.0, -5.0), RayTuple<double>.Vector(0.0, 0.0, 1.0));
			var s = new Sphere();
			var result = s.Intersect(r);
			Assert.NotNull(result);
			Assert.Equal(4.0, result.Item1);
			Assert.Equal(6.0, result.Item2);
		}

		[Fact]
		public void RayIntersectTangent() {
			var r = new Ray<double>(RayTuple<double>.Point(0.0, 1.0, -5.0), RayTuple<double>.Vector(0.0, 0.0, 1.0));
			var s = new Sphere();
			var result = s.Intersect(r);
			Assert.NotNull(result);
			Assert.Equal(5.0, result.Item1);
			Assert.Equal(5.0, result.Item2);
		}

		[Fact]
		public void RayNotIntersect() {
			var r = new Ray<double>(RayTuple<double>.Point(0.0, 2.0, -5.0), RayTuple<double>.Vector(0.0, 0.0, 1.0));
			var s = new Sphere();
			var result = s.Intersect(r);
			Assert.Null(result);
		}

		[Fact]
		public void RayIntersectInside() {
			var r = new Ray<double>(RayTuple<double>.Point(0.0, 0.0, 0.0), RayTuple<double>.Vector(0.0, 0.0, 1.0));
			var s = new Sphere();
			var result = s.Intersect(r);
			Assert.NotNull(result);
			Assert.Equal(-1.0, result.Item1);
			Assert.Equal(1.0, result.Item2);
		}

		[Fact]
		public void RayIntersectBehind() {
			var r = new Ray<double>(RayTuple<double>.Point(0.0, 0.0, 5.0), RayTuple<double>.Vector(0.0, 0.0, 1.0));
			var s = new Sphere();
			var result = s.Intersect(r);
			Assert.NotNull(result);
			Assert.Equal(-6.0, result.Item1);
			Assert.Equal(-4.0, result.Item2);
		}

	}
}
