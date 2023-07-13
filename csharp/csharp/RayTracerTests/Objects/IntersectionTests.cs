using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Research.RayTracer.Objects;

namespace Research.RayTracer.Tests.Objects {
	public class IntersectionTests {
		[Fact]
		public void HitAllPositiveT() {
			var s = new Sphere();
			var i1 = new Intersection(1, s);
			var i2 = new Intersection(2, s);
			var xs = new Intersections(i1, i2);
			var i = xs.Hit();
			Assert.Equal(i1, i);
		}

		[Fact]
		public void HitSomeNegativeT() {
			var s = new Sphere();
			var i1 = new Intersection(-1, s);
			var i2 = new Intersection(1, s);
			var xs = new Intersections(i1, i2);
			var i = xs.Hit();
			Assert.Equal(i2, i);
		}

		[Fact]
		public void HitAllNegativeT() {
			var s = new Sphere();
			var i1 = new Intersection(-2, s);
			var i2 = new Intersection(-1, s);
			var xs = new Intersections(i1, i2);
			var i = xs.Hit();
			Assert.Null(i);
		}

		[Fact]
		public void HitLowestPositiveT() {
			var s = new Sphere();
			var i1 = new Intersection(5, s);
			var i2 = new Intersection(7, s);
			var i3 = new Intersection(-3, s);
			var i4 = new Intersection(2, s);
			var xs = new Intersections(i1, i2, i3, i4);
			var i = xs.Hit();
			Assert.Equal(i4, i);
		}
	}
}
