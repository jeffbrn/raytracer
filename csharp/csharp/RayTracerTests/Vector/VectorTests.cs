using Research.RayTracer.Vector;

namespace Research.RayTracer.Tests.Vector {
	public class VectorTests {
		[Fact]
		public void Initialization() {
			var v1 = new Vector<int> { 1, 2, 3 };
			Assert.Equal(3, v1.Count);
			Assert.Equal(1, v1[0]);
			Assert.Equal(2, v1[1]);
			Assert.Equal(3, v1[2]);

			var v2 = new Vector<double>(new[] { 1.1, 1.2, 1.3, 1.4 });
			Assert.Equal(4, v2.Count);
			Assert.Equal(1.1, v2[0]);
			Assert.Equal(1.2, v2[1]);
			Assert.Equal(1.4, v2[3]);

			var v3 = new Vector<int>(2);
			Assert.Equal(2, v3.Count);
			Assert.Equal(0, v3[0]);
			Assert.Equal(0, v3[1]);
		}

		[Fact]
		public void GettersAndSetters() {
			var v = new Vector<double>(new[] { 1.1, 1.2, 1.3, 1.4 }) {
				[1] = 33.34
			};
			Assert.Equal(33.34, v[1]);
			Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = v[-1]; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { var _ = v[4]; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { v[-1] = 3.14; });
			Assert.Throws<ArgumentOutOfRangeException>(() => { v[4] = 3.14; });
		}

		[Fact]
		public void Equality() {
			var a = new Vector<double>(new[] { 1.1, 1.2, 1.3, 1.4 });
			var b = new Vector<double>(new[] { 1.1, 1.2, 1.3, 1.4 });
			Assert.Equal(a, b);
			var c = new Vector<double>(new[] { 1.0, 1, 1, 1 });
			Assert.NotEqual(a, c);
		}
	}
}
